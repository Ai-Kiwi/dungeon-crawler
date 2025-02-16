use crate::{assets::Assets, entities::monster::Monster, game_dimension::{Chunk, GameDimension}, physics::Position, render::{camera::Camera, HealthBarBufferInfo, RenderBufferItem}};

pub fn render_monsters<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, chunk : &Chunk, game_dimension : &GameDimension, camera : &Camera, assets : &'a Assets) {      
    for monster_id in chunk.monsters.iter() {
        let monster = match Monster::from_id(&game_dimension.monsters, *monster_id) {
            Some(value) => value,
            None => {
                println!("attempt to render monster id : {} doesn't exist in monster list", monster_id);
                continue
            },
        };
        let monster_data = monster;
        let x_pos = monster_data.movement.position.x;
        let y_pos = monster_data.movement.position.y;
            
        render_buffer.push(RenderBufferItem{
            render_position: Position{
                x: camera.convert_x_pos_to_screen(&x_pos,&1.0, 0.0),
                y: camera.convert_y_pos_to_screen(&y_pos,&1.0, 0.0),
            },
            render_asset: &assets.ghost,
            real_position: Position{
                x: x_pos,
                y: y_pos,
            },
            render_width: 1.0 / camera.zoom,
            render_rotation: 0.0,
            health_bar_buffer_info: {
                Some(HealthBarBufferInfo{
                    health: monster_data.health,
                    max_health: monster_data.max_health,
                    x_pos: camera.convert_x_pos_to_screen(&x_pos,&0.0, 0.0),
                    y_pos: camera.convert_y_pos_to_screen(&y_pos,&0.0, 0.0),
                    level: monster_data.level,
                    mob: monster_data.mob_type,
                })
            },
            layer: 10,
        });
    }
}