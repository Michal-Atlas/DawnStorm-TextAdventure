use crate::world::Node;
use lazy_static::lazy_static;
use serde_json::from_str;
use std::collections::HashMap;
use std::include_str;

macro_rules! load_map {
    ($($x:literal),+) => {
        {let mut m = HashMap::new();

        $(
            m.insert($x, from_str(include_str!(concat!("../../data/map/",$x,".json"))).unwrap());
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
