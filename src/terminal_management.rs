use std::{fmt, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

pub struct Terminal {
    contents: [[&'static str; 8]; 8],
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            contents: [["-"; 8]; 8],
        }
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.contents.iter().enumerate() {
            writeln!(f, "{} | {}", 8 - i, row.join(" "))?;
        }
        write!(f, "  | A B C D E F G H")?;
        Ok(())
    }
}

impl Terminal {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }

    fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        self.process_keypress()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}
