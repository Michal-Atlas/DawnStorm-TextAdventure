use crate::syscall::{Command, SysCall};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

pub type World = HashMap<&'static str, Node>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub description: String,
    pub aliases: Vec<String>,
    pub children: Option<Vec<Node>>,
    pub commands: HashMap<Command, SysCall>,
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.description.is_empty() {
            write!(f, "{} ", self.description)?;
        }
        if self.children.is_some() {
            for c in self.children.as_ref().unwrap() {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}
