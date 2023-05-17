use std::io::{self, stdout};

use crossterm::{cursor, execute, terminal};

pub struct Render {
    content: EditorContents,
    width: usize,
    height: usize,
}

impl Render {
    pub fn new() -> Result<Self, RenderError> {
        let (term_width, term_height) = crossterm::terminal::size().unwrap();
        let width = term_width as usize;
        let height = term_height as usize - 1;
        if width < 10 && height < 10 {
            return Err(RenderError::TerminalSizeError(
                "Terminal size is too small!",
            ));
        }
        Ok(Self {
            content: " ".repeat(width * height).to_string(),
            width,
            height,
        })
    }

    fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn render(&self) -> io::Result<()> {
        self.clear_screen();

        for line in &self.content {
            println!("{line}");
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
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

struct EditorContents {
    content: String,
    width: usize,
    height: usize,
}

impl EditorContents {
    fn new(width: usize, height: usize) -> Self {
        EditorContents {
            content: String::new(),
            width,
            height,
        }
    }

    fn set_contents(&mut self, string: String) {
        self.content = string;
    }

    pub fn add_text(
        &mut self,
        text: TextFormats,
        horizontal_position: InsertHorizontalPosition,
        vertical_position: InsertVerticalPosition,
        line_seperation: usize,
    ) -> Result<(), RenderError> {
        let split_content = self.split_contents(self.content, self.width);

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

    fn split_contents(&self, input: String, chunk_size: usize) -> Vec<String> {
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
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.set_contents(s.to_string());
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
