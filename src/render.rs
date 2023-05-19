use std::io::{self, stdout, Write};

use crossterm::{cursor, event::KeyCode, execute, queue, terminal};

use crate::contents::{EditorContents, InsertHorizontalPosition, PlainText};

pub struct Render {
    pub content: EditorContents,
    width: usize,
    height: usize,
    cursor_controller: CursorController,
}

impl Render {
    pub fn new() -> Result<Self, RenderError> {
        let (term_width, term_height) = terminal::size().unwrap();
        let width = term_width as usize;
        let height = term_height as usize;
        if width < 10 && height < 10 {
            return Err(RenderError::TerminalSizeError(
                "Terminal size is too small!",
            ));
        }
        Ok(Self {
            content: EditorContents::new(width, height),
            width,
            height,
            cursor_controller: CursorController::new(width, height),
        })
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_rows(&mut self) {
        self.content
            .row_contents
            .edit_single_row(crate::contents::Text::Plain(PlainText::new(
                "Hello Text",
                self.width,
                InsertHorizontalPosition::Center,
                9,
            )));
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.content, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        self.content.rows_to_string();
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.content,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.content.flush()
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
                if self.cursor_y < self.height {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.width {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}
