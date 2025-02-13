use std::collections::HashMap;

use uuid::Uuid;

use crate::{item::Item, physics::Position};

#[derive(Clone)]
pub struct DroppedItem {
    pub id : Uuid,
    pub item : Item,
    pub position : Position,
    pub count : u32,
    pub rotation : f32,
}

impl DroppedItem {
    pub fn from_id(dropped_item_hashmap : &HashMap<Uuid, DroppedItem>, id : Uuid) -> Option<&DroppedItem> {
        return dropped_item_hashmap.get(&id)
    }
    pub fn from_id_mut(dropped_item_hashmap : &mut HashMap<Uuid, DroppedItem>, id : Uuid) -> Option<&mut DroppedItem> {
        return dropped_item_hashmap.get_mut(&id)
    }
}