use crate::profession::ProfessionEnum;
use serde::{Deserialize, Serialize};

/// An action that using a Verb on a Node causes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SysCall {
    Move(String),
    Call(Box<SysCall>, Box<SysCall>),
    Print(String),
    /// Contains a list of which professions give a bonus to the check
    /// Then what to do on success and then on failure
    SkillCheck(Vec<ProfessionEnum>, Box<SysCall>, Box<SysCall>),
}

/// The Verb of the player's Input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Command {
    Move,
    Look,
    Talk,
}
