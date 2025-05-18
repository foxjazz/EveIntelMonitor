

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue, terminal,
    terminal::{Clear, ClearType},
    style::{PrintStyledContent, Stylize},
};
use std::io::{self, Write, stdout};
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

pub fn run_filter(items: &[String], x: u16, y: u16) -> Result<(), io::Error> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Show)?;

    let mut input = String::new();

    loop {
        // Clear the screen each iteration
        execute!(stdout, Clear(ClearType::All))?;

        // Draw the prompt at a fixed position
        execute!(stdout, MoveTo(x, y))?;
        write!(stdout, "Search: {}", input)?;
        stdout.flush()?;

        // Move the cursor to the end of input for blinking
        execute!(stdout, MoveTo(x + 8 + input.len() as u16, y))?;

        // Display the filtered results below the prompt
        let filtered = filter_list(items, &input);
        for (i, item) in filtered.iter().take(10).enumerate() {
            execute!(stdout, MoveTo(x, y + 2 + i as u16))?;
            writeln!(stdout, "{}", item)?;
        }

        stdout.flush()?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
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