use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::io::{stdin, stdout};
use std::path::Path;
 use crossterm::{
     cursor::{self, MoveTo},
     event::{self, Event, KeyCode},
     execute, queue, terminal,
     terminal::{Clear, ClearType},
     style::{PrintStyledContent, Stylize},
 };
 use serde::{Serialize, Deserialize};
use crate::utils::util_functions::{prompt_input, run_filter};

#[derive(Debug,Clone, Serialize, Deserialize)]
 pub struct Connection {
     pub rel_system: String,
     pub jump_number: u32
 }
 #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEntry {
     pub system: String,
     pub connections: Vec<Connection>,
    // Each system maps to a set of connected systems
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemsDb {
    pub systems: Vec<SystemEntry>,
}
impl SystemsDb {
    /// Create a new, empty SystemsDb
    pub fn new() -> Self {
        SystemsDb {
            systems: Vec::new(),
        }
    }
    fn load_from_json<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file_content = fs::read_to_string(path).expect("Failed to read db.json");
        let db: SystemsDb = serde_json::from_str(&file_content)
            .map_err(|e| {
                eprintln!("JSON parse error: {}", e);
                io::Error::new(io::ErrorKind::InvalidData, e)
            })?;
        Ok(db)
    }

    pub fn manage_systems(&mut self) {
        let mut input = String::new();

        let path = format!("{}/src/db/db.json", env!("CARGO_MANIFEST_DIR"));
        if let Ok(db) = Self::load_from_json(&path) {
            self.systems = db.systems;
        } else {
            eprintln!("Failed to load systems from db.json");
        }

        loop {
            terminal::enable_raw_mode().unwrap();
            execute!(stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            
            let items: Vec<String> = self.systems.iter().map(|x| x.system.clone()).collect();
            stdout().flush().unwrap();
            let list = run_filter("Select system to modify",&items);
             
        }
    }
}
  