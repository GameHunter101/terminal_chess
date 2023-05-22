use std::{
    fmt,
    io::{self, stdin, stdout, Read, Write},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue, terminal,
};

use crate::render::{Render, RenderError};

pub struct Terminal {
    render: Render,
    key_down: bool,
}

impl Terminal {
    pub fn new(renderer: Render) -> Self {
        Self {
            render: renderer,
            key_down: false,
        }
    }
    fn read_key(&mut self) -> crossterm::Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.read_key().unwrap() {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            } => self.render.move_cursor(direction),
            KeyEvent { code: KeyCode::Enter, .. } => self.render.press_button(),
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.render.refresh_screen().unwrap();
        self.process_keypress()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Render::clear_screen().expect("Error");
    }
}
