use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveGame {
    pub player: Entity,
}
