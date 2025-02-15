use raylib::{color::Color, ffi::Rectangle, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::{assets::Assets, game_dimension::GameDimension, item, player::Player, render::{camera::Camera, render_utils::{clickable_item_slot, ClickableItemSlotAction}, MouseInfo, RenderBufferItem}};

use super::GuiPagesInfo;


pub fn render_hotbar(render_buffer : &mut Vec<RenderBufferItem>, player : &mut Player, game_dimension : &GameDimension, camera : &Camera, d: &mut RaylibDrawHandle, assets : &Assets, mouse_info: &MouseInfo, gui_pages_info : &mut GuiPagesInfo) {
    let _ = mouse_info;
    let _ = game_dimension;
    let _ = render_buffer;
    //render player hotbar
    let hotbar:[Option<item::Item>; 9]  = player.hotbar.clone();
    //get item counts
    let hotbar_item_counts = {
        let player_inventory = player.inventory.clone();
        hotbar.iter().map(|item| {
            match item {
                Some(item) => {
                    let count = match player_inventory.get(item) {
                        Some(count) => {
                            *count
                        },
                        None => {
                            0
                        },
                    };
                    count
                },
                None => {
                    0
                },
            }
        }).collect::<Vec<u32>>()
    };

    const SLOT_SIZE: f32 = 64.0;
    const PADDING: f32 = 5.0;
    const ITEM_PADDING: f32 = 8.0; // Padding for the item within the slot
    for (index, item) in hotbar.iter().enumerate() {
        let render_x = PADDING + (index as f32 * (SLOT_SIZE + PADDING)) + ITEM_PADDING;
        let render_y = camera.screen_height as f32 - SLOT_SIZE - PADDING + ITEM_PADDING;
        let selected = index == gui_pages_info.hotbar_selected_slot.try_into().unwrap();
        //if amount is 0 delete item from hotbar
        if let Some(_) = item {
            if hotbar_item_counts[index] == 0 {
                player.hotbar[index] = None;
            }
        }
        match &gui_pages_info.inventory_selected_item {
            Some(_) => {
                let safe_item = gui_pages_info.inventory_selected_item.as_ref().unwrap().clone();
                clickable_item_slot(d, assets, render_x, render_y, &item, &hotbar_item_counts[index], selected, SLOT_SIZE, PADDING, ClickableItemSlotAction::ReplaceHotbarItem(index as i32, &safe_item), gui_pages_info, player);
            },
            None => {
                clickable_item_slot(d, assets, render_x, render_y, &item, &hotbar_item_counts[index], selected, SLOT_SIZE, PADDING, ClickableItemSlotAction::None, gui_pages_info, player);

            },
        }
    }

    //render main hand and off hand
    let main_hand = player.main_hand.clone();
    let off_hand = player.off_hand.clone();
    let main_hand_amount = match &main_hand {
        Some(item) => {
            let count = match player.inventory.get(&item) {
                Some(count) => {
                    *count
                },
                None => {
                    0
                },
            };
            count
        },
        None => {
            0
        },
    };
    let offhand_amount = match &off_hand {
        Some(item) => {
            let count = match player.inventory.get(&item) {
                Some(count) => {
                    *count
                },
                None => {
                    0
                },
            };
            count
        },
        None => {
            0
        },
    };
    //make sure they have one of they item and they aren't holding nothing, honestly not a great place for it to be in rendering code, but more performant
    if main_hand_amount == 0 {
        player.main_hand = None;
    }
    if offhand_amount == 0 {
        player.off_hand = None;
    }
    clickable_item_slot(d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69 - 64 - 5) as f32, &main_hand, &main_hand_amount, false, 64.0, ITEM_PADDING, ClickableItemSlotAction::None, gui_pages_info, player);
    clickable_item_slot(d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69) as f32, &off_hand, &offhand_amount, false, 64.0, ITEM_PADDING, ClickableItemSlotAction::None, gui_pages_info, player);
    //make sure they still have the item in their inventory, otherwise remove it
    
    //draw bars
    draw_meter(d,8.0, (camera.screen_height - 80) as f32, 500.0, 15.0, (player.health / player.stats.max_health), Color::RED);
    if player.attack_cooldown.1 > game_dimension.tick_number {
        draw_meter(d,8.0, (camera.screen_height - 100) as f32, 500.0, 15.0, (game_dimension.tick_number - player.attack_cooldown.0) as f32 / (player.attack_cooldown.1 - player.attack_cooldown.0) as f32, Color::WHITE);
    }
    draw_meter(d,8.0, 8.0 as f32, 500.0, 15.0, player.xp as f32 / player.xp_to_level_up as f32, Color::LIGHTBLUE);
    d.draw_text(&format!("Level : {}", player.level), 8, 25, 25, Color::BLACK);

}


pub fn draw_meter(d: &mut RaylibDrawHandle,x: f32,y: f32,width: f32,height: f32,full_percent:f32,color: Color) {
    d.draw_rectangle_rounded( Rectangle {
        x: x,
        y: y as f32,
        width: 500.0,
        height: 15.0,
    },
    15.0,
    0,
    Color::BLACK);
    d.draw_rectangle_rounded( Rectangle {
        x: x + (height / 6.0),
        y: y + (height / 6.0),
        width: (width - (height / 3.0)) * full_percent,
        height: 15.0 - 5.0,
    },
    15.0,
    0,
    color);

}