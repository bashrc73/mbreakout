use crate::consts::*;
use crate::game::*;
use crate::shop::*;

use dirs::home_dir;
use std::fs::*;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn game_save(game: &Game) {
    let home = home_dir().unwrap_or_default();
    let filepath = home.join(SAVE_FILENAME);

    let Ok(file) = File::create(filepath) else {
        return;
    };

    let mut writer = BufWriter::new(file);
    for s in game.secrets_generated.iter() {
        if !s.is_empty() {
            writeln!(writer, "{}", s).unwrap();
        }
    }
    for s in game.secrets_unlocked.iter() {
        if !s.is_empty() {
            writeln!(writer, "{}", s).unwrap();
        }
    }
    for s in game.levels_unlocked.iter() {
        if !s.is_empty() {
            writeln!(writer, "{}", s).unwrap();
        }
    }
    writer.flush().unwrap();
}

pub fn game_load(game: &mut Game) {
    let home = home_dir().unwrap_or_default();
    let filepath = home.join(SAVE_FILENAME);
    let Ok(file) = File::open(filepath) else {
        return;
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let Ok(line) = line else { continue };
        let _ = shop_code_add(game, line.as_str());
    }
    shop_process_secrets(game);    
}

pub fn game_reset() {
    let home = home_dir().unwrap_or_default();
    let filepath = home.join(SAVE_FILENAME);
    let _ = remove_file(filepath);
}
