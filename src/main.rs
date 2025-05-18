// from config/mod.rs // from config/config_loader.rs
// mod utils;
//
mod config;
mod db;
mod manage_chats;
use runner::start;
//use std::io::{Write, stdout};
mod runner;
mod utils;

fn main() -> std::io::Result<()> {
    start()
}
