pub mod render_hotbar;
pub mod render_inventory;
use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use raylib::{ffi::MouseButton, prelude::RaylibDrawHandle};

use crate::{assets::Assets, game_dimension::GameDimension, item::Item, player::Player, render::render_gui::render_hotbar::render_hotbar};
use crate::render::render_gui::render_inventory::render_inventory;
use super::{camera::Camera, MouseInfo, RenderBufferItem};

pub struct GuiPagesInfo {
    pub inventory_open : bool,
    pub inventory_selected_slot : i32,
    pub inventory_selected_item: Option<Item>,
    pub inventory_scroll_offset : i32,
    pub hotbar_selected_slot : i32,
}
impl GuiPagesInfo {
    pub fn new() -> Self {
        Self {
            inventory_open: false,
            inventory_selected_slot: 0,
            hotbar_selected_slot: 0,
            inventory_scroll_offset: 0,
            inventory_selected_item: None,
        }
    }
}

pub fn render_ui(render_buffer : &mut Vec<RenderBufferItem>, player : &mut Player, game_dimension : &GameDimension, camera : &Camera, d: &mut RaylibDrawHandle, assets : &Assets, gui_pages_info : &mut GuiPagesInfo) {
    let mouse_info : MouseInfo = MouseInfo {
        mouse_left_click: d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
        x: d.get_mouse_x() as f32,
        y: d.get_mouse_y() as f32,
    };

    render_hotbar(render_buffer, player, game_dimension, camera, d, assets, &mouse_info, gui_pages_info);

    render_inventory(gui_pages_info, player, camera, assets, d);

    if gui_pages_info.inventory_open == false{
        d.draw_rectangle(0, 55, 400, 120, Color::new(50,50,50,100));
        d.draw_text("Kiwi Crawler pre release", 5, 60, 25, Color::WHITE);
        d.draw_text("W,A,S,D : Used for moving", 5, 80, 15, Color::WHITE);
        d.draw_text("I : open inventory", 5, 95, 15, Color::WHITE);
        d.draw_text("P : pickup items", 5, 110, 15, Color::WHITE);
        d.draw_text("TAB : swap current item between off hand and main", 5, 125, 15, Color::WHITE);
        d.draw_text("alt + scroll : zoom in/out", 5, 140, 15, Color::WHITE);
        d.draw_text("Scroll : change selected slot and scroll in inventory", 5, 155, 15, Color::WHITE);
    }

}