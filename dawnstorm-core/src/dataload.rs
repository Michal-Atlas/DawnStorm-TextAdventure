use crate::world::Node;
use lazy_static::lazy_static;
use serde_json::from_str;
use std::collections::HashMap;
use std::include_str;

macro_rules! load_map {
    ($($x:literal),+) => {
        {let mut m = HashMap::new();

        $(
            m.insert($x, from_str(include_str!(concat!("../../data/map/",$x,".json"))).expect(format!("There was an issue in the '{}.json' file", $x).as_str()));
        )+
        m
    }};
}

lazy_static! {
    pub static ref WORLD_MAP: HashMap<&'static str, Node> = load_map! {
        "main",
        "castle_path"
    };
}
