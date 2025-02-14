use std::{collections::HashMap, ops::Deref};

use rand::{thread_rng, Rng};
use uuid::Uuid;
use crate::{game_dimension::{self, GameDimension, MagicElements}, physics::{Movement, Position, Velocity}, player::{self, Player}};
use super::DamageType;
use dyn_clone::DynClone;

enum MonsterAttack {
    Melee { reinforced_magic : Option<MagicElements>},
    Ranged { reinforced_magic : Option<MagicElements>},
    Magic { reinforced_magic : Option<MagicElements>},
    AreaOfEffect { reinforced_magic : Option<MagicElements>},
    Summon { reinforced_magic : Option<MagicElements>},
    Special { reinforced_magic : Option<MagicElements>},
}

#[derive(Clone)]
pub enum MonsterAiState {
    Attacking,
    Defend,
    Flee,
    Idle,
    Wonder {wonder_to : Position},
}

#[derive(Clone)]
pub struct Monster {
    pub id : Uuid,
    pub health : f32,
    pub max_health : f32,
    pub movement : Movement,
    pub level : u32,
    pub mob_type: MonsterType,
    pub speed : f32,
    pub ai : Box<dyn MonsterAi>,
}

dyn_clone::clone_trait_object!(MonsterAi);


pub trait MonsterAi: DynClone {
    fn update_state(&mut self, monster: &Monster, player : &mut Player); 
    fn use_state(&self, monster: Monster, player : &mut Player) -> Monster;
}

#[derive(Clone)]
struct GhostAi {
    state : MonsterAiState,
} //you can add extra ai spastic data here like fire blast for dragons.

impl MonsterAi for GhostAi {
    fn update_state(&mut self, monster: &Monster, player : &mut Player) {

        if monster.movement.position.distance_to(&player.movement.position) <= 10.0 {
            self.state = MonsterAiState::Attacking;
            return;
        }

        if monster.movement.position.distance_to(&player.movement.position) >= 15.0 {
            match &self.state {
                MonsterAiState::Wonder { wonder_to } => {
                    if &monster.movement.position.distance_to(&wonder_to) <= &1.0 {
                        self.state = MonsterAiState::Idle;
                    }
                },
                MonsterAiState::Attacking => {}
                _ => {
                    let offset_x : f32 = rand::thread_rng().gen_range(-10.0..10.0);
                    let offset_y : f32 = rand::thread_rng().gen_range(-10.0..10.0);
                    self.state = MonsterAiState::Wonder {wonder_to : Position {
                        x: monster.movement.position.x + offset_x,
                        y: monster.movement.position.y + offset_y,
                    }};
                }
            }
            return;
        }

    }    
    
    fn use_state(&self, monster: Monster, player : &mut Player) -> Monster {
        let mut monster = monster;

        match &self.state {
            MonsterAiState::Wonder { wonder_to } => {
                monster.movement.apply_force_towards(wonder_to, monster.speed / 2.0);
            },
            MonsterAiState::Attacking => {
                if monster.movement.position.distance_to(&player.movement.position) > 1.0 {
                    monster.movement.apply_force_towards(&player.movement.position, monster.speed);
                }
            }
            _ => (),
        }
        

        monster
    }
}

impl Monster {
    pub fn tick(&mut self, player : &mut Player) {
        //current way this is coded monsters can't change data for other monsters

        self.ai.update_state(&self.clone(), player);
        *self = self.ai.use_state(self.clone(), player);
    }


    pub fn deal_damage(&mut self,damage_type:DamageType) -> () {
        self.health = &self.health.clone() - damage_type.damage;
    }

    pub fn from_id(monster_hashmap : &HashMap<Uuid, Monster>, id : Uuid) -> Option<&Monster> {
        return monster_hashmap.get(&id)
    }
    pub fn from_id_mut(monster_hashmap : &mut HashMap<Uuid, Monster>, id : Uuid) -> Option<&mut Monster> {
        return monster_hashmap.get_mut(&id)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MonsterType {
    Ghost
}

impl MonsterType{
    pub fn get_mob_name(mob : MonsterType) -> String {

        match mob {
            MonsterType::Ghost => "Ghost".to_string(),
        }
        
    }
}

pub fn create_monster(monster: MonsterType) -> Monster {
    println!("spawn monster");
    let mob_id = Uuid::new_v4();
    return match monster {
        MonsterType::Ghost => Monster {
            health: 15.0,
            max_health: 15.0,
            movement: Movement {
                velocity: Velocity {
                    x: 0.0,
                    y: 0.0,
                },
                position: Position { 
                    x: 0.0, 
                    y: 0.0 
                },
                drag: 0.05,
                mass: 20.0,
            },
            level: 1,
            mob_type: MonsterType::Ghost,
            speed: 0.05,
            ai: Box::new(GhostAi {state : MonsterAiState::Idle}),
            id: mob_id,
        },
    };

}