use raylib::texture::Texture2D;

use crate::assets::Assets;
use crate::game_dimension::GameDimension;
use crate::physics::Position;
use crate::player::Player;
use super::camera::Camera;
use super::render_utils::get_texture_from_item;
use super::RenderBufferItem;

pub fn render_player<'a>(render_buffer : &mut Vec<RenderBufferItem<'a>>, player : &Player, game_dimension : &GameDimension, camera : &Camera, assets : &'a Assets){
    render_buffer.push(RenderBufferItem{
        render_position: Position{
            x: camera.convert_x_pos_to_screen(&player.movement.position.x,&1.0, 0.0),
            y: camera.convert_y_pos_to_screen(&player.movement.position.y,&1.0, 0.0),
        },
        render_asset: &assets.player,
        real_position: Position{
            x: player.movement.position.x,
            y: player.movement.position.y,
        },
        render_width: 1.0 / camera.zoom,
        render_rotation: 0.0,
        health_bar_buffer_info: None,
        layer: 5,
    });

    //draw item holding
    let item_holding_mainhand = player.main_hand.clone();
    let _item_holding_offhand = player.off_hand.clone();
    if let Some(item) = item_holding_mainhand {
        let texture: &Texture2D = get_texture_from_item(&item, &assets);
        render_buffer.push(RenderBufferItem{
            render_position: Position{
                x: camera.convert_x_pos_to_screen(&(player.movement.position.x + &player.facing.to_radians().cos() * 0.75), &1.0, &player.facing + 90.0),
                y: camera.convert_y_pos_to_screen(&(player.movement.position.y + &player.facing.to_radians().sin() * -0.75), &1.0, &player.facing + 90.0),
            },
            render_asset: texture,
            real_position: Position{
                x: player.movement.position.x,
                y: player.movement.position.y,
            },
            render_width: 1.0 / camera.zoom,
            render_rotation: &player.facing + 90.0,
            health_bar_buffer_info: None,
            layer: 5,
        });
    }
}