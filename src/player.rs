use std::{collections::HashMap, f32::consts::PI};

use crate::{entities::{health::{DamageType, Health}, monster::Monster, Entity}, game_dimension::{get_chunks_in_range, GameDimension}, item::{self, Item}, physics::{Movement, Position, Velocity}, utils::is_within_angle_range};


pub struct Player {
    pub movement : Movement,
    pub walk_dir: (i8,i8),
    pub facing : f32,
    //approach allows for there to be infinite items in the game held by the player
    pub inventory: HashMap<Item, u32>,
    pub hotbar: [Option<Item>; 9],
    pub main_hand: Option<Item>,
    pub off_hand: Option<Item>,
    pub health: f32,
    pub attack_cooldown : (u128, u128),
    pub level : u64,
    pub xp : u64,
    pub xp_to_level_up : u64,
    pub stats : PlayerStats,
    pub invested_skill_points : PlayerSkillPointsInvested,
}

#[derive(Clone)]
pub struct PlayerStats {
    pub max_health: f32,
    pub attack_damage: f32,
    pub speed: f32,
    pub resistance: f32, //slight damage absorbing, not times but minus based
    pub shielding: f32, //a bar which recovers that acts as another health meter, 
    pub health_regeneration: f32,
    pub far_sight: f32,
}

#[derive(Clone)]
pub struct PlayerSkillPointsInvested {
    pub health_points_invested : u64,
    pub damage_points_invested : u64,
    pub speed_points_invested : u64,
    pub resistance_points_invested : u64,
    pub shielding_points_invested : u64,
    pub health_regeneration_points_invested:  u64,
    pub far_sight_points_invested:  u64,
}
impl PlayerSkillPointsInvested {
    pub fn get_points_used(&self) -> u64 {
        let points_used = self.health_points_invested + self.damage_points_invested + self.speed_points_invested + self.resistance_points_invested + self.shielding_points_invested;
        return points_used;
    }
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
            walk_dir: (0,0),
            facing: 0.0,
            inventory: HashMap::new(),
            hotbar: [const { None }; 9],
            main_hand: None,
            off_hand: None,
            health: 100.0,
            level: 1,
            xp: 0,
            xp_to_level_up: 100,
            attack_cooldown: (0,0),
            stats: PlayerStats {
                max_health: 100.0,
                attack_damage: 20.0,
                speed: 0.075,
                resistance: 0.0,
                shielding: 0.0,
                health_regeneration: 1.2,
                far_sight: 1.0,
            },
            invested_skill_points: PlayerSkillPointsInvested { 
                health_points_invested: 0, 
                damage_points_invested: 0, 
                speed_points_invested: 0, 
                resistance_points_invested: 0, 
                shielding_points_invested: 0,
                health_regeneration_points_invested: 0, 
                far_sight_points_invested: 0,
            },
        }
    }

    pub fn gain_xp(&mut self,amount: u64) {
        self.xp += amount;
        if self.xp >= self.xp_to_level_up {
            self.level += 1;
            self.xp = self.xp - self.xp_to_level_up;
            self.xp_to_level_up = (100.0 * (self.level as f32).powf(1.2) ) as u64;
            //stat boost the level up.

            self.stats = Player::calculate_stats(self.level, &self.invested_skill_points);
        }
    }

    pub fn calculate_stats(level : u64, invested_points: &PlayerSkillPointsInvested) -> PlayerStats {
        let max_health = 100.0;
        let max_health = max_health + (max_health * ((level - 1) as f32).powf(1.3) / 5.0);
        let max_health = max_health + (max_health * (invested_points.health_points_invested as f32).powf(1.5) / 5.0);

        let attack_damage = 20.0;
        let attack_damage = attack_damage + (attack_damage * ((level - 1) as f32).powf(1.3) / 5.0);
        let attack_damage = attack_damage + (attack_damage * (invested_points.damage_points_invested as f32).powf(1.5) / 5.0);

        let speed = 0.075;
        let speed = speed + (speed * ((level - 1) as f32).powf(1.3) / 5.0);
        let speed = speed + (speed * (invested_points.speed_points_invested as f32).powf(1.5) / 5.0);

        let resistance = 1.0;
        let resistance = resistance + (resistance * ((level - 1) as f32).powf(1.3) / 5.0);
        let resistance = resistance + (resistance * (invested_points.resistance_points_invested as f32).powf(1.5) / 5.0);


        let shielding = 100.0;
        let shielding = shielding + (shielding * (level as f32).powf(1.12) / 5.0);
        let shielding = shielding + (shielding * ((invested_points.shielding_points_invested + 1) as f32).powf(1.12) / 5.0);
        let shielding = shielding - 100.0;

        let health_regeneration = 1.2;
        let health_regeneration = health_regeneration + (health_regeneration * ((level - 1) as f32).powf(1.3) / 5.0);
        let health_regeneration = health_regeneration + (health_regeneration * (invested_points.health_regeneration_points_invested as f32).powf(1.5) / 5.0);

        let far_sight = 1.0;
        let far_sight = far_sight + (far_sight * ((level - 1) as f32).powf(1.3) / 5.0);
        let far_sight = far_sight + (far_sight * (invested_points.far_sight_points_invested as f32).powf(1.5) / 5.0);

        PlayerStats {
            max_health: max_health,
            attack_damage: attack_damage,
            speed: speed,
            resistance: resistance,
            shielding: shielding,
            health_regeneration : health_regeneration,
            far_sight: far_sight
        }
    }

    pub fn calculate_skill_points_count(level : u64) -> u64 {
        return level;
    }

    pub fn right_hand_attack(&mut self, game_dimension : &mut GameDimension) {
        let player_position = self.movement.position.clone();
        let attack_dir = self.facing;
        let player_hand = self.main_hand.clone();
    
    
        let (weapon_attack_damage, attack_reach, attack_angle, attack_cooldown) = match player_hand {
            Some(item_hand) => {
                let weapon_attack_damage = item_hand.item.info().attributes.damage;
                let attack_distance = item_hand.item.info().attributes.attack_distance;
                let swing_distance = item_hand.item.info().attributes.swing_distance;
                let attack_cooldown = item_hand.item.info().attributes.attack_cooldown;
    
                let weapon_attack_damage = match weapon_attack_damage {
                    Some(weapon_attack_damage) => {
                        weapon_attack_damage
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
                let attack_cooldown = match attack_cooldown {
                    Some(attack_cooldown) => {
                        attack_cooldown
                    }
                    None => 0,
                };
    
                (weapon_attack_damage, attack_distance, swing_distance, attack_cooldown)
            },
            None => {
                (0.0, 0.0, 0.0, 0)
            },
        };

        if self.attack_cooldown.1 > game_dimension.tick_number {
            return;
        }
    
        if weapon_attack_damage != 0.0 || attack_reach != 0.0 || attack_angle != 0.0 {
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
                    damage: weapon_attack_damage + self.stats.attack_damage, //add weapon and player damage
                },true);
                println!("{} {} {}",weapon_attack_damage + self.stats.attack_damage, weapon_attack_damage,self.stats.attack_damage)
            }

            self.attack_cooldown = (game_dimension.tick_number, game_dimension.tick_number + attack_cooldown as u128)

        }
    }

    pub fn tick(&mut self) {
        self.regen();
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

