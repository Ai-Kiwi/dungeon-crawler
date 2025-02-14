
//reused for player and also for mobs

pub struct DamageType {
    pub damage : f32,
}

use crate::player::Player;
use super::monster::Monster;

pub trait Health{
    fn deal_damage(&mut self,damage_type:DamageType) -> ();
    fn regen(&mut self) -> ();
}

impl Health for Player {
    fn deal_damage(&mut self,damage_type:DamageType) -> () {
        self.health = &self.health.clone() - damage_type.damage;
    }
    fn regen(&mut self) -> () {
        self.health += 0.02;
        if self.health > self.max_health {
            self.health = self.max_health;
        } 
    }
}

impl Health for Monster {
    fn deal_damage(&mut self,damage_type:DamageType) -> () {
        self.health = &self.health.clone() - damage_type.damage;
    }
    fn regen(&mut self) -> () {
        
    }
}


