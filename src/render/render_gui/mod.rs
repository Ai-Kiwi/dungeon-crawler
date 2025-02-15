pub mod render_hotbar;
pub mod render_inventory;
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
}