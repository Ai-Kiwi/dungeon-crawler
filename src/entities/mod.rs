
use environmental_object::EnvironmentalObject;
use monster::Monster;

use crate::physics::Position;

pub mod monster;
pub mod building;
pub mod npc;
pub mod projectile;
pub mod environmental_object;
pub mod dropped_item;

pub struct DamageType {
    pub damage : f32,
}

pub trait Entity {
    fn position(&self) -> Position;
}

impl Entity for EnvironmentalObject {
    fn position(&self) -> Position {
        self.position.clone()
    }
}

impl Entity for Monster {
    fn position(&self) -> Position {
        self.movement.position.clone()
    }
}