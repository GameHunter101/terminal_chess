use std::{
    fmt,
    io::{self, stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    queue, terminal,
};
use rand::Rng;

use crate::render::{Render, RenderError};

pub struct Terminal {
    render: Render,
    key_down: bool,
}

impl Terminal {
    pub fn new() -> Result<Self, RenderError> {
        Ok(Self {
            render: Render::new()?,
            key_down: false,
        })
    }
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            // if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
            // }
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            KeyEvent {
                code: direction @ ((KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right)),
                modifiers: event::KeyModifiers::NONE,
                ..
            } => {
                if !self.key_down {
                    self.key_down = true;
                    /* let mut rng = rand::thread_rng();
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                            r: rng.gen_range(0..255),
                            g: rng.gen_range(0..255),
                            b: rng.gen_range(0..255),
                        })
                    )
                    .unwrap(); */
                    self.render.move_cursor(direction);
                    self.key_down = false;
                }
            }
			KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                ..
            } => {
				self.render.press_button();
			},
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.render.refresh_screen()?;
        self.process_keypress()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Render::clear_screen().expect("Error");
    }
}
