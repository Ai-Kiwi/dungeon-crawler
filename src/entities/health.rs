
//reused for player and also for mobs

pub struct DamageType {
    pub damage : f32,
}

use crate::player::Player;
use super::monster::Monster;

pub trait Health{
    fn deal_damage(&mut self,damage_type:DamageType, player:bool) -> ();
    fn regen(&mut self) -> ();
}

impl Health for Player {
    fn deal_damage(&mut self,damage_type:DamageType, player:bool) -> () {
        self.health = &self.health.clone() - damage_type.damage;
    }
    fn regen(&mut self) -> () {
        self.health += 0.02;
        if self.health > self.stats.max_health {
            self.health = self.stats.max_health;
        } 
    }
}

impl Health for Monster {
    fn deal_damage(&mut self,damage_type:DamageType,player:bool) -> () {
        self.health = &self.health.clone() - damage_type.damage;
        if player {
            self.player_damaged = true;
        }
    }
    fn regen(&mut self) -> () {
        
    }
}


