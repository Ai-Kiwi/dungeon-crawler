
use crate::{entities::monster::Monster, game_dimension::GameDimension, player::Player};




impl GameDimension {
    pub fn tick_physics(&mut self){
        let mut temp_monsters_moved_chunk: Vec<(Monster,(i32,i32))> = Vec::new();
        {
            let chunk_iter = self.chunks.iter_mut();
            for chunk in chunk_iter{
                let chunk_lock = chunk.1;
        
                //tick monsters
                for monster in &mut chunk_lock.monsters {
                    monster.movement.tick_movement();
        
                }
                
                //remove monster from list that have been deleted
                chunk_lock.monsters.retain(|monster| {
                    let new_monster_pos = monster.movement.position.clone(); 
                    if (new_monster_pos.x / 16.0).floor() as i32 != chunk.0.0 || (new_monster_pos.y / 16.0).floor() as i32 != chunk.0.1 {
                        //move monster to new chunk
                        temp_monsters_moved_chunk.push((monster.clone(),chunk.0.clone()));
                        false
                    }else{
                        true
                    }
                });
                    
            }
        }

        //add monsters which have moved into another chunk
        for moved_monster in temp_monsters_moved_chunk {
            match self.chunks.get_mut(&( (moved_monster.0.movement.position.x / 16.0).floor() as i32, (moved_monster.0.movement.position.y / 16.0).floor() as i32 )) {
                Some(chunk) => {
                    chunk.monsters.push(moved_monster.0);
                },
                None => {
                    println!("mob walked into unloaded chunk forgetting about")
                },
            }

        }
    }

}

impl Player {
    pub fn handle_movement(&mut self) {
        let player_walk_dir: (i8, i8) = self.walk_dir;
        let player_speed = self.speed;
    
        if player_walk_dir.0 != 0{
            self.movement.velocity.x += player_walk_dir.0 as f32 * player_speed;
        }   
        if player_walk_dir.1 != 0{
            self.movement.velocity.y += player_walk_dir.1 as f32 * player_speed;
        }   

        self.movement.tick_movement();
    }
}


#[derive(Clone)]
pub struct Movement {
    pub velocity: Velocity,
    pub position: Position,
    pub drag : f32,
    pub mass : f32,
}

impl Movement {
    pub fn tick_movement(&mut self) {
        self.position.x = self.position.x + self.velocity.x;
        self.position.y = self.position.y +  self.velocity.y;
        self.velocity.x = self.velocity.x * self.drag;
        self.velocity.y = self.velocity.y * self.drag;
    }
}

#[derive(Clone)]
pub struct Velocity {
    pub x : f32,
    pub y : f32,
}
#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance_to(&self, target_position: &Position) -> f32 {
        let position = self;
        let distance = ((position.x - target_position.x).powi(2) + (position.y - target_position.y).powi(2)).sqrt();
        distance
    }
}
