use std::io::{stdin, stdout, Write};

use ansi_term::Colour::Cyan;
use dawnstorm_core::load_methods::*;

fn main() {
    write_player();
    let player = Box::new(load_player().expect("Player Load"));
    let world = load_world().expect("Map Load");

    let stdin = stdin();
    let mut run = true;
    while run {
        print!("{} ", Cyan.bold().paint(">>"));
        stdout().flush();
        let mut input = String::new();
        stdin.read_line(&mut input);
    }
}
