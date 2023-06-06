use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal,
};

use crate::render::Render;

pub struct Terminal {
    render: Render,
}

impl Terminal {
    pub fn new(renderer: Render) -> Self {
        Self { render: renderer }
    }
    
    // Event loop for reading key presses
    fn read_key(&mut self) -> crossterm::Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }

    // Read the key presses, call corresponding functions
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
            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                ..
            } => self.render.move_cursor_half(direction),
            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            } => self.render.move_cursor_far(direction),
            KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            } => self.render.press_button(),

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
