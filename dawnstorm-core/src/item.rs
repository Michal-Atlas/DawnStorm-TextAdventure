use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub r#type: ItemType,
    pub major: bool,
    pub makeshift: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemType {
    Misc,
    Weapon(WeaponType),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WeaponType {
    Slashing,
    ShortSlashing,
    Blunt,
    Piercing,
    Short,
    Long,
    Shield,
}
