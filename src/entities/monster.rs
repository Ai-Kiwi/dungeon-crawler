use std::{collections::HashMap, ops::Deref};

use rand::{thread_rng, Rng};
use uuid::Uuid;
use crate::{game_dimension::{self, GameDimension, MagicElements}, physics::{Movement, Position, Velocity}, player::{self, Player}};
use dyn_clone::DynClone;

use super::health::{DamageType, Health};

enum MonsterAttack {
    Melee { reinforced_magic : Option<MagicElements>},
    Ranged { reinforced_magic : Option<MagicElements>},
    Magic { reinforced_magic : Option<MagicElements>},
    AreaOfEffect { reinforced_magic : Option<MagicElements>},
    Summon { reinforced_magic : Option<MagicElements>},
    Special { reinforced_magic : Option<MagicElements>},
}

#[derive(Clone, Default)]
pub enum MonsterAiState {
    Attacking,
    Defend,
    Flee,
    #[default]
    Idle,
    Wonder {wonder_to : Position},
}

#[derive(Clone)]
pub struct Monster {
    pub id : Uuid,
    pub health : f32,
    pub max_health : f32,
    pub damage : f32,
    pub movement : Movement,
    pub level : u32,
    pub mob_type: MonsterType,
    pub speed : f32,
    pub ai : Box<dyn MonsterAi>,
    pub player_damaged : bool,
}

dyn_clone::clone_trait_object!(MonsterAi);
#[derive(Clone, Default)]
struct AttackCooldown {
    expire_time : u128,
    length : u128,
}
impl AttackCooldown {
    pub fn reset(&mut self, tick_number : u128) {
        self.expire_time = tick_number + self.length;
    }
    pub fn has_expired(&self, tick_number : u128) -> bool {
        self.expire_time < tick_number
    }
    pub fn new(length : u128) -> Self {
        Self { expire_time: 0, length: length  }
    }
}


pub trait MonsterAi: DynClone {
    fn update_state(&mut self, monster: &Monster, player : &mut Player); 
    fn use_state(&mut self, monster: Monster, player : &mut Player, tick_number : u128) -> Monster;
}

#[derive(Clone, Default)]
struct GhostAi {
    state : MonsterAiState,
    basic_attack_cooldown : AttackCooldown,
} //you can add extra ai spastic data here like fire blast for dragons.

impl MonsterAi for GhostAi {
    fn update_state(&mut self, monster: &Monster, player : &mut Player) {


        if monster.movement.position.distance_to(&player.movement.position) <= 10.0 {
            self.state = MonsterAiState::Attacking;
            return;
        }
        
        match &self.state {
            MonsterAiState::Wonder { wonder_to } => {
                if &monster.movement.position.distance_to(&wonder_to) <= &1.0 {
                    self.state = MonsterAiState::Idle;
                }
            },
            MonsterAiState::Attacking => {

            }
            _ => {
                if monster.movement.position.distance_to(&player.movement.position) >= 15.0 {
                    let offset_x : f32 = rand::thread_rng().gen_range(-10.0..10.0);
                    let offset_y : f32 = rand::thread_rng().gen_range(-10.0..10.0);
                    self.state = MonsterAiState::Wonder {wonder_to : Position {
                        x: monster.movement.position.x + offset_x,
                        y: monster.movement.position.y + offset_y,
                    }};
                }
            }
        }
            return;
        

    }    
    
    fn use_state(&mut self, monster: Monster, player : &mut Player, tick_number : u128) -> Monster {
        let mut monster = monster;

        match &self.state {
            MonsterAiState::Wonder { wonder_to } => {
                monster.movement.apply_force_towards(wonder_to, monster.speed / 2.0);
            },
            MonsterAiState::Attacking => {
                if monster.movement.position.distance_to(&player.movement.position) > 1.0 {
                    monster.movement.apply_force_towards(&player.movement.position, monster.speed);
                }

                if monster.movement.position.distance_to(&player.movement.position) < 1.5 {
                        //in range to attack
                        if self.basic_attack_cooldown.has_expired(tick_number) {
                            self.basic_attack_cooldown.reset(tick_number);
                            player.deal_damage(DamageType{
                                damage: monster.damage,
                            },false);
                        }
                }
            }
            _ => (),
        }
        

        monster
    }
}

impl Monster {
    pub fn tick(&mut self, player : &mut Player, tick_number : u128) {
        //current way this is coded monsters can't change data for other monsters

        self.ai.update_state(&self.clone(), player);
        let new_monster = self.ai.use_state(self.clone(), player, tick_number);
        let old_ai = self.ai.clone();
        *self = new_monster;
        self.ai = old_ai;
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

pub fn create_monster(monster: MonsterType, level : u64) -> Monster {
    let mob_id = Uuid::new_v4();
    //the rates here increase faster then for player has they don't have skill points. This will also later be different rates of increase for different mobs as different mobs have different stats
    //general idea is player level == mob level in terms of skill
    let damage = 20.0 * (level as f32).powf(1.2);
    let health = 100.0 * (level as f32).powf(1.2);
    let speed = 0.075;
    return match monster {
        MonsterType::Ghost => Monster {
            health: health,
            max_health: health,
            damage: damage,
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
            speed: 0.075,
            ai: Box::new(GhostAi {state : MonsterAiState::Idle, basic_attack_cooldown: AttackCooldown::new(30) } ),
            id: mob_id,
            player_damaged : false,
        },
    };

}