use std::{collections::HashMap, f32::consts::PI};

use crate::{entities::{monster::Monster, DamageType, Entity}, game_dimension::{get_chunks_in_range, GameDimension}, item::{self, Item}, physics::{Movement, Position, Velocity}, utils::is_within_angle_range};


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
    pub health: f32,
    pub max_health: f32,
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
            health: 100.0,
            max_health: 100.0,
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
            let mut mobs_attacked = Vec::new();
            for chunk in chunks.iter() {
                //let chunks_lock = game_dimension_lock.chunks.read().unwrap();
                match game_dimension.chunks.get(chunk) {
                    Some(chunk_data) => {
                        for entity in chunk_data.monsters.iter() { 
                            let mob_data = Monster::from_id(&game_dimension.monsters, *entity).unwrap();
                            if mob_data.movement.position.distance_to(&player_position) <= attack_reach {
                                
                                let entity_position = mob_data.movement.position.clone();
                                let dx = entity_position.x - player_position.x;
                                let dy = entity_position.y - player_position.y;
                                let mut angle_deg = -dy.atan2(dx).to_degrees();
                                if angle_deg < 0.0 {
                                    angle_deg += 360.0; // adjust negative angles to positive
                                }
        
                                if is_within_angle_range(angle_deg, attack_dir, attack_angle) {
                                    mobs_attacked.push(*entity);
                                }
                            }
                        }
    
    
                    },
                    None => (),
                }
            }

            for monster in mobs_attacked {
                Monster::from_id_mut(&mut game_dimension.monsters, monster).unwrap().deal_damage(DamageType{
                    damage: attack_damage,
                });
            }



        }
    }

    pub fn pickup_items(&mut self, game_dimension : &mut GameDimension) {
        static PICKUP_RANGE: f32 = 1.0;

        let chunks: Vec<(i32, i32)> = get_chunks_in_range(&self.movement.position, PICKUP_RANGE);
        let mut dropped_items_testing = Vec::new();

        for chunk in chunks {
            match game_dimension.chunks.get(&chunk) {
                Some(chunk_data) => {
                    for item in &chunk_data.dropped_items {
                        dropped_items_testing.push(*item);
                    }
                },
                None => (),
            }
        }

        for item_id in dropped_items_testing {
            let item_data = game_dimension.dropped_items.get(&item_id).unwrap().clone();

            if item_data.position.distance_to(&self.movement.position) <= PICKUP_RANGE {
                //in range
                self.give_item(&item_data.item,&item_data.count);
                game_dimension.dropped_items.remove(&item_id);
                let chunk_pos = GameDimension::position_to_chunk( &item_data.position  );
                game_dimension.chunks.get_mut(&chunk_pos).unwrap().dropped_items.retain(|&x| x != item_id);
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

