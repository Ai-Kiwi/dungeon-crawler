use crate::physics::Position;



pub struct EnvironmentalObject {
    pub object_type : EnvironmentalObjectType,
    pub position : Position,
}


pub enum EnvironmentalObjectType {
    DeadTree,
    Tree,
    AppleTree,
    SwampTree,
    Bush,
    JungleTree
}