use dawnstorm_core::{entity::Entity, syscall::Command, world::World};
mod search;
mod sysexec;
use search::room_search;
use sysexec::sys_exec;

pub fn parser(player: &mut Entity, world: &mut World, current_node: &mut String, command: &str) {
    let mut input = command.split_whitespace();
    let command = input.next();
    // Transform str to Command Enum
    let command = match command {
        Some(command) => {
            if COMMAND_ALIASES.contains_key(command) {
                COMMAND_ALIASES[command].clone()
            } else {
                println!("I don't know that verb");
                return;
            }
        }
        None => return,
    };
    // Search verb to apply verb on
    match room_search(
        &world[current_node.as_str()],
        &input
            .map(|x| x.to_string().to_lowercase())
            .collect::<Vec<_>>()
            .as_slice(),
    ) {
        Some(s) => {
            // If the object has the associated Action
            if s.commands.contains_key(&command) {
                // Execute it
                sys_exec(s.commands[&command].clone(), player, world, current_node);
            } else {
                println!("I'm not sure how to do that");
            }
        }
        None => println!("Can't find the object you were looking for."),
    }
}

const COMMAND_ALIASES: phf::Map<&'static str, Command> = phf::phf_map! {
    "move" => Command::Move,
    "m" => Command::Move,
    "walk" => Command::Move,
    "talk" => Command::Talk,
    "ask" => Command::Talk,
    "look" => Command::Look,
    "watch" => Command::Look,
};
