use std::collections::HashMap;

use uuid::Uuid;

use crate::physics::Position;


#[derive(Clone)]
pub struct EnvironmentalObject {
    pub id : Uuid,
    pub object_type : EnvironmentalObjectType,
    pub position : Position,
}

#[derive(Clone)]
pub enum EnvironmentalObjectType {
    DeadTree,
    Tree,
    AppleTree,
    SwampTree,
    Bush,
    JungleTree
}

impl EnvironmentalObject {
    pub fn from_id(environmental_object_hashmap : &HashMap<Uuid, EnvironmentalObject>, id : Uuid) -> Option<&EnvironmentalObject> {
        return environmental_object_hashmap.get(&id)
    }
    pub fn from_id_mut(environmental_object_hashmap : &mut HashMap<Uuid, EnvironmentalObject>, id : Uuid) -> Option<&mut EnvironmentalObject> {
        return environmental_object_hashmap.get_mut(&id)
    }
}