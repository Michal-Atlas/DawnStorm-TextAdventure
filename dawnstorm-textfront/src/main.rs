mod create_character;
mod parser;
use colored::*;
use create_character::create_character;
use dawnstorm_core::defaults::DEFAULT_WORLD;
use parser::parser;
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut world = DEFAULT_WORLD.clone();
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
                if l == "exit" {
                    break;
                }
                parser(&mut player, &mut world, &l);
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
