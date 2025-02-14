
use crate::{entities::monster::Monster, game_dimension::GameDimension, player::Player};




impl GameDimension {
    pub fn tick_physics(&mut self){
        let mut temp_monsters_moved_chunk: Vec<(Monster,(i32,i32))> = Vec::new();
        {

            //tick monsters
            for monster in &mut self.monsters {
                let old_movement_pos = monster.1.movement.position.clone();
                monster.1.movement.tick_movement();

                //look if gone into unloaded chunks
                let old_chunk_pos = GameDimension::position_to_chunk(&old_movement_pos);
                let new_chunk_pos = GameDimension::position_to_chunk(&monster.1.movement.position);

                if self.chunks.contains_key(&new_chunk_pos) == false {
                    println!("{} {}", old_chunk_pos.0, old_chunk_pos.1);
                    println!("{} {}", new_chunk_pos.0, new_chunk_pos.1);
                    println!("{}", monster.1.id);
                    println!("{} {}",monster.1.movement.position.x,monster.1.movement.position.y);
                    monster.1.movement.position = old_movement_pos;
                    println!("{} {}",monster.1.movement.position.x,monster.1.movement.position.y);
                    println!("mob pressing moved into unloaded chunks, teleported back");
                    continue;
                }
                if old_chunk_pos != new_chunk_pos {
                    self.chunks.get_mut(&old_chunk_pos).unwrap().monsters.retain(|&x| &x != monster.0);
                    self.chunks.get_mut(&new_chunk_pos).unwrap().monsters.push(*monster.0);
                }
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
    pub fn apply_force_towards(&mut self, target: &Position, force: f32) {
        let size_x = target.x - self.position.x;
        let size_y = target.y - self.position.y;
        let size = ((size_x * size_x) + (size_y * size_y)).sqrt();
        self.velocity.x += self.velocity.x +((size_x / size) * force);
        self.velocity.y +=self.velocity.y + ((size_y / size) * force);
    }
}

#[derive(Clone)]
pub struct Velocity {
    pub x : f32,
    pub y : f32,
}
impl Velocity {
    pub fn new() -> Self { 
        Velocity { 
            x: 0.0, 
            y: 0.0 
        }
    }
    fn add(self, other: &Velocity) -> Velocity {
        Velocity {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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
    pub fn new() -> Self { 
        Position { 
            x: 0.0, 
            y: 0.0 
        }
    }
}