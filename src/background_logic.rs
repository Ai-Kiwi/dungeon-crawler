
use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::{entities::monster::{create_monster, Monster, MonsterType}, game_dimension::{self, get_chunks_in_range, GameDimension}, physics::Position, player::Player};

struct MonsterSpawnChance {
    mob: MonsterType,
    chance: f32,
    spawn_radius: f32,
    max_spawns_per_radius: u32,
}

impl GameDimension {
    pub fn background_tick(&mut self, player: &mut Player) {
        let _ = player;
        let mut monsters_to_delete = Vec::new();

        for monster in  &mut self.monsters {
            monster.1.tick(player, self.tick_number);
            if monster.1.health <= 0.0 {
                monsters_to_delete.push(*monster.0);
            }
        }

        for monster in monsters_to_delete {
            let chunk_pos = GameDimension::position_to_chunk( &self.monsters.get(&monster).unwrap().movement.position  );
            self.chunks.get_mut(&chunk_pos).unwrap().monsters.retain(|&x| x != monster);
            self.monsters.remove(&monster);
        }

        //handle if to tick mob spawns
        if (self.tick_number % 60) == 0 {
            self.tick_mob_spawning(player);
        }
    }

    pub fn tick_mob_spawning(&mut self, player: &mut Player){
        //test if it has been 60 ticks, so only runs every second
        println!("running mob spawns");
        
        //todo mob spawning it picks a random block in the chunk then seees if it is safe to spawn there if it is it does it.
        //this approach has some problems but is what currently works
        //will be done for each chunk each tick.

        //per chunk checks
        //make sure player nearby is not in range
        //make sure no more mobs nearby

        let loaded_chunks: Vec<(i32,i32)> = self.chunks.keys().cloned().collect();
        //println!("{}",loaded_chunks.iter().count());
        for chunk in loaded_chunks {
            
            let mut monster_spawn_chances: Vec<MonsterSpawnChance> = Vec::new();
            //run logic for chunk
            let chunk_x = chunk.0;
            let chunk_y = chunk.1;

            let offset_x: f32 = thread_rng().gen();
            let offset_x: i32 = (offset_x * 16.0).round() as i32;
            let offset_y: f32 = thread_rng().gen();
            let offset_y: i32 = (offset_y * 16.0).round() as i32;

            let spawn_loc_x = (chunk_x * 16) + offset_x;
            let spawn_loc_y = (chunk_y * 16) + offset_y;

            match self.get_biome(spawn_loc_x, spawn_loc_y) {
                game_dimension::Biome::Plains => {
                    monster_spawn_chances.push(MonsterSpawnChance{
                        mob: MonsterType::Ghost,
                        chance: 0.05,
                        spawn_radius: 128.0,
                        max_spawns_per_radius: 3,
                    });
                },
                game_dimension::Biome::Desert => {
                    monster_spawn_chances.push(MonsterSpawnChance{
                        mob: MonsterType::Ghost,
                        chance: 0.05,
                        spawn_radius: 128.0,
                        max_spawns_per_radius: 3,
                    });
                },
                game_dimension::Biome::SeasonalForest => (),
                game_dimension::Biome::Forest => {
                    monster_spawn_chances.push(MonsterSpawnChance{
                        mob: MonsterType::Ghost,
                        chance: 0.05,
                        spawn_radius: 128.0,
                        max_spawns_per_radius: 9,
                    });
                },
                game_dimension::Biome::Swamp => (),
                game_dimension::Biome::Tundra => (),
                game_dimension::Biome::Shrubland => {
                    monster_spawn_chances.push(MonsterSpawnChance{
                        mob: MonsterType::Ghost,
                        chance: 0.05,
                        spawn_radius: 128.0,
                        max_spawns_per_radius: 3,
                    });
                },
                game_dimension::Biome::TropicalJungle => (),
                game_dimension::Biome::DarkForest => (),
                game_dimension::Biome::Beach => (),
                game_dimension::Biome::SnowyPlains => (),
                game_dimension::Biome::Ocean => (),
            }

            for chance in monster_spawn_chances {
                let odd: f32 = thread_rng().gen();
                if odd <= chance.chance {
                    let in_range_chunks = get_chunks_in_range(&Position{x:spawn_loc_x as f32, y: spawn_loc_y as f32}, chance.spawn_radius);
                    let mut nearby_count = 0;
                    let mut test_mob_ids: Vec<Uuid> = Vec::new();
                    for counting_chunk in in_range_chunks {
                        match self.chunks.get(&counting_chunk) {
                            Some(chunk_data) => {
                                for monster in &chunk_data.monsters {
                                    test_mob_ids.push(*monster);
                                }
                            },
                            None => (),
                        }
                    }
                    for monster_id in test_mob_ids {
                        match Monster::from_id(&self.monsters, monster_id) {
                            Some(monster) => {
                                if monster.mob_type == chance.mob && monster.movement.position.distance_to(&Position{x:spawn_loc_x as f32, y: spawn_loc_y as f32}) <= chance.spawn_radius {
                                    nearby_count = nearby_count + 1;
                                }
                            },
                            None => (),
                        }
                    }


                    if nearby_count < chance.max_spawns_per_radius && player.movement.position.distance_to(&Position{x:spawn_loc_x as f32, y: spawn_loc_y as f32}) >= 50.0 {
                        let mut monster = create_monster(chance.mob);
                        monster.movement.position.x = spawn_loc_x as f32 + 0.5;
                        monster.movement.position.y = spawn_loc_y as f32+ 0.5;
                        println!("{}", nearby_count);
                        //spawn in the mob
                        self.chunks.get_mut(&chunk).unwrap().monsters.push(monster.id);
                        self.monsters.insert(monster.id, monster);
                    }



                }
            }

            //around a spawn ever
            
        }
    }
}

