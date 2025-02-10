use rand::Rng;
use crate::{game_dimension::GameDimension, physics::{Movement, Position, Velocity}};
use super::DamageType;

#[derive(Clone)]
pub enum Goal {
    //AttackPlayer,
    WonderTo {
        location : Position,
    },
    Idle {
        auto_expire_time : u32
    }
}

#[derive(Clone)]
pub struct Monster {
    pub health : f32,
    pub max_health : f32,
    pub movement : Movement,
    pub level : u32,
    pub current_goal : Goal,
    pub mob_type: MonsterType,
    pub tick_age : u32,
    pub speed : f32,
}

impl Monster {
    pub fn tick(&mut self, game_dimension: &mut GameDimension) {
        //current way this is coded monsters can't change data for other monsters

        let _ = game_dimension;
        let mut rng = rand::thread_rng();

        match &self.current_goal {
            Goal::WonderTo { location } => {
                if location.x > self.movement.position.x {
                    self.movement.velocity.x += 1.0 * self.speed * 0.5;
                }else if location.x < self.movement.position.x {
                    self.movement.velocity.x += -1.0 * self.speed * 0.5;
                }

                if location.y > self.movement.position.y {
                    self.movement.velocity.y += 1.0 * self.speed * 0.5;
                }else if location.y < self.movement.position.y {
                    self.movement.velocity.y += -1.0 * self.speed * 0.5;
                }

                let wait_time: f32 = rng.gen();
                if (location.y - self.movement.position.y).abs() + (location.x - self.movement.position.x).abs() < 1.0 {
                    self.current_goal = Goal::Idle { auto_expire_time: self.tick_age + 20 + (wait_time * 20.0).floor() as u32 }
                }
            },
            Goal::Idle { auto_expire_time } => {
                
                if self.tick_age > *auto_expire_time {
                    let random_x_offset : f32 = rng.gen();
                    let random_y_offset : f32 = rng.gen();
    
                    self.current_goal = Goal::WonderTo { location: Position {
                        x: self.movement.position.x + ((random_x_offset - 0.5) * 32.0 ),
                        y: self.movement.position.y + ((random_y_offset - 0.5) * 32.0 ),
                    } }
                }

                
            },
        }

        self.tick_age += 1;
    }


    pub fn deal_damage(&mut self,damage_type:DamageType) -> () {
        self.health = &self.health.clone() - damage_type.damage;
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
            current_goal: Goal::Idle { auto_expire_time: 0 },
            mob_type: MonsterType::Ghost,
            tick_age: 0,
            speed: 0.05,
        },
    };

}