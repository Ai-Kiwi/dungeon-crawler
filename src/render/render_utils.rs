use raylib::{color::Color, ffi::MouseButton, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};

use crate::{assets::Assets, item::{self, Item, PremadeItem}, player::Player};

use super::render_gui::GuiPagesInfo;

pub fn get_texture_from_item<'a>(item: &Item, assets: &'a Assets) -> &'a Texture2D {

    let texture: &raylib::prelude::Texture2D = match item.item {
        PremadeItem::Stick => &assets.stick,
    };
    texture
}

pub enum ClickableItemSlotAction<'a> {
    SelectInventoryItem(i32,&'a Item),
    SelectHotbarItem,
    ReplaceHotbarItem(i32,&'a Item),
    None
}

pub fn clickable_item_slot(draw: &mut RaylibDrawHandle, assets: &Assets, x: f32, y: f32, item: &Option<item::Item>, count: &u32, selected : bool, slot_size: f32, item_padding: f32, on_click: ClickableItemSlotAction, gui_pages_info : &mut GuiPagesInfo, player : &mut Player) {

    let rgba = if selected {
        Color::new(128, 128, 128, 128) // Example RGBA color
    }else{
        Color::new(0, 0, 0, 128) // Example RGBA color
    };
    draw.draw_rectangle_lines_ex(
        raylib::core::math::Rectangle::new(
            x, 
            y, 
            slot_size, 
            slot_size
        ), 
        3.0, // Line thickness
        Color::BLACK
    );
    if clickable_rectangle(draw, x, y, slot_size,  slot_size, rgba) {
        match on_click {
            ClickableItemSlotAction::SelectInventoryItem(slot_number, item) => {
                gui_pages_info.inventory_selected_slot = slot_number;
                gui_pages_info.inventory_selected_item = Some(item.clone());
            },
            ClickableItemSlotAction::ReplaceHotbarItem(slot, item) => {
                player.hotbar[slot as usize] = Some(item.clone());
                if gui_pages_info.hotbar_selected_slot == slot {
                    player.main_hand = Some(item.clone());
                }
            },
            ClickableItemSlotAction::SelectHotbarItem => {

            },
            ClickableItemSlotAction::None => (),
        }
    }
    
    // Draw the item in the slot
    if let Some(item_value) = item {
        let item_type = &item_value.item;
        if item_type == &PremadeItem::Stick{
            draw.draw_texture_ex(&assets.stick, Vector2::new(x + item_padding, y + item_padding), 0.0, (slot_size - 2.0 * item_padding) / 16.0, Color::WHITE);
        }else{
            draw.draw_texture_ex(&assets.invalid, Vector2::new(x + item_padding, y + item_padding), 0.0, (slot_size - 2.0 * item_padding) / 16.0, Color::WHITE);
        }
        draw.draw_text(&(count.to_string().as_str()), (x + (slot_size * 0.15) - 5.0) as i32, (y + (slot_size * 0.65)) as i32, (slot_size * 0.35) as i32, Color::WHITE);
    }

}

pub fn clickable_rectangle(d: &mut RaylibDrawHandle, x : f32,y : f32,width : f32,height : f32, color : Color) -> bool {
    let mouse_position = d.get_mouse_position();
    let mouse_down = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    d.draw_rectangle(x as i32, y as i32, width as i32, height as i32, color);

    if mouse_position.x <= x + width && mouse_position.x >= x && mouse_position.y <= y + height && mouse_position.y >= y {
        if mouse_down {
            return true;
        }
    }
    return false;
}