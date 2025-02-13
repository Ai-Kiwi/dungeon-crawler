
use noise::core::simplex::simplex_2d;
use rand::Rng;
use noise::Vector2;
use uuid::Uuid;


use crate::{entities::{dropped_item::{self, DroppedItem}, environmental_object::{self, EnvironmentalObject}}, game_dimension::{Biome, Block, Chunk, GameDimension}, item::{Item, PremadeItem}, physics::Position, player::Player};

impl GameDimension {
    pub fn tick_chunk_loading(&mut self, player: &Player) {
        let chunks_to_create= {

            let player_pos: Position = player.movement.position.clone();
            let mut chunks_to_create: Vec<(i32,i32)> = Vec::new();

            for chunk_x in (((player_pos.x / 16.0) - 16.0) as i32)..(((player_pos.x / 16.0) + 16.0) as i32){
                for chunk_y in (((player_pos.y / 16.0) - 16.0) as i32 )..(((player_pos.y / 16.0) + 16.0) as i32){
                    if self.chunks.get(&(chunk_x,chunk_y)).is_some() == false {
                        chunks_to_create.push((chunk_x,chunk_y));
                    }
            
                }
            }
            chunks_to_create
        };
        
        for chunk in chunks_to_create {
            self.load_chunk(chunk.0,chunk.1);
        }

    }
    
}

impl GameDimension {
    
