use crate::item::Item;
use crate::profession::Profession;
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
}
impl Entity {
    pub fn Sleep(&mut self, hours: u8) {
        self.body.Sleep(hours);
        self.soul.Sleep(hours);
        self.influence.Sleep(hours);
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub max: u8,
    pub spent: u8,
}
impl Stat {
    pub fn Sleep(&mut self, hours: u8) {
        let divisor = match hours {
            1..=3 => 4,
            4..=7 => 2,
            8..=255 => 1,
            _ => return,
        };
        self.spent = self.spent.checked_sub(self.max / divisor).unwrap_or(0);
    }
}
#[derive(Debug, Serialize, Deserialize)]
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
