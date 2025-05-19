use crate::config::config_loader::{Config, load_config};
use crate::config::expand_path;
use crate::db;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue, terminal,
    terminal::{Clear, ClearType},
    style::{PrintStyledContent, Stylize},
};
use std::io::{Write, stdout};
use std::{thread, time::Duration};
use crate::manage_chats::manage_chats;
use crate::db::SystemsDb;
use crate::utils::util_functions::get_key;

#[derive(Debug)]
enum Mode {
    Listening,
    LogsMonitored,
    AddSystem,
    Config,
}

#[derive(Debug)]
struct AppState {
    mode: Mode,
    position: String,
}
pub fn start() -> std::io::Result<()> {
    let path = format!("{}/src/config/config.json", env!("CARGO_MANIFEST_DIR"));
    //let config: Config = load_config("src/config/config.json");
    let config = load_config(&path);
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    let mut db = SystemsDb::new();
    let mut state = AppState {
        mode: Mode::Listening,
        position: config.position,
    };

    loop {
        // Clear screen
        queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Render state
        writeln!(
            stdout,
            "Mode: {:?}\nPosition: {}",
            state.mode, state.position
        )?;
        // writeln!(
        //     stdout,
        //     "\nPress 'a' to Add System\n 'c' for Config\n 'p' to change position\n 'q' to Quit."
        // )?;
        queue!(
            stdout,
            MoveTo(0, 5),
            crossterm::style::PrintStyledContent("'a'".dark_cyan()),
            MoveTo(6, 5),
            crossterm::style::PrintStyledContent("Manage System".yellow()),

            MoveTo(0, 6),
            crossterm::style::PrintStyledContent("'c'".dark_cyan()),
            MoveTo(6, 6),
            crossterm::style::PrintStyledContent("Config".yellow()),
            MoveTo(0, 6),
            crossterm::style::PrintStyledContent("'m'".dark_cyan()),
            MoveTo(6, 6),
            crossterm::style::PrintStyledContent("Manage Chats".yellow()),
            MoveTo(0, 7),
            crossterm::style::PrintStyledContent("'p'".dark_cyan()),
            MoveTo(6, 7),
            crossterm::style::PrintStyledContent("Change Position".yellow()),

            MoveTo(0, 8),
            crossterm::style::PrintStyledContent("'q'".dark_cyan()),
            MoveTo(6, 8),
            crossterm::style::PrintStyledContent("Quit".yellow())
        )?;
        stdout.flush()?;


        // Poll input
        let code = get_key()?;
        match code {
            KeyCode::Char('q') => break,
            KeyCode::Char('a') => {
                state.mode = Mode::AddSystem;
                db.manage_systems();},
            KeyCode::Char('c') => state.mode = Mode::Config,
            KeyCode::Char('l') => state.mode = Mode::Listening,
            KeyCode::Char('m') => {
                state.mode = Mode::LogsMonitored;
                manage_chats(); // Call the manage_chats function
            },
            _ => {} 
        }
       
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
