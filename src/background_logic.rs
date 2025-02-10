
use crate::{entities::Entity, game_dimension::GameDimension, player::Player};


impl GameDimension {
    pub fn background_tick(&mut self, player: &mut Player) {
        let _ = player;

        let mut monsters_to_tick = Vec::new();

        for chunk in &self.chunks {
            {
                //tick monsters
                for monster in chunk.1.monsters.iter() {
                    //monster.tick(game_dimension);
                    monsters_to_tick.push(monster.clone());
                }
            }
        }

        for monster in monsters_to_tick.iter_mut() {
            monster.tick(self);
        }

        monsters_to_tick.retain(|x| {
            x.health > 0.0 
        });

        //add the monsters back
        //delete all monster vectors
        for chunk in &mut self.chunks {
            chunk.1.monsters = Vec::new();
            for monster in &monsters_to_tick {
                let chunk_x = (monster.position().x / 16.0).floor() as i32;
                let chunk_y = (monster.position().y / 16.0).floor() as i32;
                
                if chunk_x == chunk.0.0 && chunk_y == chunk.0.1 {
                    chunk.1.monsters.push(monster.clone());
                }

            }



        }

        //add to chunks


    
    
        //handle mob spawning for each chunk
        //let loaded_chunks: Vec<(i32,i32)> = self.chunks.keys().cloned().collect();
    
        //for chunk in loaded_chunks {
            //run logic for chunk
    
            //do mob spawning
            //pick a random location
            //let chunk_x = 
            //let chunk_y = 
            //()
        //}



    }
}