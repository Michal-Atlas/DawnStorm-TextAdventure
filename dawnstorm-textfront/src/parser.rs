
use dawnstorm_core::{entity::Entity, world::World};
mod search;
mod sysexec;
use search::room_search;

use sysexec::sys_exec;

pub fn parser(player: &mut Entity, world: &mut World, current_node: &mut String, command: &str) {
    let mut input = command.split_whitespace();
    let command = input.next();
    let command = match command {
        Some(command) => {
            if COMMAND_ALIASES.contains_key(command) {
                COMMAND_ALIASES[command]
            } else {
                command
            }
        }
        None => return,
    };
    match room_search(
        &world[current_node.as_str()],
        input.map(|x| x.to_string().to_lowercase()).collect(),
    ) {
        Some(s) => {
            if s.commands.contains_key(command) {
                sys_exec(&s.commands[command].clone(), player, world, current_node);
            } else {
                println!("I'm not sure how to do that");
            }
        }
        None => println!("Can't find the object you were looking for."),
    }
}

const COMMAND_ALIASES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "move" => "walk",
    "m" => "walk",
};
