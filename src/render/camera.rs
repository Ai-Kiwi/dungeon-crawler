use trig::Trig;

use crate::physics::{Position, Velocity};

pub struct Camera {
    pub zoom : f32,
    pub base_zoom : f32,
    pub screen_width: u32,
    pub screen_height : u32,
    pub velocity: Velocity,
    pub position: Position,
}

impl Camera {
    pub fn convert_x_pos_to_screen(&self, x_pos: &f32, object_width: &f32, rotation: f32) -> f32 {
        //calculate offset factoring in height and what not
        let offset_x = x_pos - ((rotation + 135.0).sind() * (object_width * 0.75));

        let camera_adjusted = offset_x - self.position.x;

        let zoom_adjusted = camera_adjusted / self.zoom;

        let x_adjusted = (self.screen_width as f32 / 2.0) + zoom_adjusted;

        x_adjusted
    }
    
    pub fn convert_y_pos_to_screen(&self, y_pos: &f32, object_height: &f32, rotation: f32) -> f32 {

        //calculate offset factoring in height and what not
        let offset_y = y_pos - ((rotation + 135.0).cosd() * (object_height * 0.75));

        let camera_adjusted = offset_y - self.position.y;

        let zoom_adjusted = camera_adjusted / self.zoom;

        let flipped_y_adjusted = (self.screen_height as f32 / 2.0) - zoom_adjusted;

        flipped_y_adjusted
    }

    pub fn new() -> Self {
        Camera {
            zoom: 0.025,
            //zoom: 0.1,
            velocity: Velocity {
                x: 0.0,
                y: 0.0,
            },
            position: Position{
                x: 0.0,
                y: 0.0,
            },
            screen_width: 1280,
            screen_height: 720,
            base_zoom: 1.0,
        }
    }
}