mod game_dimension;
mod utils;
mod player;
mod assets;
mod camera;
mod entities;
mod physics;
mod chunk_generation;
mod background_logic;
mod item;

use std::{cmp::Ordering, f32::consts::PI, time::SystemTime};

use assets::Assets;
use camera::Camera;
use entities::{environmental_object, monster::MonsterType};
use game_dimension::{Block, GameDimension};
use item::{Item, PremadeItem};
use player::Player;
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};
use physics::Position;
use raylib::prelude::Texture2D;

//cargo build --target x86_64-pc-windows-gnu --release

const GAME_TITLE: &str = "kiwi crawler pre release build";
pub const TPS: u128 = 60;// / 30;

struct RenderBufferItem<'a> {
    render_position : Position,
    render_asset : &'a Texture2D,
    real_position : Position,
    render_scale : f32,
    render_rotation : f32,
    health_bar_buffer_info: Option<HealthBarBufferInfo>,
    layer: u8,
}

struct HealthBarBufferInfo {
    health: f32,
    max_health: f32,
    x_pos: f32,
    y_pos: f32,
    level: u32,
    mob: MonsterType,
}


fn get_texture_from_item<'a>(item: &Item, assets: &'a Assets) -> &'a Texture2D {

    let texture = match item.item {
        PremadeItem::Stick => &assets.stick,
    };
    texture
}

