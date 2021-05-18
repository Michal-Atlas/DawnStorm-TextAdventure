use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profession {
    pub xp: u8,
    pub tier: u8,
    pub profession: ProfessionEnum,
}

impl Profession {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfessionEnum {
    Fighter,
    Recruit,
    Squire,
    Hunter,
    Herbalist,
    Noble,
    Thief,
}
