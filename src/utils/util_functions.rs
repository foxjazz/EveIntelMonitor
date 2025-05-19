

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue, terminal,
    terminal::{Clear, ClearType},
    style::{PrintStyledContent, Stylize},
};
use std::io::{self, Write, stdout};
use crossterm::event::KeyEvent;

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
fn filter_list<'a>(items: &'a [String], query: &str) -> Vec<&'a String> {
    items
        .iter()
        .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn run_filter(title: &str,items: &[String]) -> Result<(), io::Error> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Show)?;

    let mut input = String::new();

    loop {
        // Clear the screen each iteration
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Clear(ClearType::All))?;
        write!(stdout, "\r{}\r\n", title)?;
        // Draw the prompt at a fixed position
        write!(stdout, "Search: {}", input)?;
        stdout.flush()?;

        let (x,y) = cursor::position().unwrap();

        // Display the filtered results below the prompt
        let filtered = filter_list(items, &input);
        for (i, item) in filtered.iter().take(10).enumerate() {
            execute!(stdout, MoveTo(x, y + 2 + i as u16))?;
            writeln!(stdout, "{}", item)?;
        }

        execute!(stdout, MoveTo(x  as u16, y))?;
        stdout.flush()?;

        // Handle input
        let code = get_key()?;
        match code {
            KeyCode::Char(c) => input.push(c),
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Enter => break,
            KeyCode::Esc => {
                input.clear();
                break;
            }
            _ => {}
        }

    }

    // Cleanup
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        cursor::MoveTo(0, 0)
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}
pub fn get_key() -> io::Result<KeyCode> {
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                return Ok(code);
            }
        }
    }
}