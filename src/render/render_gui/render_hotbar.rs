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

    let slot_size = 64.0;
    let padding = 5.0;
    let item_padding = 8.0; // Padding for the item within the slot
    for (index, item) in hotbar.iter().enumerate() {
        let render_x = padding + (index as f32 * (slot_size + padding)) + item_padding;
        let render_y = camera.screen_height as f32 - slot_size - padding + item_padding;
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
                clickable_item_slot(d, assets, render_x, render_y, &item, &hotbar_item_counts[index], selected, slot_size, padding, ClickableItemSlotAction::ReplaceHotbarItem(index as i32, &safe_item), gui_pages_info, player);
            },
            None => {
                clickable_item_slot(d, assets, render_x, render_y, &item, &hotbar_item_counts[index], selected, slot_size, padding, ClickableItemSlotAction::None, gui_pages_info, player);

            },
        }
    }

    //render inventory assuming opened right now, it will just be hotbar but but repeating for all items
    //it will also be scrollable and only take up a portion of the screen
    //render the inventory
    if gui_pages_info.inventory_open == true {
            
        let inventory = player.inventory.clone();
        let inventory_slot_size = 64.0;
        let inventory_padding = 5.0;
        let inventory_slots_per_row = 10;
        let mut inventory_slot_index = 0;
        for item in inventory.iter().skip((gui_pages_info.inventory_scroll_offset * inventory_slots_per_row) as usize) {
            let row = inventory_slot_index / inventory_slots_per_row;
            let col = inventory_slot_index % inventory_slots_per_row;
        
            let render_x = inventory_padding + (col as f32 * (inventory_slot_size + inventory_padding));
            let render_y = inventory_padding + (row as f32 * (inventory_slot_size + inventory_padding));
        
            if render_y > camera.screen_height as f32 - 100.0 {
                break;
            }
            
            let selected = gui_pages_info.inventory_selected_item.as_ref() == Some(item.0);
            clickable_item_slot(d, &assets, render_x, render_y, &Some(item.0.clone()), item.1, selected, inventory_slot_size, item_padding, ClickableItemSlotAction::SelectInventoryItem(inventory_slot_index, item.0), gui_pages_info, player);
            inventory_slot_index += 1;
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
    clickable_item_slot(d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69 - 64 - 5) as f32, &main_hand, &main_hand_amount, false, 64.0, item_padding, ClickableItemSlotAction::None, gui_pages_info, player);
    clickable_item_slot(d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69) as f32, &off_hand, &offhand_amount, false, 64.0, item_padding, ClickableItemSlotAction::None, gui_pages_info, player);
    //make sure they still have the item in their inventory, otherwise remove it

    //draw player health bar
    d.draw_rectangle_rounded( Rectangle {
        x: 8.0 + 5.0,
        y: (camera.screen_height - 80) as f32,
        width: 500.0,
        height: 15.0,
    },
    15.0,
    0,
    Color::BLACK);
    d.draw_rectangle_rounded( Rectangle {
        x: 8.0 + 5.0 + 2.5,
        y: (camera.screen_height - 80) as f32 + 2.5,
        width: (500.0 - 5.0) * (player.health / player.max_health),
        height: 15.0 - 5.0,
    },
    15.0,
    0,
    Color::RED);

}
