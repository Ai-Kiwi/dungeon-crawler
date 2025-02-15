mod game_dimension;
mod utils;
mod player;
mod assets;
mod entities;
mod physics;
mod chunk_generation;
mod background_logic;
mod item;
mod render;

use std::time::SystemTime;
use render::{camera::Camera, main_render, render_gui::GuiPagesInfo};
use assets::Assets;
use game_dimension::GameDimension;
use player::Player;
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::RaylibDraw};

//cargo build --target x86_64-pc-windows-gnu --release

const GAME_TITLE: &str = "kiwi crawler pre release build";
pub const RENDER_DISTANCE: i32 = 32;
pub const TPS: u128 = 60;// / 30;




fn main() {
    let mut game_dimension = GameDimension::new();
    let mut player = Player::new();
    let mut camera = Camera::new();
    let (mut rl, thread) = raylib::init()
    .size(camera.screen_width as i32, camera.screen_height as i32)
    .title(GAME_TITLE)
    //.fullscreen()
    .resizable()
    .build();
    let assets = Assets::load(&mut rl, &thread);

    

    rl.set_target_fps(142);
    rl.set_exit_key(None);

    let mut gui_info = GuiPagesInfo::new();
    let time_now = SystemTime::now();

    //main thread used for rendering
    while !rl.window_should_close() {

        //update tick stuff
        while time_now.elapsed().unwrap().as_millis() * 60 / 1000 > game_dimension.tick_number {
            player.tick();
            player.handle_movement();
            game_dimension.tick_chunk_loading(&mut player);
            game_dimension.background_tick(&mut player);
            game_dimension.tick_physics();
            game_dimension.tick_number = game_dimension.tick_number + 1;
        }
        //later threads to possibly add
        //handle game ticks
        //handle path finding
        //particle system/effects
        //game load/save system
        //npc thread
        //}



        //movement stuff
        let player_position = player.movement.position.clone();
        {
            if rl.is_key_down(KeyboardKey::KEY_W) {
                player.walk_dir.1 = 1;
            }else if rl.is_key_down(KeyboardKey::KEY_S) {
                player.walk_dir.1 = -1;
            }else{
                player.walk_dir.1 = 0;
            }
            
            if rl.is_key_down(KeyboardKey::KEY_A) {
                player.walk_dir.0 = -1;
            }else if rl.is_key_down(KeyboardKey::KEY_D) {
                player.walk_dir.0 = 1;
            }else{
                player.walk_dir.0 = 0;
            }
        }

        let mouse_position = rl.get_mouse_position();
        let mouse_down = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        player.facing = {
            let dx =  mouse_position.x - (camera.screen_width as f32 / 2.0);
            let dy = (mouse_position.y - (camera.screen_height as f32 / 2.0 )) * -1.0; //fip final value as mouse vertical is flipped
            let mut angle_deg = -dy.atan2(dx).to_degrees();
            if angle_deg < 0.0 {
                angle_deg += 360.0; // adjust negative angles to positive
            }
            angle_deg
        };
        //handle facing char
        if mouse_down {
            player.right_hand_attack(&mut game_dimension);
        }

        //handle pickup items
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            player.pickup_items(&mut game_dimension);
        }

        let scroll = rl.get_mouse_wheel_move();
        if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            if scroll  > 0.0 {
                camera.base_zoom = camera.base_zoom * 0.9; 
            }else if scroll < 0.0 {
                camera.base_zoom = camera.base_zoom * 1.1;
            }
        }else{
            if gui_info.inventory_open == true {
                gui_info.inventory_scroll_offset += scroll as i32 * -1;
                if gui_info.inventory_scroll_offset < 0 { gui_info.inventory_scroll_offset = 0;}
            }else{
                gui_info.hotbar_selected_slot += scroll as i32 * -1;
                if gui_info.hotbar_selected_slot < 0 { gui_info.hotbar_selected_slot = 8;}
                if gui_info.hotbar_selected_slot > 8 {gui_info.hotbar_selected_slot = 0; }
                if scroll != 0.0 {
                    let new_hotbar_slot = player.hotbar[gui_info.hotbar_selected_slot as usize].clone();
                    player.main_hand = new_hotbar_slot;
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            let old_offhand_item = player.off_hand.clone();
            let old_mainhand_item = player.main_hand.clone();
            player.main_hand = old_offhand_item;
            player.off_hand = old_mainhand_item;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            gui_info.inventory_open = !gui_info.inventory_open;
            gui_info.inventory_scroll_offset = 0;
        }
        
        //make camera follow player
        camera.screen_height = rl.get_screen_height() as u32;
        camera.screen_width = rl.get_screen_width() as u32;
        camera.position.x = player_position.x;
        camera.position.y = player_position.y;
        //camera.zoom =  32.0 / (camera.screen_height as f32);
        camera.zoom =  (16.0 / (camera.screen_height as f32)) * camera.base_zoom;
        
        //////////////////////
        //  render the game //
        //////////////////////

        let mut d = rl.begin_drawing(&thread);

        main_render(&mut d, &game_dimension, &mut player, &camera, &assets, &mut gui_info);
    

        //draw the ui
        {
            d.draw_text("W,A,S,D : Used for moving", 5, 80, 15, Color::PURPLE);
            d.draw_text("I : open inventory", 5, 95, 15, Color::PURPLE);
            d.draw_text("P : pickup items", 5, 110, 15, Color::PURPLE);
            d.draw_text("TAB : swap current item between off hand and main", 5, 125, 15, Color::PURPLE);
            d.draw_text("alt + scroll : zoom in/out", 5, 140, 15, Color::PURPLE);
            d.draw_text("Scroll : change selected slot and scroll in inventory", 5, 155, 15, Color::PURPLE);
        }
        



    }

}