mod create_character;
mod parser;
use colored::*;
use create_character::create_character;
use dawnstorm_core::dataload::WORLD_MAP;
use parser::parser;
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut world = WORLD_MAP.clone();
    let mut current_room = String::from("main");
    let player = create_character();
    if player.is_none() {
        return;
    }
    let mut player = player.unwrap();

    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(l) => {
                rl.add_history_entry(l.as_str());
                match l.as_str() {
                    "exit" => break,
                    "look around" => println!("{}", &world[current_room.as_str()]),
                    _ => {
                        parser(&mut player, &mut world, &mut current_room, &l);
                    }
                }
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Sorry to see you go".bright_red().bold());
                break;
            }
            _ => {
                panic!("Error in Parsing")
            }
        }
    }
}
