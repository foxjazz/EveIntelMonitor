use crate::config::Config;
use crate::db::SystemsDb;
use std::fs;
use std::path::Path;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;

pub fn start_listening(config: &Config, systems_db: &mut SystemsDb) {
    // TODO: Implement the listening logic here
    let current_position = config.position.clone();

    println!("Starting listener with config: {:?}", config);
    // Find the most recent log file in the chatlog folder that starts with the first monitor_file

    if let Some(monitor_file) = config.monitor_files.first() {
        let chatlog_dir = Path::new(&config.chatlog_folder);
        if let Ok(entries) = fs::read_dir(chatlog_dir) {
            let mut latest_log = None;
            let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                    if fname.starts_with(monitor_file) {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                if modified > latest_time {
                                    latest_time = modified;
                                    latest_log = Some(path.clone());
                                }
                            }
                        }
                    }
                }
            }

            if let Some(log_path) = latest_log {
                println!("Most recent log for {}: {:?}", monitor_file, log_path);
                // You can now process the log_path as needed
            } else {
                println!("No log files found starting with {}", monitor_file);
            }
        } else {
            println!("Could not read chatlog folder: {}", config.chatlog_folder);
        }
    } else {
        println!("No monitor_files configured.");
    }


}
