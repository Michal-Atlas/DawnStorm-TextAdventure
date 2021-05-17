use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SysCall {
    Move(String),
    Call(Box<SysCall>, Box<SysCall>),
}
