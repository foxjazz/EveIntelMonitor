use config::config_loader::{Config, load_config};
use config::expand_path;
// mod config;
mod db;
// mod utils;
fn main() {
    let path = "/src/config/config.json";
    //let config: Config = load_config("src/config/config.json");
    let config = load_config(path);
    let log_folder = expand_path(&config.log_folder);
    let sound_folder = expand_path(&config.sound_jump_folder);
    println!("Hello, world!");
}
