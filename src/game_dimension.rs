use std::collections::HashMap;
use std::hash::Hash;

use noise::permutationtable::PermutationTable;
use rand::Rng;
use uuid::Uuid;

use crate::entities::dropped_item::DroppedItem;
use crate::entities::environmental_object::{self, EnvironmentalObject};
use crate::entities::monster::Monster;
use crate::physics::Position;


pub enum MagicElements {
    Fire,
    Ice,
    Wind,
    Earth,
    Dark,
    Light,
    
}


#[derive(Clone)]
pub enum Block {
    Dirt,
    DirtWithPebble,
    Cobblestone,
    Unloaded,
    Sand,
    Grass,
    SeasonalGrass,
    DarkGrass,
    Water,
    SwampWater,
    SwampGrass,
    SnowyGrass,
}

impl Block {
    pub fn info(&self) -> &'static BlockInfo {
        &BLOCK_INFOS[self.clone() as usize]
    }
}

pub struct BlockInfo {
    is_solid: bool,
}

pub static BLOCK_INFOS: &[BlockInfo] = &[
    BlockInfo { is_solid: false },
    BlockInfo { is_solid: false },
    BlockInfo { is_solid: false },
    BlockInfo { is_solid: true },
    BlockInfo { is_solid: true },
    BlockInfo { is_solid: true },

];




#[derive(Clone,PartialEq)]
pub enum Biome {
    Plains,
    Desert,
    SeasonalForest,
    Forest,
    Swamp,
    Tundra,
    Shrubland,
    TropicalJungle,
    DarkForest,
    Beach,
    SnowyPlains,
    Ocean,
}

pub struct Chunk {
    pub block_data : [Block; 16 * 16],
    pub biome_data : [Biome; 16 * 16],
    pub environmental_objects : Vec<Uuid>,
    pub monsters : Vec<Uuid>,
    pub dropped_items : Vec<Uuid>,
}

#[derive(Clone)]
pub struct BiomeNoise {
    pub biome_humidity : PermutationTable,
    pub biome_spiritual : PermutationTable,
    pub biome_height : PermutationTable,
    pub biome_temperature : PermutationTable,
    pub biome_swamp_water : PermutationTable,
}


pub struct GameDimension{
    pub chunks : HashMap<(i32,i32),Chunk>,
    pub biome_noise: BiomeNoise,
    pub tick_number: u128,
    pub environmental_objects : HashMap<Uuid, EnvironmentalObject>,
    pub monsters : HashMap<Uuid, Monster>,
    pub dropped_items : HashMap<Uuid, DroppedItem>,
}

impl GameDimension {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let biome_humidity_seed: f32 = rng.gen();
        let biome_humidity_hasher = PermutationTable::new((biome_humidity_seed * 4294967295.0) as u32);
        let biome_spiritual_seed: f32 = rng.gen();
        let biome_spiritual_hasher = PermutationTable::new((biome_spiritual_seed * 4294967295.0) as u32);
        let biome_height_seed: f32 = rng.gen();
        let biome_height_hasher = PermutationTable::new((biome_height_seed * 4294967295.0) as u32);
        let biome_temperature_seed: f32 = rng.gen();
        let biome_temperature_hasher = PermutationTable::new((biome_temperature_seed * 4294967295.0) as u32);
        let biome_swamp_water_seed: f32 = rng.gen();
        let biome_swamp_water_hasher = PermutationTable::new((biome_swamp_water_seed * 4294967295.0) as u32);
        
        Self {
            chunks : HashMap::new(),
            biome_noise: BiomeNoise{
                biome_humidity: biome_humidity_hasher,
                biome_spiritual: biome_spiritual_hasher,
                biome_height: biome_height_hasher,
                biome_temperature: biome_temperature_hasher,
                biome_swamp_water : biome_swamp_water_hasher,
            },
            tick_number: 0,
            environmental_objects: HashMap::new(),
            monsters: HashMap::new(),
            dropped_items: HashMap::new(),
        }
    }

    pub fn get_block(&self, x : i32, y : i32) -> Block {
        let chunk_x = (x as f32 / 16.0).floor() as i32; 
        let chunk_y = (y as f32 / 16.0).floor() as i32;

        let chunk_data: Option<&Chunk> = self.chunks.get(&(chunk_x,chunk_y));

        let in_chunk_x = x - (chunk_x * 16);
        let in_chunk_y= y - (chunk_y * 16);

        match chunk_data {
            Some(data) => {
                let array_loc: usize = (in_chunk_x + (in_chunk_y * 16)).try_into().unwrap();
                let result = data.block_data.get(array_loc).unwrap().clone();
                result
            },
            None => Block::Unloaded,
        }
    }

    pub fn get_biome(&self, x : i32, y : i32) -> Biome {
        let chunk_x = (x as f32 / 16.0).floor() as i32; 
        let chunk_y = (y as f32 / 16.0).floor() as i32;

        let chunk_data: Option<&Chunk> = self.chunks.get(&(chunk_x,chunk_y));

        let in_chunk_x = x - (chunk_x * 16);
        let in_chunk_y= y - (chunk_y * 16);

        match chunk_data {
            Some(data) => {
                let array_loc: usize = (in_chunk_x + (in_chunk_y * 16)).try_into().unwrap();
                let result = data.biome_data.get(array_loc).unwrap().clone();
                result
            },
            None => Biome::Shrubland,
        }
    }

    pub fn position_to_chunk(position : &Position) -> (i32,i32) {
        let chunk_x = (position.x / 16.0).floor() as i32;
        let chunk_y = (position.y / 16.0).floor() as i32;
        (chunk_x,chunk_y)
    }


}


pub fn get_chunks_in_range(position : &Position, radius: f32,) -> Vec<(i32,i32)> {
    let left_screen_chunk: i32 = ((position.x - (radius / 2.0)) / 16.0 ).floor() as i32;
    let bottom_screen_chunk: i32 = ((position.y - (radius / 2.0)) / 16.0 ).floor() as i32;
    let right_screen_chunk: i32 = ((position.x + (radius / 2.0)) / 16.0 ).ceil() as i32;
    let top_screen_chunk: i32 = ((position.y + (radius / 2.0)) / 16.0 ).ceil() as i32;
    let mut nearby_chunks = Vec::new();
    for chunk_x in left_screen_chunk..right_screen_chunk {
        for chunk_y in bottom_screen_chunk..top_screen_chunk {
            nearby_chunks.push((chunk_x,chunk_y));
        }
    }
    nearby_chunks
}