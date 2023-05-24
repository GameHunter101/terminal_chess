use std::collections::HashMap;
use std::io::{self, stdout, Write};

use crate::Render;

use crossterm::{queue, terminal};

pub struct Screen {
    content: String,
    pub screen_rows: ScreenRows,
    pub width: usize,
    pub height: usize,
    pub button_map: HashMap<&'static str, Box<dyn Fn()>>,
}

impl Screen {
    pub fn new(button_map: HashMap<&'static str, Box<dyn Fn()>>) -> Self {
        let (term_width, term_height) = terminal::size().unwrap();
        /* let mut button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
        button_map.insert("next_screen", Box::new(||{render.set_screen(1)})); */
        Self {
            content: String::new(),
            screen_rows: ScreenRows::new(term_width as usize, term_height as usize),
            width: term_width as usize,
            height: term_height as usize,
            button_map,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }

    pub fn compile_screen(&mut self) {
        // self.content.clear();
        let rows = self.screen_rows.rows.clone();
        for row in rows {
            // println!("Row: {}",row);
            self.push_str(&row);

            queue!(self, terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
        }
        stdout().flush();
    }
}

impl io::Write for Screen {
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

pub struct ScreenRows {
    pub rows: Vec<String>,
    pub buttons: Vec<Vec<ButtonText>>,
    width: usize,
    height: usize,
}

impl ScreenRows {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: vec![" ".to_string().repeat(width); height],
            buttons: vec![Vec::new(); height],
            width,
            height,
        }
    }

    pub fn edit_single_row(&mut self, text: Text) {
        let text_len = text.length();

        self.rows[text.position_y()].replace_range(
            text.position_x()..text.position_x() + text_len,
            &text.text(),
        );


        match text {
            Text::Button(button) => self.buttons[button.position_y].push(button),
            _ => {}
        };
    }

    pub fn edit_multiple_rows(
        &mut self,
        text: &[Text],
        row_spacing: usize,
        vertical_position: InsertVerticalPosition,
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
            let line_text = line.text();

            let text_object = Text::new(
                line_text,
                line.position_x(),
                line.position_y(),
                match line {
                    Text::Plain(_) => None,
                    Text::Button(button) => Some(button.on_click),
                },
            );
            self.edit_single_row(text_object);
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

#[derive(Clone)]
pub enum Text {
    Plain(PlainText),
    Button(ButtonText),
}

impl Text {
    fn new(
        text: String,
        position_x: usize,
        position_y: usize,
        on_click: Option<&'static str>,
    ) -> Self {
        let length = (&text).len();
        match on_click {
            None => Self::Plain(PlainText {
                text,
                position_x,
                position_y,
                length,
            }),
            Some(on_click) => Self::Button(ButtonText {
                text,
                position_x,
                position_y,
                length,
                on_click,
            }),
        }
    }
}

pub trait TextContent {
    fn text(&self) -> String;
    fn position_x(&self) -> usize;
    fn position_y(&self) -> usize;
    fn length(&self) -> usize;
}

impl TextContent for Text {
    fn text(&self) -> String {
        match self {
            Self::Plain(plain) => plain.text.clone(),
            Self::Button(button) => button.text.clone(),
        }
    }
    fn position_x(&self) -> usize {
        match self {
            Self::Plain(plain) => plain.position_x,
            Self::Button(button) => button.position_x,
        }
    }
    fn position_y(&self) -> usize {
        match self {
            Self::Plain(plain) => plain.position_y,
            Self::Button(button) => button.position_y,
        }
    }
    fn length(&self) -> usize {
        match self {
            Self::Plain(plain) => plain.length,
            Self::Button(button) => button.length,
        }
    }
}

#[derive(Clone)]
pub struct PlainText {
    pub text: String,
    pub position_x: usize,
    pub position_y: usize,
    pub length: usize,
}

impl PlainText {
    pub fn new(
        text: String,
        screen_width: usize,
        screen_height: usize,
        horizontal_position: InsertHorizontalPosition,
        vertical_position: InsertVerticalPosition,
    ) -> Self {
        let text_len = text.len();
        let horizontal_start_position = match horizontal_position {
            InsertHorizontalPosition::Exact(pos) => pos,
            InsertHorizontalPosition::Center => (screen_width - text_len) / 2,
            InsertHorizontalPosition::Right => screen_width - text_len,
        };
        let vertical_start_position = match vertical_position {
            InsertVerticalPosition::Exact(pos) => pos,
            InsertVerticalPosition::Center => screen_height / 2,
            InsertVerticalPosition::Bottom => screen_height - 1,
        };
        // let temp: & str = text.as_str();
        Self {
            text,
            position_x: horizontal_start_position,
            position_y: vertical_start_position,
            length: text_len,
        }
    }

    pub fn from_multi_lines(
        text: String,
        screen_width: usize,
        screen_height: usize,
        horizontal_position: InsertHorizontalPosition,
    ) -> Vec<Text> {
        text.split("\n")
            .enumerate()
            .map(|(i, line)| {
                let text_len = line.len();
                let horizontal_start_position = match horizontal_position {
                    InsertHorizontalPosition::Exact(pos) => pos,
                    InsertHorizontalPosition::Center => (screen_width - text_len) / 2,
                    InsertHorizontalPosition::Right => screen_width - text_len - 1,
                };
                Text::Plain(Self {
                    text: line.to_string(),
                    position_x: horizontal_start_position,
                    position_y: i,
                    length: text_len,
                })
            })
            .collect()
    }
}

impl std::default::Default for PlainText {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            position_x: 0,
            position_y: 0,
            length: 0,
        }
    }
}

#[derive(Clone)]
pub struct ButtonText {
    pub text: String,
    pub position_x: usize,
    pub position_y: usize,
    pub length: usize,
    pub on_click: &'static str,
}

impl ButtonText {
    pub fn new(
        text: String,
        screen_width: usize,
        screen_height: usize,
        horizontal_position: InsertHorizontalPosition,
        vertical_position: InsertVerticalPosition,
        on_click: &'static str,
    ) -> Self {
        let text_len = text.len();
        let horizontal_start_position = match horizontal_position {
            InsertHorizontalPosition::Exact(pos) => pos,
            InsertHorizontalPosition::Center => (screen_width - text_len) / 2,
            InsertHorizontalPosition::Right => screen_width - text_len - 1,
        };
        let vertical_start_position = match vertical_position {
            InsertVerticalPosition::Exact(pos) => pos,
            InsertVerticalPosition::Center => screen_height / 2,
            InsertVerticalPosition::Bottom => screen_height - 1,
        };
        Self {
            text,
            position_x: horizontal_start_position,
            position_y: vertical_start_position,
            length: text_len,
            on_click,
        }
    }
}

impl std::default::Default for ButtonText {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            position_x: 0,
            position_y: 0,
            length: 0,
            on_click: "",
        }
    }
}
