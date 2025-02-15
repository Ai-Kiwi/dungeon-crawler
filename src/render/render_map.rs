use crate::{assets::Assets, game_dimension::{Block, Chunk, GameDimension}, physics::Position};

use super::{camera::Camera, render_entities::{render_dropped_items::render_dropped_items, render_environmental_objects::render_environmental_objects, render_monsters::render_monsters}, RenderBufferItem};

pub fn render_map<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, game_dimension : &GameDimension, camera : &Camera, assets : &'a Assets) {
    //calculate what is in range of camera
    let left_screen_chunk: i32 = ((camera.position.x - ((camera.screen_width / 2) as f32) * camera.zoom) / 16.0 ).floor() as i32;
    let bottom_screen_chunk: i32 = ((camera.position.y - ((camera.screen_height / 2) as f32) * camera.zoom) / 16.0 ).floor() as i32;
    let right_screen_chunk: i32 = ((camera.position.x + ((camera.screen_width / 2) as f32) * camera.zoom + 1.0) / 16.0 ).ceil() as i32; //not sure why just seemed to need 1 more
    let top_screen_chunk: i32 = ((camera.position.y + ((camera.screen_height / 2) as f32) * camera.zoom + 1.0) / 16.0 ).ceil() as i32;

    for chunk_x in left_screen_chunk..right_screen_chunk {
        for chunk_y in bottom_screen_chunk..top_screen_chunk {
            if let Some(chunk) = game_dimension.chunks.get(&(chunk_x, chunk_y)) {
                render_background_tiles(render_buffer, chunk, game_dimension, camera, &(chunk_x,chunk_y), assets);
                render_monsters(render_buffer, chunk, game_dimension, camera, &assets);
                render_environmental_objects(render_buffer, chunk, game_dimension, camera, assets);
                render_dropped_items(render_buffer, chunk, game_dimension, camera, assets);
            }
        }
    }
}

fn render_background_tiles<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, chunk : &Chunk, game_dimension : &GameDimension, camera : &Camera, chunk_pos : &(i32,i32), assets : &'a Assets) {                        
    let mut i=0;
    let (chunk_x, chunk_y) = *chunk_pos;
    
    for y in (0+(chunk_y*16))..(16+(chunk_y*16)){
        for x in (0+(chunk_x*16))..(16+(chunk_x*16)){
            let block = &chunk.block_data[i];
        
            let texture = match block {
                Block::Dirt => &assets.dirt,
                Block::Cobblestone => &assets.stone,
                Block::Unloaded => &assets.invalid,
                Block::DirtWithPebble => &assets.dirt_with_pebble,
                Block::Sand => &assets.sand,
                Block::Grass => &assets.grass,
                Block::SeasonalGrass => &assets.seasonal_grass,
                Block::DarkGrass => &assets.dark_grass,
                Block::Water => &assets.water,
                Block::SwampGrass => &assets.swamp_grass,
                Block::SwampWater => &assets.swamp_water,
                Block::SnowyGrass => &assets.snowy_grass,
            };
            
            render_buffer.push(RenderBufferItem{
                render_position: Position{
                    x: camera.convert_x_pos_to_screen(&(x as f32 + 0.5), &1.0, 0.0),
                    y: camera.convert_y_pos_to_screen(&(y as f32 + 0.5), &1.0, 0.0),
                },
                render_asset: texture,
                real_position: Position{
                    x: x as f32 + 0.5,
                    y: y as f32 + 0.5,
                },
                render_width: 1.0 / camera.zoom,
                render_rotation: 0.0,
                health_bar_buffer_info: None,
                layer: 1,
            });
            i = i + 1;
        }
    }
}