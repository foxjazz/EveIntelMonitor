use serde::Deserialize;
use shellexpand::tilde;
use std::fs;
use std::path::PathBuf;
pub mod config_loader;

pub fn expand_path(path: &str) -> PathBuf {
    PathBuf::from(tilde(path).as_ref())
}
// You can also define stuff directly here
pub struct Config {
    pub position: String,
    pub log_folder: String,
    pub sound_jump_folder: String,
    pub monitor_files: Vec<String>,
}
