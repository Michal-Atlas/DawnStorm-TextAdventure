use crate::savegame::SaveGame;
use std::{fs::File, io::Write};

pub fn save_game(save_path: &str, save_game: SaveGame) -> Result<(), serde_json::Error> {
    let mut f = File::open(save_path).expect("No Player Found in Data Directory");
    f.write_all(serde_json::to_string(&save_game)?.as_bytes())
        .unwrap();
    Ok(())
}
