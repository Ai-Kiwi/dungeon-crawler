use crate::{assets::Assets, entities::dropped_item::DroppedItem, game_dimension::{Chunk, GameDimension}, physics::Position, render::{camera::Camera, render_utils::get_texture_from_item, RenderBufferItem}};

pub fn render_dropped_items<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, chunk : &Chunk, game_dimension : &GameDimension, camera : &Camera, assets : &'a Assets) {
    for dropped_item_id in &chunk.dropped_items {
        match DroppedItem::from_id(&game_dimension.dropped_items, *dropped_item_id) {
            Some(item) => {
                let texture = get_texture_from_item(&item.item, &assets);
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&item.position.x, &1.0, item.rotation),
                        y: camera.convert_y_pos_to_screen(&item.position.y, &1.0, item.rotation),
                    },
                    render_asset: texture,
                    real_position: Position{
                        x: item.position.x,
                        y: item.position.y,
                    },
                    render_width: 1.0 / camera.zoom,
                    render_rotation: item.rotation,
                    health_bar_buffer_info: None,
                    layer: 3,
                });
            },
            None => {
                println!("failed to render dropped item, doesn't exist. possibly was meant to be deleted from chunk but wasn't")
            },
        };

    }
}