fn render_item_slot(draw: &mut RaylibDrawHandle, assets: &Assets, x: f32, y: f32, item: &Option<item::Item>, count: &u32, selected : bool, slot_size: f32, item_padding: f32) {

    if selected {
        // Draw the inventory slot
        draw.draw_rectangle_lines_ex(
            raylib::core::math::Rectangle::new(
                x, 
                y, 
                slot_size, 
                slot_size
            ), 
            3.0, // Line thickness
            Color::WHITE
        );
        let rgba = Color::new(128, 128, 128, 128); // Example RGBA color
        draw.draw_rectangle(
            x as i32, 
            y as i32, 
            slot_size as i32, 
            slot_size as i32, 
            rgba
        );
    }else{
        // Draw the inventory slot
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
        let rgba = Color::new(0, 0, 0, 128); // Example RGBA color
        draw.draw_rectangle(
            x as i32, 
            y as i32, 
            slot_size as i32, 
            slot_size as i32, 
            rgba
        );
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
    let mut selected_hotbar_slot = 0;

    

    rl.set_target_fps(142);
    rl.set_exit_key(None);

    let mut inventory_scroll_offset = 0;
    let mut inventory_open = false;
    let mut selected_inventory_item: Option<Item> = None;

    let time_now = SystemTime::now();

    //main thread used for rendering
    while !rl.window_should_close() {

        //update tick stuff
        while time_now.elapsed().unwrap().as_millis() * 60 / 1000 > game_dimension.tick_number {
            player.handle_movement();
            game_dimension.tick_physics();
            game_dimension.tick_chunk_loading(&mut player);
            game_dimension.background_tick(&mut player);
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
        let player_facing = {
            let dx = mouse_position.x - (camera.screen_width as f32 / 2.0);
            let dy = (mouse_position.y - (camera.screen_height as f32 / 2.0 )) * -1.0; //fip final value as mouse vertical is flipped
            let angle_rad = dy.atan2(dx); // angle in radians
            let mut angle_deg = angle_rad * 180.0 / PI; // convert to degrees
            if angle_deg < 0.0 {
                angle_deg += 360.0; // adjust negative angles to positive
            }
            angle_deg
        };
        //handle facing char
        if mouse_down {
            player.interact_direction = player_facing;

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
            if inventory_open == true {
                inventory_scroll_offset += scroll as i32 * -1;
                if inventory_scroll_offset < 0 {
                    inventory_scroll_offset = 0;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_I) {
                    inventory_open = false;
                }
            }else{
                selected_hotbar_slot += scroll as i32 * -1;
                if selected_hotbar_slot < 0 {
                    selected_hotbar_slot = 8;
                }
                if selected_hotbar_slot > 8 {
                    selected_hotbar_slot = 0;
                }
                if scroll != 0.0 {
                    let new_hotbar_slot = player.hotbar[selected_hotbar_slot as usize].clone();
                    player.main_hand = new_hotbar_slot;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    let old_offhand_item = player.off_hand.clone();
                    let old_mainhand_item = player.main_hand.clone();
                    player.main_hand = old_offhand_item;
                    player.off_hand = old_mainhand_item;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_I) {
                    inventory_open = true;
                    inventory_scroll_offset = 0;
                }
            }
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
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::PURPLE);


        //draw the map
        {
            let mut render_buffer: Vec<RenderBufferItem> = Vec::new();

            //calculate what is in range of camera
            let left_screen_chunk: i32 = ((camera.position.x - ((camera.screen_width / 2) as f32) * camera.zoom) / 16.0 ).floor() as i32;
            let bottom_screen_chunk: i32 = ((camera.position.y - ((camera.screen_height / 2) as f32) * camera.zoom) / 16.0 ).floor() as i32;
            let right_screen_chunk: i32 = ((camera.position.x + ((camera.screen_width / 2) as f32) * camera.zoom + 1.0) / 16.0 ).ceil() as i32; //not sure why just seemed to need 1 more
            let top_screen_chunk: i32 = ((camera.position.y + ((camera.screen_height / 2) as f32) * camera.zoom + 1.0) / 16.0 ).ceil() as i32;
            
            for chunk_x in left_screen_chunk..right_screen_chunk {
                for chunk_y in bottom_screen_chunk..top_screen_chunk {
                    if let Some(chunk) = game_dimension.chunks.get(&(chunk_x, chunk_y)) {
                        let chunk_data = chunk;
                        let block_data = chunk_data.block_data.clone();
                        let environmental_objects = &chunk_data.environmental_objects;
                    
                        let mut i=0;
                    
                        for y in (0+(chunk_y*16))..(16+(chunk_y*16)){
                            for x in (0+(chunk_x*16))..(16+(chunk_x*16)){
                                let block = &block_data[i];
                            
                                let texture = match block {
                                    Block::Dirt => &assets.dirt,
                                    Block::Cobblestone => &assets.stone,
                                    Block::Unloaded => &assets.invalid,
                                    Block::DirtWithPebble => &assets.dirt_with_pebble,
                                    Block::Sand => &assets.sand,
                                    Block::Grass => &assets.grass,
                                    Block::SeasonalGrass => &assets.seasonal_grass,
                                    Block::DarkGrass => &assets.dark_grass,
                                    Block::Water => &assets.water,
                                    Block::SwampGrass => &assets.swamp_grass,
                                    Block::SwampWater => &assets.swamp_water,
                                    Block::SnowyGrass => &assets.snowy_grass,
                                };
                                                            
                                render_buffer.push(RenderBufferItem{
                                    render_position: Position{
                                        x: camera.convert_x_pos_to_screen(&(x as f32 + 0.5), &1.0, 0.0),
                                        y: camera.convert_y_pos_to_screen(&(y as f32 + 0.5), &1.0, 0.0),
                                    },
                                    render_asset: texture,
                                    real_position: Position{
                                        x: x as f32 + 0.5,
                                        y: y as f32 + 0.5,
                                    },
                                    render_scale: 1.0 / camera.zoom / 16.0,
                                    render_rotation: 0.0,
                                    health_bar_buffer_info: None,
                                    layer: 1,
                                });
                                i = i + 1;
                            }
                        }
                    
                        let environmental_object_iter = environmental_objects.iter();
                    
                        for item in environmental_object_iter {
                            match item.object_type {
                                environmental_object::EnvironmentalObjectType::DeadTree => {
                                    render_buffer.push(RenderBufferItem{
                                        render_position: Position{
                                            x: camera.convert_x_pos_to_screen(&item.position.x,&1.0, 0.0),
                                            y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&1.0, 0.0),
                                        },
                                        render_asset: &assets.dead_tree,
                                        real_position: Position{
                                            x: item.position.x,
                                            y: item.position.y,
                                        },
                                        render_scale: 1.0 / camera.zoom / 16.0,
                                        render_rotation: 0.0,
                                        health_bar_buffer_info: None,
                                        layer: 5,
                                    });
                                },
                                environmental_object::EnvironmentalObjectType::Tree => {
                                    render_buffer.push(RenderBufferItem{
                                        render_position: Position{
                                            x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                                            y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                                        },
                                        render_asset: &assets.tree,
                                        real_position: Position{
                                            x: item.position.x,
                                            y: item.position.y,
                                        },
                                        render_scale: 1.0 / camera.zoom / 16.0,
                                        render_rotation: 0.0,
                                        health_bar_buffer_info: None,
                                        layer: 5,
                                    });
                                },
                                environmental_object::EnvironmentalObjectType::AppleTree => {
                                    render_buffer.push(RenderBufferItem{
                                        render_position: Position{
                                            x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                                            y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                                        },
                                        render_asset: &assets.apple_tree,
                                        real_position: Position{
                                            x: item.position.x,
                                            y: item.position.y,
                                        },
                                        render_scale: 1.0 / camera.zoom / 16.0,
                                        render_rotation: 0.0,
                                        health_bar_buffer_info: None,
                                        layer: 5,
                                    });
                                },
                                environmental_object::EnvironmentalObjectType::SwampTree => {
                                    render_buffer.push(RenderBufferItem{
                                        render_position: Position{
                                            x: camera.convert_x_pos_to_screen(&item.position.x,&2.0, 0.0),
                                            y: camera.convert_y_pos_to_screen(&(&item.position.y + 1.0),&2.0, 0.0),
                                        },
                                        render_asset: &assets.swamp_tree,
                                        real_position: Position{
                                            x: item.position.x,
                                            y: item.position.y,
                                        },
                                        render_scale: 1.0 / camera.zoom / 16.0,
                                        render_rotation: 0.0,
                                        health_bar_buffer_info: None,
                                        layer: 5,
                                    });
                                },
                                environmental_object::EnvironmentalObjectType::Bush => {
                                    render_buffer.push(RenderBufferItem{
                                        render_position: Position{
                                            x: camera.convert_x_pos_to_screen(&item.position.x,&1.0, 0.0),
                                            y: camera.convert_y_pos_to_screen(&&item.position.y,&1.0, 0.0),
                                        },
                                        render_asset: &assets.bush,
                                        real_position: Position{
                                            x: item.position.x,
                                            y: item.position.y,
                                        },
                                        render_scale: 1.0 / camera.zoom / 16.0,
                                        render_rotation: 0.0,
                                        health_bar_buffer_info: None,
                                        layer: 5,
                                    });
                                },
                            }
                        }

                        let monsters_objects = &chunk_data.monsters;


                        let monsters_object_iter = monsters_objects.iter();

                        for monster in monsters_object_iter {
                            
                            let monster_data = monster;
                            let x_pos = monster_data.movement.position.x;
                            let y_pos = monster_data.movement.position.y;
                                
                            render_buffer.push(RenderBufferItem{
                                render_position: Position{
                                    x: camera.convert_x_pos_to_screen(&x_pos,&1.0, 0.0),
                                    y: camera.convert_y_pos_to_screen(&y_pos,&1.0, 0.0),
                                },
                                render_asset: &assets.ghost,
                                real_position: Position{
                                    x: x_pos,
                                    y: y_pos,
                                },
                                render_scale: 1.0 / camera.zoom / 16.0,
                                render_rotation: 0.0,
                                health_bar_buffer_info: {
                                    Some(HealthBarBufferInfo{
                                        health: monster_data.health,
                                        max_health: monster_data.max_health,
                                        x_pos: camera.convert_x_pos_to_screen(&x_pos,&0.0, 0.0),
                                        y_pos: camera.convert_y_pos_to_screen(&y_pos,&0.0, 0.0),
                                        level: monster_data.level,
                                        mob: monster_data.mob_type,
                                    })
                                },
                                layer: 10,
                            });
                        }

                        //render items

                        for dropped_item in &chunk.dropped_items {
                            let texture = get_texture_from_item(&dropped_item.item, &assets);
                            render_buffer.push(RenderBufferItem{
                                render_position: Position{
                                    x: camera.convert_x_pos_to_screen(&dropped_item.position.x, &1.0, 0.0),
                                    y: camera.convert_y_pos_to_screen(&dropped_item.position.y, &1.0, 0.0),
                                },
                                render_asset: texture,
                                real_position: Position{
                                    x: dropped_item.position.x,
                                    y: dropped_item.position.y,
                                },
                                render_scale: 1.0 / camera.zoom / 16.0,
                                render_rotation: 0.0,
                                health_bar_buffer_info: None,
                                layer: 3,
                            });
                        }





                    
                    } else {
                        // Handle the case where the chunk is not found
                        //println!("Chunk not found at ({}, {})", chunk_x, chunk_y);
                    }
                
                }
            }

            //render player 

            render_buffer.push(RenderBufferItem{
                render_position: Position{
                    x: camera.convert_x_pos_to_screen(&player_position.x,&1.0, 0.0),
                    y: camera.convert_y_pos_to_screen(&player_position.y,&1.0, 0.0),
                },
                render_asset: &assets.player,
                real_position: Position{
                    x: player_position.x,
                    y: player_position.y,
                },
                render_scale: 1.0 / camera.zoom / 16.0,
                render_rotation: 0.0,
                health_bar_buffer_info: None,
                layer: 5,
            });

            //draw item holding
            let item_holding_mainhand = player.main_hand.clone();
            let item_holding_offhand = player.off_hand.clone();
            if let Some(item) = item_holding_mainhand {
                let texture = get_texture_from_item(&item, &assets);
                render_buffer.push(RenderBufferItem{
                    render_position: Position{
                        x: camera.convert_x_pos_to_screen(&(player_position.x + player_facing.to_radians().cos() * 0.5), &1.0, player_facing),
                        y: camera.convert_y_pos_to_screen(&(player_position.y + player_facing.to_radians().sin() * 0.5), &1.0, player_facing),
                    },
                    render_asset: texture,
                    real_position: Position{
                        x: player_position.x,
                        y: player_position.y,
                    },
                    render_scale: 1.0 / camera.zoom / 16.0,
                    render_rotation: player_facing,
                    health_bar_buffer_info: None,
                    layer: 5,
                });
            }


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
                d.draw_texture_ex(item.render_asset, Vector2::new(item.render_position.x, item.render_position.y), item.render_rotation, item.render_scale, Color::WHITE);
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
                        d.draw_text(&format!("Level : {level}"), (bar_info.x_pos - 50.0).floor() as i32, (bar_info.y_pos - 30.0).floor() as i32, 12, Color::WHITE);
                        d.draw_text(&format!("Level : {name}"), (bar_info.x_pos - 60.0).floor() as i32, (bar_info.y_pos - 30.0).floor() as i32, 12, Color::WHITE);


                    },
                    None => (),
                }

            }
        }

        //draw the ui
        {
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
                let selected = index == selected_hotbar_slot.try_into().unwrap();
                //if amount is 0 delete item from hotbar
                if let Some(_) = item {
                    if hotbar_item_counts[index] == 0 {
                        player.hotbar[index] = None;
                    }
                }
            
                render_item_slot(&mut d, &assets, render_x, render_y, &item, &hotbar_item_counts[index], selected, slot_size, padding);
            
                if mouse_down {
                    if mouse_position.x <= padding + (index as f32 * (slot_size + padding)) + slot_size && mouse_position.x >= padding + (index as f32 * (slot_size + padding)) && mouse_position.y <= camera.screen_height as f32 - padding && mouse_position.y >= camera.screen_height as f32 - slot_size - padding {
                        player.hotbar[index] = selected_inventory_item.clone();
                    }
                }
            }

            //render inventory assuming opened right now, it will just be hotbar but but repeating for all items
            //it will also be scrollable and only take up a portion of the screen
            //render the inventory
            if inventory_open == true {
            
                let inventory = player.inventory.clone();
                let inventory_slot_size = 64.0;
                let inventory_padding = 5.0;
                let inventory_slots_per_row = 10;
                let mut inventory_slot_index = 0;
                for item in inventory.iter().skip((inventory_scroll_offset * inventory_slots_per_row) as usize) {
                    let row = inventory_slot_index / inventory_slots_per_row;
                    let col = inventory_slot_index % inventory_slots_per_row;
                
                    let render_x = inventory_padding + (col as f32 * (inventory_slot_size + inventory_padding));
                    let render_y = inventory_padding + (row as f32 * (inventory_slot_size + inventory_padding));
                
                    if render_y > camera.screen_height as f32 - 100.0 {
                        break;
                    }
                
                    if mouse_position.x <= render_x + inventory_slot_size && mouse_position.x >= render_x && mouse_position.y <= render_y + inventory_slot_size && mouse_position.y >= render_y {
                        if mouse_down {
                            selected_inventory_item = Some(item.0.clone());
                        }
                    }
                    let selected = selected_inventory_item.as_ref() == Some(item.0);
                    render_item_slot(&mut d, &assets, render_x, render_y, &Some(item.0.clone()), item.1, selected, inventory_slot_size, item_padding);
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
            if main_hand_amount == 0 {
                player.main_hand = None;
            }
            if offhand_amount == 0 {
                player.off_hand = None;
            }
            render_item_slot(&mut d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69 - 64 - 5) as f32, &main_hand, &main_hand_amount, false, 64.0, item_padding);
            render_item_slot(&mut d, &assets, (camera.screen_width - 69) as f32, (camera.screen_height - 69) as f32, &off_hand, &offhand_amount, false, 64.0, item_padding);
            //make sure they still have the item in their inventory, otherwise remove it















            d.draw_text(GAME_TITLE, 5, 5, 15, Color::WHITE);
            d.draw_text(&(fps.to_string().as_str()), 5, 20, 15, Color::WHITE);
            d.draw_text(&(player.movement.position.x.to_string().as_str()), 5, 35, 15, Color::WHITE);
            d.draw_text(&(player.movement.position.y.to_string().as_str()), 5, 50, 15, Color::WHITE);

            d.draw_text("W,A,S,D : Used for moving", 5, 80, 15, Color::WHITE);
            d.draw_text("I : open inventory", 5, 95, 15, Color::WHITE);
            d.draw_text("P : pickup items", 5, 110, 15, Color::WHITE);
            d.draw_text("TAB : swap current item between off hand and main", 5, 125, 15, Color::WHITE);
            d.draw_text("alt + scroll : zoom in/out", 5, 140, 15, Color::WHITE);
            d.draw_text("Scroll : change selected slot and scroll in inventory", 5, 155, 15, Color::WHITE);

        }
        



    }

}






