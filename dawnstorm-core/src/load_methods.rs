use crate::{
    entity::{Entity, Stat},
    item::{Item, WeaponType},
    profession::Profession,
    world::{Edge, Node, World},
};
use std::{collections::HashMap, io::Read};
use std::{fs::File, io::Write};

pub fn load_player() -> Result<Entity, serde_json::Error> {
    let mut f = File::open("data/player.json").expect("No Player Found in Data Directory");
    let mut buff = String::new();
    f.read_to_string(&mut buff).unwrap();
    serde_json::from_str(&buff)
}
pub fn load_world() -> Result<World, serde_json::Error> {
    let mut f = File::open("data/world.json").expect("No Map Found in Data Directory");
    let mut buff = String::new();
    f.read_to_string(&mut buff).unwrap();
    serde_json::from_str(&buff)
}

#[cfg(feature = "default_gen")]
pub fn write_player() {
    let player = Entity {
        body: Stat { max: 5, spent: 0 },
        soul: Stat { max: 5, spent: 0 },
        influence: Stat { max: 5, spent: 0 },
        size: crate::entity::Size::Normal,
        professions: vec![Profession {
            xp: 0,
            tier: 2,
            profession: String::from("Recruit"),
        }],
        flags: HashMap::new(),
        inventory: vec![Item {
            name: String::from("Blade of Blading"),
            r#type: crate::item::ItemType::Weapon(WeaponType::ShortSlashing),
            major: false,
            makeshift: false,
        }],
    };
    let mut f = File::create("data/player.json").unwrap();
    f.write_all(serde_json::to_string_pretty(&player).unwrap().as_bytes())
        .unwrap();
}

#[cfg(feature = "default_gen")]
pub fn write_world() {
    let mut map = HashMap::new();
    map.insert(
        "town_square",
        Node {
            name: String::from("Town Square"),
            edges: vec![Edge {
                target: String::from("back_alley"),
                flags: vec![],
            }],
        },
    );
    map.insert(
        "back_alley",
        Node {
            name: String::from("Back Alley"),
            edges: vec![Edge {
                target: String::from("town_square"),
                flags: vec![],
            }],
        },
    );
    let mut f = File::create("data/world.json").unwrap();
    f.write_all(serde_json::to_string_pretty(&map).unwrap().as_bytes())
        .unwrap();
}
