use crate::syscall::SysCall;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

pub type World = HashMap<&'static str, Node>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub description: String,
    pub aliases: Vec<String>,
    pub children: Vec<Node>,
    pub commands: HashMap<String, SysCall>,
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.description)?;
        for c in &self.children {
            write!(f, "{} ", c.description)?;
        }
        Ok(())
    }
}
