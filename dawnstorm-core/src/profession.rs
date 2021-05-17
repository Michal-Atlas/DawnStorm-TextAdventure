use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profession {
    pub xp: u8,
    pub tier: u8,
    pub profession: String,
    //pub abilities: Vec<Ability>,
}

impl Profession {}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct Ability {}
