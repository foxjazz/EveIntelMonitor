mod config;
mod db;
use config::config_loader::{Config, load_config};
use config::expand_path; // from config/mod.rs // from config/config_loader.rs
// mod utils;
fn main() {
    let path = format!("{}/src/config/config.json", env!("CARGO_MANIFEST_DIR"));
    //let config: Config = load_config("src/config/config.json");
    let config = load_config(&path);
    let log_folder = expand_path(&config.log_folder);
    let sound_folder = expand_path(&config.sound_folder);
    println!("Hello, world!");
}
