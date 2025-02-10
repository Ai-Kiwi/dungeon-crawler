#[derive(Debug,Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub item: PremadeItem,
    pub enchantments: Vec<Enchantment>
}

#[derive(Debug)]
pub struct ItemInfo {
    pub premade: PremadeItem,
    pub name: String,
    pub description: String,
    pub value: u32,
    pub attributes: ItemAttributes,
    pub item_type: ItemCategory,      
    pub max_stack: Option<u32>, 
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Enchantment {
    
}

#[derive(Default, Debug)]
pub struct ItemAttributes {
    pub damage: Option<f32>,         // Damage dealt if a weapon
    pub attack_distance: Option<f32>,// Distance damage dealt if a weapon
    pub swing_distance: Option<f32>,// Distance damage dealt if a weapon
    pub durability: Option<u32>,     // Durability for items like tools
    pub heal_amount: Option<u32>,    // Health restored if consumable
    pub effect: Option<String>,      // Special effect (e.g., "Fire Resistance")
    pub special_attributes: Vec<SpecialAttribute>, // Special attributes (e.g., "Unbreakable")
}

#[derive(Debug)]
pub enum SpecialAttribute {
    Unbreakable,
    SoulBound,
    CurseOfVanishing,
    ParalysisInfliction,
    PoisonInfliction,
    

}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Consumable,
    CraftingMaterial,
    Miscellaneous,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PremadeItem {
    Stick,
}

impl PremadeItem {
    pub fn info(&self) -> ItemInfo {
        match self {
            PremadeItem::Stick => {
                ItemInfo{
                    premade: PremadeItem::Stick,
                    name: "Stick".to_owned(),
                    description: "Epic stick to smack the hell out of people".to_owned(),
                    value: 1,
                    attributes: ItemAttributes{
                        damage: Some(3.0),
                        attack_distance: Some(2.0),
                        swing_distance: Some(75.0),
                        ..Default::default()
                    },
                    item_type: ItemCategory::Miscellaneous,
                    max_stack: None,
                }
            },
        }
    }
}