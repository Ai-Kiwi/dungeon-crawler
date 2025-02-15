
use raylib::prelude::RaylibDrawHandle;

use super::GuiPagesInfo;
use crate::{assets::Assets, player::Player, render::{camera::Camera, render_utils::{clickable_item_slot, ClickableItemSlotAction}}};

pub fn render_inventory(gui_pages_info: &mut GuiPagesInfo, player: &mut Player, camera: &Camera, assets : &Assets, d : &mut RaylibDrawHandle) {
    if gui_pages_info.inventory_open == true {
    
        
        let inventory = player.inventory.clone();
        const INVENTORY_SLOT_SIZE: f32 = 64.0;
        const INVENTORY_PADDING: f32 = 5.0;
        const INVENTORY_SLOTS_PER_ROW: i32 = 10;
        let mut inventory_slot_index = 0;
        for item in inventory.iter().skip((gui_pages_info.inventory_scroll_offset * INVENTORY_SLOTS_PER_ROW) as usize) {
            let row = inventory_slot_index / INVENTORY_SLOTS_PER_ROW;
            let col = inventory_slot_index % INVENTORY_SLOTS_PER_ROW;
        
            let render_x = INVENTORY_PADDING + (col as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_PADDING));
            let render_y = INVENTORY_PADDING + (row as f32 * (INVENTORY_SLOT_SIZE + INVENTORY_PADDING));
        
            if render_y > camera.screen_height as f32 - 100.0 {
                break;
            }
            
            let selected = gui_pages_info.inventory_selected_item.as_ref() == Some(item.0);
            clickable_item_slot(d, &assets, render_x, render_y, &Some(item.0.clone()), item.1, selected, INVENTORY_SLOT_SIZE, INVENTORY_PADDING, ClickableItemSlotAction::SelectInventoryItem(inventory_slot_index, item.0), gui_pages_info, player);
            inventory_slot_index += 1;
        }
    }
}

