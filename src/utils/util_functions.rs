

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue, terminal,
    terminal::{Clear, ClearType},
    style::{PrintStyledContent, Stylize},
};
pub fn path_format(path: &str) -> String {
    if path.ends_with('/') {
        path.to_string()
    } else {
        format!("{}/", path)
    }
}
use rustyline::DefaultEditor;
pub fn prompt_input(prompt: &str) -> String {
    terminal::disable_raw_mode().unwrap();
    let mut rl = DefaultEditor::new().unwrap();

    match rl.readline(prompt) {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).unwrap();
            return line;
        }
        Err(_) => return String::from("Error"),
    }
    terminal::enable_raw_mode().unwrap();
}