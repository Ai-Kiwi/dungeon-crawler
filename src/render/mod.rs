pub mod camera;
pub mod render_entities;
pub mod render_map;
pub mod render_gui;
pub mod render_player;
pub mod render_utils;

use std::cmp::Ordering;

use camera::Camera;
use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};
use assets::Assets;
use render_gui::GuiPagesInfo;
use crate::{assets, entities::monster::MonsterType, game_dimension::GameDimension, physics::Position, player::Player};
use crate::render::render_map::render_map;
use crate::render::render_gui::render_ui;
use crate::render::render_player::render_player;

pub struct HealthBarBufferInfo {
    health: f32,
    max_health: f32,
    x_pos: f32,
    y_pos: f32,
    level: u32,
    mob: MonsterType,
}

pub struct RenderBufferItem<'a> {
    render_position : Position,
    render_asset : &'a Texture2D,
    real_position : Position,
    render_width : f32,
    render_rotation : f32,
    health_bar_buffer_info: Option<HealthBarBufferInfo>,
    layer: u8,
}

pub fn main_render(d: &mut RaylibDrawHandle,game_dimension : &GameDimension, player : &mut Player, camera : &Camera, assets : &Assets, gui_pages_info : &mut GuiPagesInfo) {
    d.clear_background(Color::PURPLE);
    let mut render_buffer: Vec<RenderBufferItem> = Vec::new();

    render_map(&mut render_buffer, game_dimension, camera, assets);

    render_player(&mut render_buffer, player, game_dimension, camera, assets);


    render_buffer.sort_by(|a, b|{
        if a.layer != b.layer {
            if a.layer < b.layer {
                Ordering::Less
            }else{
                Ordering::Greater
            }
        }else{
            if a.real_position.y > b.real_position.y {
                Ordering::Less
            }else if a.real_position.y == b.real_position.y{
                if a.real_position.x > b.real_position.x {
                    Ordering::Less
                }else{
                    Ordering::Greater
                }
            }else{
                Ordering::Greater
            }
        }
    });

    //draw health bar for monsters
    for item in render_buffer.iter() {
        d.draw_texture_ex(item.render_asset, Vector2::new(item.render_position.x, item.render_position.y), item.render_rotation, item.render_width / (item.render_asset.width as f32), Color::WHITE);
        match &item.health_bar_buffer_info {
            Some(bar_info) => {
                bar_info.health;
                bar_info.max_health;
                d.draw_rectangle((bar_info.x_pos - 50.0).floor() as i32, (bar_info.y_pos - 20.0).floor() as i32, 100, 10, Color::BLACK);

                d.draw_rectangle((bar_info.x_pos - 50.0).floor() as i32 + 2, (bar_info.y_pos - 20.0).floor() as i32 + 2, (100.0 * (bar_info.health / bar_info.max_health)).floor() as i32 - 4, 10 - 4, Color::RED);

                //draw item type

                //draw level
                let level = bar_info.level;
                let name = MonsterType::get_mob_name(bar_info.mob);
                d.draw_text(&format!("Level : {level}"), (bar_info.x_pos - 50.0).floor() as i32, (bar_info.y_pos - 32.5).floor() as i32, 12, Color::WHITE);
                d.draw_text(&format!("{name}"), (bar_info.x_pos - 50.0).floor() as i32, (bar_info.y_pos - 42.5).floor() as i32, 12, Color::WHITE);


            },
            None => (),
        }

    }

    render_ui(&mut render_buffer, player, game_dimension, camera, d, assets, gui_pages_info);
}

pub struct MouseInfo {
    mouse_left_click : bool,
    x : f32,
    y : f32,
}


