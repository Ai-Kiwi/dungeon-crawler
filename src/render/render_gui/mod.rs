pub mod render_hotbar;
pub mod render_inventory;
pub mod render_sidemenu;
use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use raylib::{ffi::MouseButton, prelude::RaylibDrawHandle};
use render_sidemenu::SideMenuState;
use crate::player;
use crate::render::render_gui::render_sidemenu::render_sidemenu;

use crate::{assets::Assets, game_dimension::GameDimension, item::Item, player::Player, render::render_gui::render_hotbar::render_hotbar};
use crate::render::render_gui::render_inventory::render_inventory;
use super::{camera::Camera, MouseInfo, RenderBufferItem};

pub struct GuiPagesInfo {
    pub inventory_open : bool,
    pub inventory_selected_slot : i32,
    pub inventory_selected_item: Option<Item>,
    pub inventory_scroll_offset : i32,
    pub hotbar_selected_slot : i32,
    pub side_menu_page_open : SideMenuPage,
    pub side_menu_open: bool,
    pub side_menu_state: SideMenuState,
}
impl GuiPagesInfo {
    pub fn new(player : &Player) -> Self {
        Self {
            inventory_open: false,
            inventory_selected_slot: 0,
            hotbar_selected_slot: 0,
            inventory_scroll_offset: 0,
            inventory_selected_item: None,
            side_menu_page_open: SideMenuPage::Info,
            side_menu_open: true,
            side_menu_state: SideMenuState::new(player),
        }
    }
}
#[derive(PartialEq)]
pub enum SideMenuPage {
    Stats,
    Crafting,
    Info
}

pub fn render_ui(render_buffer : &mut Vec<RenderBufferItem>, player : &mut Player, game_dimension : &GameDimension, camera : &Camera, d: &mut RaylibDrawHandle, assets : &Assets, gui_pages_info : &mut GuiPagesInfo) {
    let mouse_info : MouseInfo = MouseInfo {
        mouse_left_click: d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
        x: d.get_mouse_x() as f32,
        y: d.get_mouse_y() as f32,
    };

    render_hotbar(render_buffer, player, game_dimension, camera, d, assets, &mouse_info, gui_pages_info);

    render_inventory(gui_pages_info, player, camera, assets, d);

    if gui_pages_info.side_menu_open {
        render_sidemenu(d, gui_pages_info, player, camera, assets);
    }

}