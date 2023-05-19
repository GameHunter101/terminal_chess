use std::io::{self, stdout, Write};

use crossterm::{queue, terminal};

pub struct EditorContents {
    content: String,
    pub row_contents: RowContents,
    height: usize,
}

impl EditorContents {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            content: String::new(),
            row_contents: RowContents::new(width, height),
            height,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }

    pub fn rows_to_string(&mut self) {
        // self.content.clear();
        let rows = self.row_contents.rows.clone();
        for i in 0..self.height {
            self.push_str(&rows[i]);

            queue!(self, terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
        }
        stdout().flush();
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
        // self.rows_to_string();
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

pub struct RowContents {
    pub rows: Vec<String>,
	pub buttons: Vec<Vec<ButtonText>>,
    width: usize,
    height: usize,
}

impl RowContents {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: vec![" ".to_string().repeat(width); height],
			buttons: vec![vec![];height],
            width,
            height,
        }
    }

    pub fn edit_single_row(&mut self, text: Text) {
        let text = match text {
            Text::Plain(plain_text) => Box::new(plain_text) as Box<dyn TextContent>,
            Text::Button(button_text) => {
				self.buttons[button_text.position_y()].push(button_text);
				Box::new(button_text) as Box<dyn TextContent>
			},
        };
        let text_len = text.length();

        self.rows[text.position_y()].replace_range(
            text.position_x()..text.position_x() + text_len,
            &text.text(),
        );
    }

    pub fn edit_multiple_rows(
        &mut self,
        text: Vec<Text>,
        row_spacing: usize,
        vertical_position: InsertVerticalPosition,
        horizontal_position: InsertHorizontalPosition,
    ) {
        let plain_text_height = text.len();
        let full_text_height = plain_text_height + (plain_text_height - 1);
        let start_vertical_position = match vertical_position {
            InsertVerticalPosition::Exact(pos) => pos,
            InsertVerticalPosition::Center => (self.height - full_text_height) / 2,
            InsertVerticalPosition::Bottom => self.height - full_text_height,
        };

        for (i, line) in text.iter().enumerate() {
            let row_number = start_vertical_position + i + i * row_spacing;
            let line = match *line {
                Text::Plain(plain_text) => Box::new(plain_text) as Box<dyn TextContent>,
                Text::Button(button_text) => Box::new(button_text) as Box<dyn TextContent>,
            };
            self.edit_single_row(Text::Plain(PlainText::new(
                line.text(),
                self.width,
                horizontal_position,
                row_number,
            )));
        }
    }
}

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

pub enum Text {
    Plain(PlainText),
    Button(ButtonText),
}

pub trait TextContent {
    fn text(&self) -> &'static str;
    fn position_x(&self) -> usize;
    fn position_y(&self) -> usize;
    fn length(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct PlainText {
    text: &'static str,
    position_x: usize,
    position_y: usize,
    length: usize,
}

impl PlainText {
    pub fn new(
        text: &'static str,
        screen_width: usize,
        horizontal_position: InsertHorizontalPosition,
        vertical_position: usize,
    ) -> Self {
        let text_len = text.len();
        let start_position = match horizontal_position {
            InsertHorizontalPosition::Exact(pos) => pos,
            InsertHorizontalPosition::Center => (screen_width - text_len) / 2,
            InsertHorizontalPosition::Right => screen_width - text_len - 1,
        };
        Self {
            text,
            position_x: start_position,
            position_y: vertical_position,
            length: text_len,
        }
    }
}

impl TextContent for PlainText {
    fn text(&self) -> &'static str {
        self.text
    }
    fn position_x(&self) -> usize {
        self.position_x
    }
    fn position_y(&self) -> usize {
        self.position_y
    }
    fn length(&self) -> usize {
        self.length
    }
}

#[derive(Clone, Copy,Debug)]
pub struct ButtonText {
    text: &'static str,
    position_x: usize,
    position_y: usize,
    length: usize,
    pub on_click: fn(usize, usize),
}

impl ButtonText {
	pub fn new(
        text: &'static str,
        screen_width: usize,
        horizontal_position: InsertHorizontalPosition,
        vertical_position: usize,
		callback: fn(usize,usize)
    ) -> Self {
        let text_len = text.len();
        let start_position = match horizontal_position {
            InsertHorizontalPosition::Exact(pos) => pos,
            InsertHorizontalPosition::Center => (screen_width - text_len) / 2,
            InsertHorizontalPosition::Right => screen_width - text_len - 1,
        };
        Self {
            text,
            position_x: start_position,
            position_y: vertical_position,
            length: text_len,
			on_click: callback,
        }
    }
}

impl TextContent for ButtonText {
    fn text(&self) -> &'static str {
        self.text
    }
    fn position_x(&self) -> usize {
        self.position_x
    }
    fn position_y(&self) -> usize {
        self.position_y
    }
    fn length(&self) -> usize {
        self.length
    }
}