    pub fn load_chunk(&mut self, chunk_x : i32, chunk_y : i32,) {
        let mut chunk_data: [Block; 16*16] = [const { Block::Unloaded }; 16*16];
        let mut biome_data: [Biome; 16*16] = [const {Biome::Plains}; 16*16];
        let mut environmental_objects: Vec<EnvironmentalObject> = Vec::new();
        let mut dropped_items: Vec<DroppedItem> = Vec::new();
        let mut rng = rand::thread_rng();
        let biome_noise = &self.biome_noise;

        let mut i: usize = 0;
        while i < 16 * 16 {
            //println!("gen");
            let block_x = (chunk_x * 16) + i as i32 % 16;
            let block_y = (chunk_y * 16) + (i as f32 / 16.0).floor() as i32;

            let biome_humidity_result = (simplex_2d(Vector2::new((block_x as f32 / 800.0) as f64, (block_y as f32 / 800.0) as f64), &biome_noise.biome_humidity).0 + 1.0) / 2.0;
            //let biome_spiritual_result = (simplex_2d(Vector2::new((block_x as f32 / 10.0) as f64, (block_y as f32 / 10.0) as f64), &biome_noise.biome_spiritual).0 + 1.0) / 2.0;
            let biome_height_result = (simplex_2d(Vector2::new((block_x as f32 / 800.0) as f64, (block_y as f32 / 800.0) as f64), &biome_noise.biome_height).0 + 1.0) / 2.0;
            let biome_temperature_result = (simplex_2d(Vector2::new((block_x as f32 / 2400.0) as f64, (block_y as f32 / 2400.0) as f64), &biome_noise.biome_temperature).0 + 1.0) / 2.0;


            let mut biome;
            let block;


            //get the biome 
            if biome_temperature_result > 0.66 {
                if biome_humidity_result > 0.8 {
                    biome = Biome::TropicalJungle;
                }else if biome_humidity_result > 0.33 {
                    biome = Biome::SeasonalForest;
                }else{
                    biome = Biome::Desert;
                }
            }else if biome_temperature_result > 0.33 {
                if biome_humidity_result > 0.66 {
                    biome = Biome::DarkForest;
                }else if biome_humidity_result > 0.33 {
                    biome = Biome::Forest;
                }else{
                    biome = Biome::Plains;
                }
            
            }else if biome_temperature_result > 0.1{
                if biome_humidity_result > 0.66 {
                    biome = Biome::Forest;
                }else if biome_humidity_result > 0.33 {
                    biome = Biome::Shrubland;
                }else{
                    biome = Biome::Plains;
                }
            }else{
                if biome_humidity_result > 0.66 {
                    biome = Biome::SnowyPlains;
                }else if biome_humidity_result > 0.25 {
                    biome = Biome::Plains;
                }else{
                    biome = Biome::Tundra;
                }
            }
            if biome == Biome::DarkForest || biome == Biome::SeasonalForest {
                if biome_height_result < 0.30 {
                    biome = Biome::Swamp;
                }
            }
            if biome == Biome::Plains || biome == Biome::Forest  {
                if biome_height_result < 0.27 {
                    biome = Biome::Beach;
                }
            }
            if biome_height_result < 0.25 {
                biome = Biome::Ocean;
            }
            
            let offset_environment_item_x: f32 = rng.gen();
            let offset_environment_item_x = (offset_environment_item_x - 0.5) * 0.99;
            let offset_environment_item_y: f32 = rng.gen();
            let offset_environment_item_y = (offset_environment_item_y - 0.5) * 0.99;

            match biome {
                Biome::Plains => {
                    block = Block::Grass;
                    let chance: f32 = rng.gen();

                    if chance < 0.025 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::Bush,
                            position: Position{
                                x: block_x as f32,
                                y: block_y as f32,
                            },
                            id: Uuid::new_v4(),
                        });
                    }

                },
                Biome::Desert => {
                    block = Block::Sand;
                    let chance: f32 = rng.gen();

                    if chance < 0.025 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::DeadTree,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }
                },
                Biome::SeasonalForest => {
                    block = Block::SeasonalGrass;
                },
                Biome::Forest => {
                    block = Block::Grass;
                    let chance: f32 = rng.gen();
                    if chance < 0.1 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::Tree,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }else if chance < 0.15 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::Bush,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }else if chance < 0.4 {
                        let rotation: f32 = rng.gen();
                        let rotation = rotation * 360.0;
                        dropped_items.push(DroppedItem { 
                            item: Item{
                                item:PremadeItem::Stick,
                                enchantments: Vec::new(),
                            }, 
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            count: 1, 
                            rotation: rotation,
                            id: Uuid::new_v4(),
                        });

                    }
                },
                Biome::Swamp => {
                    let biome_swamp_water_result = (simplex_2d(Vector2::new((block_x as f32 / 20.0) as f64, (block_y as f32 / 20.0) as f64), &biome_noise.biome_swamp_water).0 + 1.0) / 2.0;
                    let chance: f32 = rng.gen();

                    if biome_swamp_water_result + biome_height_result > 0.7 {
                        block = Block::SwampGrass;

                        if chance < 0.1 {
                            environmental_objects.push(EnvironmentalObject{
                                object_type: environmental_object::EnvironmentalObjectType::SwampTree,
                                position: Position{
                                    x: block_x as f32 + offset_environment_item_x,
                                    y: block_y as f32 + offset_environment_item_y,
                                },
                                id: Uuid::new_v4(),
                            });
                        }

                    }else{
                        block = Block::SwampWater;

                        if chance < 0.0333 {
                            environmental_objects.push(EnvironmentalObject{
                                object_type: environmental_object::EnvironmentalObjectType::SwampTree,
                                position: Position{
                                    x: block_x as f32 + offset_environment_item_x,
                                    y: block_y as f32 + offset_environment_item_y,
                                },
                                id: Uuid::new_v4(),
                            });
                        }
                    }
                },
                Biome::Tundra => {
                    let biome_swamp_water_result = (simplex_2d(Vector2::new((block_x as f32 / 20.0) as f64, (block_y as f32 / 20.0) as f64), &biome_noise.biome_swamp_water).0 + 1.0) / 2.0;
                    if biome_swamp_water_result + biome_humidity_result > 1.0 {
                        block = Block::SnowyGrass;
                    }else{
                        block = Block::SeasonalGrass;
                    }
                    let chance: f32 = rng.gen();

                    if chance < 0.1 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::DeadTree,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }
                },
                Biome::Shrubland => {
                    block = Block::Grass;
                    let chance: f32 = rng.gen();

                    if chance < 0.1 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::Bush,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }else if chance < 0.2 {
                        let rotation: f32 = rng.gen();
                        let rotation = rotation * 360.0;
                        dropped_items.push(DroppedItem { 
                            item: Item{
                                item:PremadeItem::Stick,
                                enchantments: Vec::new(),
                            }, 
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            count: 1, 
                            rotation: rotation,
                            id: Uuid::new_v4(),
                        });

                    }


                },
                Biome::TropicalJungle => {
                    block = Block::Grass;
                    let chance: f32 = rng.gen();

                    if chance < 0.4 {
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::JungleTree,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }else if chance < 0.5{
                        environmental_objects.push(EnvironmentalObject{
                            object_type: environmental_object::EnvironmentalObjectType::Bush,
                            position: Position{
                                x: block_x as f32 + offset_environment_item_x,
                                y: block_y as f32 + offset_environment_item_y,
                            },
                            id: Uuid::new_v4(),
                        });
                    }
                },
                Biome::DarkForest => {
                    block = Block::DarkGrass;
                },
                Biome::Beach => {
                    block = Block::Sand;
                },
                Biome::SnowyPlains => {
                    block = Block::SnowyGrass;
                },
                Biome::Ocean => {
                    block = Block::Water;
                },
            }


            chunk_data[i] = block;
            biome_data[i] = biome;

            i += 1;
        }

        let mut environmental_objects_vec: Vec<Uuid> = Vec::new();
        let mut dropped_items_vec: Vec<Uuid> = Vec::new();

        for environmental_object in environmental_objects {
            environmental_objects_vec.push(environmental_object.id);
            self.environmental_objects.insert(environmental_object.id, environmental_object);
        }
        for dropped_item in dropped_items {
            dropped_items_vec.push(dropped_item.id);
            self.dropped_items.insert(dropped_item.id, dropped_item);
        }






        self.chunks.insert((chunk_x,chunk_y), 
        Chunk{
            block_data: chunk_data,
            biome_data : biome_data,
            environmental_objects: environmental_objects_vec,
            monsters : Vec::new(),
            dropped_items: dropped_items_vec,
        });
    }
}