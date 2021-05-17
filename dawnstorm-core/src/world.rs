use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type World = HashMap<String, Node>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub target: String,
    pub flags: Vec<String>,
}
