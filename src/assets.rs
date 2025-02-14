use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::Texture2D;

pub struct Assets {
    pub player : Texture2D,
    pub invalid : Texture2D,
    pub dirt : Texture2D,
    pub stone : Texture2D,
    pub dirt_with_pebble : Texture2D,
    pub ghost : Texture2D,
    pub grass : Texture2D,
    pub sand : Texture2D,
    pub seasonal_grass: Texture2D,
    pub dark_grass: Texture2D,
    pub water: Texture2D,
    pub swamp_grass: Texture2D,
    pub swamp_water: Texture2D,
    pub snowy_grass: Texture2D,
    pub dead_tree: Texture2D,
    pub tree: Texture2D,
    pub apple_tree: Texture2D,
    pub bush: Texture2D,
    pub swamp_tree: Texture2D,
    pub stick: Texture2D,
    pub jungle_tree: Texture2D,
}

const IMAGE_BYTES: &[u8] = include_bytes!("../assets/apple_tree.png");



impl Assets {
    pub fn load(rl: &mut RaylibHandle, thread : &RaylibThread) -> Self {
        Assets {
            player: rl.load_texture(&thread, "assets/player.png").unwrap(),
            invalid: rl.load_texture(&thread, "assets/not_loaded.png").unwrap(),
            dirt: rl.load_texture(&thread, "assets/dirt.png").unwrap(),
            stone: rl.load_texture(&thread, "assets/stone.png").unwrap(),
            dirt_with_pebble: rl.load_texture(&thread, "assets/dirt_pebble.png").unwrap(),
            ghost: rl.load_texture(&thread, "assets/ghost.png").unwrap(),
            grass: rl.load_texture(&thread, "assets/grass.png").unwrap(),
            sand: rl.load_texture(&thread, "assets/sand.png").unwrap(),
            seasonal_grass: rl.load_texture(&thread, "assets/seasonal_grass.png").unwrap(),
            dark_grass: rl.load_texture(&thread, "assets/dark_grass.png").unwrap(),
            water: rl.load_texture(&thread, "assets/water.png").unwrap(),
            swamp_grass: rl.load_texture(&thread, "assets/swamp_grass.png").unwrap(),
            swamp_water: rl.load_texture(&thread, "assets/swamp_water.png").unwrap(),
            snowy_grass: rl.load_texture(&thread, "assets/snowy_grass.png").unwrap(),
            dead_tree: rl.load_texture(&thread, "assets/dead_tree.png").unwrap(),
            tree: rl.load_texture(&thread, "assets/tree.png").unwrap(),
            apple_tree: rl.load_texture(&thread, "assets/apple_tree.png").unwrap(),
            bush: rl.load_texture(&thread, "assets/bush.png").unwrap(),
            swamp_tree: rl.load_texture(&thread, "assets/swamp_tree.png").unwrap(),
            stick: rl.load_texture(&thread, "assets/stick.png").unwrap(),
            jungle_tree: rl.load_texture(&thread, "assets/jungle_tree.png").unwrap(),

        }
    }
}