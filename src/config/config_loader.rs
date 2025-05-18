use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub position: String,
    pub log_folder: String,
    pub sound_folder: String,
    pub monitor_files: Vec<String>,
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
    
    config
}

pub fn save_config(path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string_pretty(config)?;
    fs::write(path, serialized)?;
    Ok(())
}
