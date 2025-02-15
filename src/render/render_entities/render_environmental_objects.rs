use crate::{assets::Assets, entities::environmental_object::{self, EnvironmentalObject}, game_dimension::{Chunk, GameDimension}, physics::Position, render::{camera::Camera, RenderBufferItem}};

pub fn render_environmental_objects<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, chunk : &Chunk, game_dimension : &GameDimension, camera : &Camera, assets : &'a Assets) {      
    for item_id in chunk.environmental_objects.iter() {
        let item = EnvironmentalObject::from_id(&game_dimension.environmental_objects, *item_id).unwrap();
        match item.object_type {
            environmental_object::EnvironmentalObjectType::DeadTree => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&1.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&1.0, 0.0),
                    },
                    render_asset: &assets.dead_tree,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 2.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
            environmental_object::EnvironmentalObjectType::Tree => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                    },
                    render_asset: &assets.tree,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 2.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
            environmental_object::EnvironmentalObjectType::AppleTree => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                    },
                    render_asset: &assets.apple_tree,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 2.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
            environmental_object::EnvironmentalObjectType::SwampTree => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                    },
                    render_asset: &assets.swamp_tree,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 2.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
            environmental_object::EnvironmentalObjectType::Bush => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&1.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&&item.position.y,&1.0, 0.0),
                    },
                    render_asset: &assets.bush,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 1.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
            environmental_object::EnvironmentalObjectType::JungleTree => {
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x,&3.0, 0.0),
                        y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&3.0, 0.0),
                    },
                    render_asset: &assets.jungle_tree,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 3.0 / camera.zoom,
                    render_rotation: 0.0,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            },
        }
    }
}