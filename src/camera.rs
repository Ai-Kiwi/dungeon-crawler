use crate::physics::{Position, Velocity};

pub struct Camera {
    pub zoom : f32,
    pub screen_width: u32,
    pub screen_height : u32,
    pub velocity: Velocity,
    pub position: Position,
}

impl Camera {
    pub fn convert_x_pos_to_screen(&self, x_pos: &f32, object_width: &f32, rotation: f32) -> f32 {
        let result = ((x_pos - self.position.x - (object_width / 2.0)) / self.zoom) + (self.screen_width as f32 / 2.0);
        result
    }
    
    pub fn convert_y_pos_to_screen(&self, y_pos: &f32, object_height: &f32, rotation: f32) -> f32 {

        let offset_y = (object_height / 2.0) * rotation.to_degrees().sin();

        let center_y = y_pos + offset_y;

        let camera_adjusted = center_y - self.position.y;

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
        }
    }
}