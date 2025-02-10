use std::{collections::HashMap, f32::consts::PI};

use crate::{entities::{DamageType, Entity}, game_dimension::{get_chunks_in_range, GameDimension}, item::{self, Item}, physics::{Movement, Position, Velocity}, utils::is_within_angle_range};


pub struct Player {
    pub movement : Movement,
    pub speed: f32,
    pub walk_dir: (i8,i8),
    pub interact_direction : f32,
    //approach allows for there to be infinite items in the game held by the player
    pub inventory: HashMap<Item, u32>,
    pub hotbar: [Option<Item>; 9],
    pub main_hand: Option<Item>,
    pub off_hand: Option<Item>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            movement: Movement {
                velocity: Velocity{
                    x: 0.0,
                    y: 0.0,
                },
                position: Position{
                    x: 0.0,
                    y: 0.0,
                },
                drag: 0.1,
                mass: 15.0,
            },
            speed: 0.075,
            walk_dir: (0,0),
            interact_direction: 0.0,
            inventory: HashMap::new(),
            hotbar: [const { None }; 9],
            main_hand: None,
            off_hand: None,
        }
    }

    pub fn right_hand_attack(&mut self, game_dimension : &mut GameDimension) {
        let player_position = self.movement.position.clone();
        let attack_dir = self.interact_direction;
        let player_hand = self.main_hand.clone();
    
    
        let (attack_damage, attack_reach, attack_angle) = match player_hand {
            Some(item_hand) => {
                let damage = item_hand.item.info().attributes.damage;
                let attack_distance = item_hand.item.info().attributes.attack_distance;
                let swing_distance = item_hand.item.info().attributes.swing_distance;
    
                let damage = match damage {
                    Some(damage) => {
                        damage
                    }
                    None => 0.0,   
                };
                let attack_distance: f32 = match attack_distance {
                    Some(attack_distance) => {
                        attack_distance 
                    }
                    None => 0.0,   
                };
                let swing_distance = match swing_distance {
                    Some(swing_distance) => {
                        swing_distance
                    }
                    None => 0.0,   
                };
    
                (damage, attack_distance, swing_distance)
            },
            None => {
                (0.0, 0.0, 0.0)
            },
        };
    
        if attack_damage != 0.0 || attack_reach != 0.0 || attack_angle != 0.0 {
            //values valid so its attacking with hand
            let chunks = get_chunks_in_range(&player_position, attack_reach);
    
            for chunk in chunks.iter() {
                //let chunks_lock = game_dimension_lock.chunks.read().unwrap();
                match game_dimension.chunks.get_mut(chunk) {
                    Some(chunk_data) => {
                        for entity in chunk_data.monsters.iter_mut() { 
                            if entity.movement.position.distance_to(&player_position) <= attack_reach {
                                let entity_position = entity.movement.position.clone();
                                let dx = entity_position.x - player_position.x;
                                let dy = entity_position.y - player_position.y;
                                let angle_rad = dy.atan2(dx);
                                let mut angle_deg = angle_rad * 180.0 / PI;
                                if angle_deg < 0.0 {
                                    angle_deg += 360.0; // adjust negative angles to positive
                                }
        
        
                                if is_within_angle_range(angle_deg, attack_dir, attack_angle) {
                                    entity.deal_damage(DamageType{
                                        damage: attack_damage,
                                    });
                                }
                            }
                        }
    
    
                    },
                    None => (),
                }
            }
        }
    }

    pub fn pickup_items(&mut self, game_dimension : &mut GameDimension) {
        static PICKUP_RANGE: f32 = 1.0;

        let chunks: Vec<(i32, i32)> = get_chunks_in_range(&self.movement.position, PICKUP_RANGE);

        for chunk in chunks {
            match game_dimension.chunks.get_mut(&chunk) {
                Some(chunk_data) => {
                    chunk_data.dropped_items.retain(|x| {
                        if x.position.distance_to(&self.movement.position) <= PICKUP_RANGE {
                            //in range
                            self.give_item(&x.item,&x.count);


                            false
                        }else{
                            true
                        }
                    });
                },
                None => (),
            }
            

        }



    }

    pub fn give_item(&mut self, item: &Item, count: &u32) {
        match self.inventory.get(item) {
            Some(current_count) => self.inventory.insert(item.clone(), current_count.clone() + count),
            None => self.inventory.insert(item.clone(), *count),
        }; 
    }
}

