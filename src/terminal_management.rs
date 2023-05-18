use std::{
    fmt,
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

use crate::render::{Render, RenderError};

pub struct Terminal {
    render: Render,
}

impl Terminal {
    pub fn new() -> Result<Self, RenderError> {
        Ok(Self {
            render: Render::new()?,
        })
    }
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

/* #[derive(Clone, Copy)]
pub enum InsertHorizontalPosition {
    Exact(usize),
    Center,
    Right,
}

pub enum InsertVerticalPosition {
    Exact(usize),
    Center,
    Bottom,
}

#[derive(Clone)]
pub enum TextFormats {
    SingleLine(&'static str),
    MultiLine(Vec<&'static str>),
} */

pub struct EditorContents {
    content: String,
}

impl EditorContents {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}


/* fn set_contents(&mut self, string: String) {
    self.content = string;
}

pub fn add_text(
    &mut self,
    text: TextFormats,
    horizontal_position: InsertHorizontalPosition,
    vertical_position: InsertVerticalPosition,
    line_seperation: usize,
) -> Result<(), RenderError> {
    let mut split_content = self.split_contents(&self.content, self.width);

    let start_vertical_position =
        self.vertical_text_position(&text, vertical_position, line_seperation)?;
    match &text {
        &TextFormats::MultiLine(ref lines) => {
            for (i, line) in lines.iter().enumerate() {
                let (start_horizontal_position, end_horizontal_position) =
                    self.horizontal_text_position(line.to_string(), horizontal_position)?;

                let line_index = start_vertical_position + i * line_seperation + i;
                split_content[line_index]
                    .replace_range(start_horizontal_position..end_horizontal_position, &line);
            }
        }
        &TextFormats::SingleLine(ref line) => {
            let (start_horizontal_position, end_horizontal_position) =
                self.horizontal_text_position(line.to_string(), horizontal_position)?;
            split_content[start_vertical_position]
                .replace_range(start_horizontal_position..end_horizontal_position, &line);
        }
    }

    self.content = split_content.join("");

    Ok(())
}

fn split_contents(&self, input: &String, chunk_size: usize) -> Vec<String> {
    input
        .as_bytes()
        .chunks(chunk_size)
        .map(|str| std::str::from_utf8(str).unwrap().to_string())
        .collect::<Vec<String>>()
}

fn horizontal_text_position(
    &self,
    text: String,
    position: InsertHorizontalPosition,
) -> Result<(usize, usize), RenderError> {
    let text_len = text.len();
    if text_len > self.width {
        return Err(RenderError::AddTextError("String is too long!"));
    }
    let position = match position {
        InsertHorizontalPosition::Exact(pos) => {
            if pos + text_len > self.width {
                return Err(RenderError::AddTextError(
                    "Horizontal position overflows terminal size",
                ));
            }
            (pos, pos + text_len)
        }
        InsertHorizontalPosition::Center => {
            let center_position = self.width / 2;
            let start_position = center_position - text_len / 2;
            (start_position, start_position + text_len)
        }
        InsertHorizontalPosition::Right => {
            let start_position = self.width - text_len;
            (start_position, self.width)
        }
    };

    Ok(position)
}

fn vertical_text_position(
    &self,
    text: &TextFormats,
    position: InsertVerticalPosition,
    line_seperation: usize,
) -> Result<usize, RenderError> {
    let text_height = match text {
        TextFormats::MultiLine(multi) => multi.len(),
        TextFormats::SingleLine(_) => 1,
    };
    if text_height > self.height {
        return Err(RenderError::AddTextError(
            "Text has too many lines for screen",
        ));
    }

    let text_height_with_padding = text_height + (text_height - 1) * line_seperation;

    let vertical_start_position = match position {
        InsertVerticalPosition::Bottom => self.height - text_height_with_padding,
        InsertVerticalPosition::Center => (self.height - text_height_with_padding) / 2,
        InsertVerticalPosition::Exact(position) => position,
    };

    if text_height_with_padding + vertical_start_position > self.height {
        return Err(RenderError::AddTextError(
            "Text and settings overflow screen height",
        ));
    }
    Ok(vertical_start_position)
} */
