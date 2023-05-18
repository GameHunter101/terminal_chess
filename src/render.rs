use std::io::{self, stdout, Write};

use crossterm::{cursor, execute, queue, terminal};

use crate::terminal_management::EditorContents;

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
            content: EditorContents::new(),
            width,
            height,
            cursor_controller: CursorController::new(),
        })
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_rows(&mut self) {
        for i in 0..self.height {
            if i == self.height / 3 {
                let mut welcome = format!("Pound Editor --- Version {}", 1.0);
                if welcome.len() > self.width {
                    welcome.truncate(self.width);
                }
                let mut padding = (self.width - welcome.len()) / 2;
                if padding != 0 {
                    self.content.push('~');
                    padding -= 1;
                }
                (0..padding).for_each(|_| self.content.push(' '));
                self.content.push_str(&welcome);
            } else {
                self.content.push('~');
            }
            queue!(
                self.content,
                terminal::Clear(terminal::ClearType::UntilNewLine)
            )
            .unwrap();
            if i < self.height - 1 {
                self.content.push_str("\r\n");
            }
            stdout().flush();
        }
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.content, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
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
}

impl CursorController {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}
