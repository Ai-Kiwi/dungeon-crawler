use crate::{item::Item, physics::Position};

pub struct DroppedItem {
    pub item : Item,
    pub position : Position,
    pub count : u32,
    pub rotation : f32,
}

