use serde::Deserialize;
use shellexpand::tilde;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub position: String,
    pub log_folder: String,
    pub sound_folder: String,
}

pub fn expand_path(path: &str) -> PathBuf {
    PathBuf::from(tilde(path).as_ref())
}
pub fn load_config(path: &str) -> Config {
    let raw = fs::read_to_string(path).expect("Failed to read config.json");
    let mut config: Config = serde_json::from_str(&raw).expect("Invalid JSON format");

    config.log_folder = expand_path(&config.log_folder)
        .to_string_lossy()
        .to_string();
    config.sound_folder = expand_path(&config.sound_folder)
        .to_string_lossy()
        .to_string();

    println!("Log folder: {:?}", config.log_folder);
    println!("Sound folder: {:?}", config.sound_folder);
    config
}
