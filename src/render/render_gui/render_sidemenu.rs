use raylib::{color::{self, Color}, ffi::{true_, Font, GetFontDefault, MouseButton}, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::{assets::Assets, player::Player, render::camera::Camera};

use super::{GuiPagesInfo, SideMenuPage};



pub fn render_sidemenu(d : &mut RaylibDrawHandle, gui_pages_info: &mut GuiPagesInfo, player: &mut Player, camera: &Camera, assets : &Assets) {

    let window_x: f32 = camera.screen_width as f32 * (2.0/3.0);
    let window_y: f32 = 5.0;
    let window_width: f32 = camera.screen_width as f32 * (1.0/3.0);
    let window_height: f32 = camera.screen_height as f32 - 10.0;

    let window_ratio = window_width / 100.0;

    d.draw_rectangle(window_x as i32, window_y as i32, window_width as i32, window_height as i32, Color::new(0, 0, 0, 128));

    if clickable_button(d, "Stats", window_x + 5.0, window_y + 5.0, 75.0, 25.0, 15.0, gui_pages_info.side_menu_page_open == SideMenuPage::Stats) {
        gui_pages_info.side_menu_page_open = SideMenuPage::Stats;
    }
    if clickable_button(d, "Crafting", window_x + 85.0, window_y + 5.0, 75.0, 25.0, 15.0, gui_pages_info.side_menu_page_open == SideMenuPage::Crafting) {
        gui_pages_info.side_menu_page_open = SideMenuPage::Crafting;
    }
    if clickable_button(d, "Info", window_x + 165.0, window_y + 5.0, 75.0, 25.0, 15.0, gui_pages_info.side_menu_page_open == SideMenuPage::Info) {
        gui_pages_info.side_menu_page_open = SideMenuPage::Info;
    }

    match gui_pages_info.side_menu_page_open {
        SideMenuPage::Stats => {
            d.draw_text("Player stats", (window_x + 5.0) as i32, (window_y + 35.0) as i32, 25, Color::WHITE);
        },
        SideMenuPage::Crafting => {

        },
        SideMenuPage::Info => {
            d.draw_text("Info", (window_x + 5.0) as i32, (window_y + 35.0) as i32, 25, Color::WHITE);
            d.draw_text("Kiwi Crawler pre release", (window_x + 5.0) as i32, (window_y as i32) + 60, 20, Color::WHITE);
            d.draw_text("W,A,S,D : Used for moving", (window_x + 5.0) as i32, (window_y as i32) + 80, 15, Color::WHITE);
            d.draw_text("I : open inventory", (window_x + 5.0) as i32, (window_y as i32) + 95, 15, Color::WHITE);
            d.draw_text("P : pickup items", (window_x + 5.0) as i32, (window_y as i32) + 110, 15, Color::WHITE);
            d.draw_text("TAB : swap current item between off hand and main", (window_x + 5.0) as i32, (window_y as i32) + 125, 15, Color::WHITE);
            d.draw_text("alt + scroll : zoom in/out", (window_x + 5.0) as i32, (window_y as i32) + 140, 15, Color::WHITE);
            d.draw_text("Scroll : change selected slot and scroll in inventory", (window_x + 5.0) as i32, (window_y as i32) + 155, 15, Color::WHITE);
            d.draw_text("Q : open or close this menu", (window_x + 5.0) as i32, (window_y as i32) + 170, 15, Color::WHITE);

        },
    }
}

pub fn clickable_button(d : &mut RaylibDrawHandle, text : &str, x : f32, y : f32, width : f32, height : f32, font_size : f32, selected : bool) -> bool {
    if selected {
        d.draw_rectangle(x as i32, y as i32, width as i32, height as i32, Color::new(75,75,75,255));
    }else{
        d.draw_rectangle(x as i32, y as i32, width as i32, height as i32, Color::new(25,25,25,255));
    }
    
    
    d.draw_rectangle_lines(x as i32, y as i32, width as i32, height as i32, Color::BLACK);

    let font_width = d.measure_text(text, font_size as i32);
    d.draw_text( text, ((x - (font_width as f32 /2.0)) + (width / 2.0)) as i32, ((y - (font_size / 2.0)) + (height / 2.0)) as i32, font_size as i32, Color::WHITE);

    let mouse_pos = d.get_mouse_position();

    if mouse_pos.x >= x && mouse_pos.y >= y && mouse_pos.x <= x + width && mouse_pos.y <= y + height {
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            return true;
        }else{
            return false;
        }
    }{
        return false;
    }
}