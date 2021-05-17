use crate::item::Item;
use crate::profession::Profession;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub body: Stat,
    pub soul: Stat,
    pub influence: Stat,
    pub size: Size,
    pub professions: Vec<Profession>,
    pub flags: HashMap<String, bool>,
    pub inventory: Vec<Item>,
    pub abilities: Vec<Ability>,
}
impl Entity {
    pub fn sleep(&mut self, hours: u8) {
        self.body.sleep(hours);
        self.soul.sleep(hours);
        self.influence.sleep(hours);
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub max: u8,
    pub spent: u8,
}
impl Stat {
    pub fn new(max: u8) -> Self {
        Self { max, spent: 0 }
    }
    pub fn sleep(&mut self, hours: u8) {
        let divisor = match hours {
            1..=3 => 4,
            4..=7 => 2,
            8..=255 => 1,
            _ => return,
        };
        self.spent = self.spent.checked_sub(self.max / divisor).unwrap_or(0);
    }
}
#[derive(Debug, Display, Serialize, Deserialize)]
pub enum Size {
    Scrawny,
    Tiny,
    Small,
    Normal,
    Large,
    Huge,
    Giant,
    Colossal,
    Immense,
    Primal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ability {}
