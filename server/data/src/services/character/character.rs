use std::ops::{Add, Div};

use crate::entities;

#[derive(Debug, Clone)]
pub struct Character {
    pub model: entities::character::Model,
}

impl From<entities::character::Model> for Character {
    fn from(model: entities::character::Model) -> Self {
        Self { model }
    }
}

impl Character {
    pub fn decrease_exp(&mut self, town: bool) {
        if self.model.exp <= 0 || self.model.exp >= 200 {
            return;
        }

        let reduction_rate = match town {
            true => 0.01,
            false => {
                let temp_rate = if self.model.job.eq(&3) { 0.08 } else { 0.2 };
                temp_rate.div((self.model.luk as f64).add(0.05))
            }
        };

        // set exp to the max of 0 or the current exp minus the next level xp times reduction rate
        // TODO: get next level xp
        self.model.exp = 0.max(self.model.exp - (self.model.exp as f64 * reduction_rate) as i32);
    }

    pub fn update_hp(&mut self, hp: i32) {
        self.model.hp = 0.max(self.model.hp.add(hp)).min(self.model.max_hp);
    }

    pub fn update_mp(&mut self, mp: i32) {
        self.model.mp = 0.max(self.model.mp.add(mp)).min(self.model.max_mp);
    }
}
