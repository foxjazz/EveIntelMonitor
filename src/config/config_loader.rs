use serde::Deserialize;
use shellexpand::tilde;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    position: String,
    log_folder: String,
    sound_jump_folder: String,
}

pub fn load_config(path: &str) -> Config {
    let raw = fs::read_to_string(path).expect("Failed to read config.json");
    serde_json::from_str(&raw).expect("Invalid JSON format");

    let config = load_config("src/config/config.json");

    let log_folder = expand_path(&config.log_folder);
    let sound_folder = expand_path(&config.sound_jump_folder);

    println!("Log folder: {:?}", log_folder);
    println!("Sound folder: {:?}", sound_folder);
}
