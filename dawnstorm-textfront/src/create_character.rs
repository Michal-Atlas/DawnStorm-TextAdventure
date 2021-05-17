use colored::*;
use dawnstorm_core::entity::{Entity, Size, Stat};
use rustyline::{error::ReadlineError, Editor};
use std::collections::HashMap;

macro_rules! get_stat {
    ($name:expr, $var:expr) => {
        let mut rl = Editor::<()>::new();
        loop {
            match rl.readline(format!("{} > ", $name).as_str()) {
                Ok(l) => {
                    let l = l.parse();
                    if l.is_err() {
                        continue;
                    }
                    $var = l.unwrap();
                    break;
                }

                Err(ReadlineError::Eof) => {
                    println!("{}", "Sorry to see you go".bright_red().bold());
                    return None;
                }
                _ => {}
            }
        }
    };
}

pub fn create_character() -> Option<Entity> {
    let mut player = Entity {
        body: Stat::new(5),
        soul: Stat::new(5),
        influence: Stat::new(5),
        size: Size::Normal,
        professions: Vec::new(),
        flags: HashMap::new(),
        inventory: Vec::new(),
        abilities: Vec::new(),
    };
    /*get_stat!("Strength", player.body.max);
    get_stat!("Intelligence", player.soul.max);
    get_stat!("Charisma", player.influence.max);
    println!(
        "Your power level is {}, the recommended level is between 15 and 20",
        player.body.max + player.soul.max + player.influence.max
    );*/

    Some(player)
}
