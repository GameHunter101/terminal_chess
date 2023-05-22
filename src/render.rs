use std::io::{self, stdout, Write};
use std::collections::HashMap;

use crossterm::{cursor, event::KeyCode, execute, queue, terminal};
use rand::Rng;

use crate::screen::{
    ButtonText, InsertHorizontalPosition, InsertVerticalPosition, PlainText, Screen, Text,
    TextContent,
};

pub struct Render {
    pub screens: Vec<Screen>,
    pub current_screen: usize,
    width: usize,
    height: usize,
    cursor_controller: CursorController,
}

impl Render {
    pub fn new(screen: Screen) -> Result<Self, RenderError> {
        let (term_width, term_height) = terminal::size().unwrap();
        let width = term_width as usize;
        let height = term_height as usize;
        if width < 10 && height < 10 {
            return Err(RenderError::TerminalSizeError(
                "Terminal size is too small!",
            ));
        }
		Ok(Self {
            screens: vec![screen],
            current_screen: 0,
            width,
            height,
            cursor_controller: CursorController::new(width, height),
        })
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

    pub fn press_button(&self) {
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        for button in &self.screens[self.current_screen].screen_rows.buttons[cursor_y] {
            if button.position_x() <= cursor_x && button.position_x() + button.length() > cursor_x {
                let on_click = self.screens[self.current_screen]
                    .button_map
                    .get_key_value(button.on_click)
                    .unwrap()
                    .1;
                on_click();
            }
        }
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.screens[self.current_screen],
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        self.screens[self.current_screen].compile_screen();
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.screens[self.current_screen],
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.screens[self.current_screen].flush()
    }

    pub fn new_screen(&mut self, screen: Screen) {
        self.screens.push(screen);
    }

    pub fn set_screen(&mut self, screen_index: usize) {
        self.current_screen = screen_index;
    }
}

#[derive(Debug)]
pub enum RenderError {
    AddTextError(&'static str),
    TerminalSizeError(&'static str),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Render error")
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
}

impl CursorController {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < self.height - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.width - 1 {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}
