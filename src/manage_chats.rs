use std::io::{self, Write, stdout};
use std::{thread, time::Duration};
use crate::config::config_loader::{Config, load_config, save_config};
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue, terminal,
    terminal::{Clear, ClearType},
    style::{PrintStyledContent, Stylize},
};
use crate::utils::util_functions::*;
fn verify_chat_name(chat_name: &str, log_folder: &str) -> Option<String> {
    let log_folder = path_format(log_folder);
    let chat_path = &log_folder;
    let file_name = &chat_name;
    if std::path::Path::new(&chat_path).exists() {
        // Find the first file in the log_folder that starts with chat_name and contains "_20"
        if let Ok(entries) = std::fs::read_dir(log_folder) {
            for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            if file_name_str.starts_with(chat_name) && file_name_str.contains("_20") {
                if let Some(pos) = file_name_str.find("_20") {
                return Some(file_name_str[..pos].to_string());
                }
            }
            }
        }
    }
    None
}
pub fn manage_chats() {

    let mut status = String::new();
    let mut error = String::new();
    let path = format!("{}/src/config/config.json", env!("CARGO_MANIFEST_DIR"));
    let mut config = load_config(&path); 
    loop {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("\nCurrent Chats:\n\r");
        for (index, chat) in config.monitor_files.iter().enumerate() {
            println!("{}. {}\r", index + 1, chat);
        }

        let mut choice = String::new();
        let mut selected_index = 0;
        let options = ["Add a chat\r", "Remove a chat\r", "Exit and save\r"];
        loop {
            execute!(stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("\n\r");
        print!("status: {}\n\r", status);
        println!("\n\r");
        if !error.is_empty() {
            print!("error: {}\n\r", error);
        }
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
            println!("Use arrow keys to navigate and press Enter to select:\n\r");
            for (i, option) in options.iter().enumerate() {
                if i == selected_index {
                    println!("> {}\r", option); // Highlight the selected option
                } else {
                    println!("  {}\r", option);
                }
            }

            io::stdout().flush().unwrap();

            if let Event::Key(event) = event::read().unwrap() {

                match event.code {
                    
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < options.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Char(c) if c == 'a' || c == 'A' => {
                        choice = "1".to_string();
                        break;
                    }
                    KeyCode::Char(c) if c == 'r' || c == 'R' => {
                        choice = "2".to_string();
                        break;
                    }
                    KeyCode::Char(c) if c == 'e' || c == 'E' || c == 'q' => {
                        choice = "3".to_string();
                        break;
                    }
                    KeyCode::Esc => {
                        choice = "3".to_string();
                        break;
                    }
                    KeyCode::Enter => {
                        choice = (selected_index + 1).to_string();
                        break;
                    }
                    _ => {}
                }
            }
        }

        match choice.trim() {
            "1" => {

                io::stdout().flush().unwrap();
                let mut chat_name = prompt_input("Enter the name of the chat to add: "); 
                if (chat_name.len() > 0) {
                    if let Some(valid_name) = verify_chat_name(&chat_name, &config.log_folder) {
                        // Extract the part before the first "_20" in the file name
                        chat_name = valid_name;
                        // If verify_chat_name returns None, we already handle the error and set status below
                        // Prevent duplicates in monitor_files
                        if config.monitor_files.iter().any(|c| c == &chat_name) {
                            status = format!("Chat '{}' is already being monitored.", chat_name);
                            continue;
                        }
                        status = format!("Chat added: {}", chat_name);
                    } else {
                        status = "Chat log not found in log folder. Please enter a valid chat name.".to_string();
                        continue;
                    }
                    chat_name = chat_name.trim().to_string();
                    // Prevent duplicates in monitor_files (already checked above, but double-check here)
                    if !config.monitor_files.iter().any(|c| c == &chat_name) {
                        config.monitor_files.push(chat_name.clone());
                        status = format!("Chat added: {}. Current chats: {:?}", chat_name, config.monitor_files);
                    }
                    
                } else {
                    status = "Invalid input. Please enter a valid chat name.".to_string();
                    continue;
                }
                terminal::enable_raw_mode().unwrap(); // Re-enable raw mode after input
                io::stdout().flush().unwrap(); // Ensure stdout is flushed after re-enabling raw mode
            }
            "2" => {
                io::stdout().flush().unwrap();

                // List chats with reference numbers
                println!("\nCurrent Chats:");
                for (i, chat) in config.monitor_files.iter().enumerate() {
                    println!("{}. {}", i + 1, chat);
                }
                // Prompt user for the number of the chat to remove
                let chat_index = prompt_input("Enter the number of the chat to remove: ");
                let chat_index = chat_index.trim();
                if let Ok(index) = chat_index.parse::<usize>() {
                    if index > 0 && index <= config.monitor_files.len() {
                        config.monitor_files.remove(index - 1);
                        println!("Chat removed.");
                    } else {
                        println!("Invalid chat number. Please enter a number between 1 and {}.", config.monitor_files.len());
                        continue;
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                    continue;
                }
                
                // Save config after removing a chat
                if let Err(err) = save_config(&path, &config) {
                    eprintln!("Failed to save configuration: {}", err);
                }
            }
            "3" => {
                println!("Exiting.");
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(600));
                break;
            }
            _ => {
                status = "Invalid choice. Please try again.".to_string();
            }
        }
    }
}