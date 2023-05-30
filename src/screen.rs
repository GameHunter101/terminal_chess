use std::collections::HashMap;
use std::io::{self, stdout, Write};

use crate::chess::Board;
use crate::Render;

use crossterm::style::Stylize;
use crossterm::{queue, terminal};
use regex::Regex;

pub struct Screen {
    content: String,
    pub screen_rows: ScreenRows,
    pub width: usize,
    pub height: usize,
    pub button_map: HashMap<&'static str, Box<dyn Fn()>>,
    pub game: Option<Board>,
}

impl Screen {
    pub fn new(button_map: HashMap<&'static str, Box<dyn Fn()>>, board: Option<Board>) -> Self {
        let (term_width, term_height) = terminal::size().unwrap();
        /* let mut button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
        button_map.insert("next_screen", Box::new(||{render.set_screen(1)})); */
        Self {
            content: String::new(),
            screen_rows: ScreenRows::new(term_width as usize, term_height as usize),
            width: term_width as usize,
            height: term_height as usize,
            button_map,
            game: board,
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
            self.push_str(&row.join(""));

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
    pub rows: Vec<Vec<String>>,
    pub buttons: Vec<Vec<ButtonText>>,
    width: usize,
    height: usize,
}

impl ScreenRows {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: vec![vec![" ".to_string(); width]; height],
            buttons: vec![Vec::new(); height],
            width,
            height,
        }
    }

    // Splits a given string into characters, appends ANSI formatting on first and last characters
    pub fn split_ansi_string(text: String, text_len: usize) -> Vec<String> {
        let escape_indices = text
            .match_indices("\u{1b}")
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        if escape_indices.len() > 1 {
            let back_escape_index = escape_indices[escape_indices.len() / 2];
            let front_ansi: String = (&text.chars().collect::<Vec<char>>()
                [..back_escape_index - text_len])
                .iter()
                .collect();
            let back_ansi: String = (&text.chars().collect::<Vec<char>>()[back_escape_index..])
                .iter()
                .collect();

            let chars: Vec<char> = (&text.chars().collect::<Vec<char>>()
                [back_escape_index - text_len..back_escape_index])
                .to_vec();
            let mut strings: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
            let last_index = &strings.len() - 1;
            let mut first_element = front_ansi;
            first_element.push_str(&strings[0]);
            strings[0] = first_element;
            let mut last_element = strings[last_index].clone();
            last_element.push_str(&back_ansi);
            strings[last_index] = last_element;
            strings
        } else {
            let chars: Vec<char> = text.to_string().chars().collect();
            let strings: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
            strings
        }
    }

    pub fn edit_single_row(&mut self, text: Text) {
        let text_len = text.length();

        let split_text = ScreenRows::split_ansi_string(text.text(), text_len);

        for (i, str) in split_text.iter().enumerate() {
            self.rows[text.position_y()][i + text.position_x()] = str.to_string();
        }

        match text {
            Text::Button(button) => self.buttons[button.position_y].push(button),
            _ => {}
        };
    }

    pub fn clear_row(&mut self, row: InsertVerticalPosition) {
        let row_num = match row {
            InsertVerticalPosition::Bottom => self.height - 1,
            InsertVerticalPosition::Center => self.height / 2,
            InsertVerticalPosition::Exact(num) => num,
        };
        self.rows[row_num] = vec![" ".to_string(); self.width];
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

#[derive(Clone, Debug)]
pub enum Text {
    Plain(PlainText),
    Button(ButtonText),
}

impl Text {
    pub fn new(
        text: String,
        position_x: usize,
        position_y: usize,
        on_click: Option<&'static str>,
    ) -> Self {
        let re = Regex::new("\u{1b}\\[[^m]+m").unwrap();
        let clean_string = re.replace_all(&text, "");
        let length = clean_string.chars().count();
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

#[derive(Clone, Debug)]
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
        let re = Regex::new("\u{1b}\\[[^m]+m").unwrap();
        let clean_string = re.replace_all(&text, "");
        let text_len = clean_string.chars().count();
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

#[derive(Clone, Debug)]
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
        let re = Regex::new("\u{1b}\\[[^m]+m").unwrap();
        let clean_string = re.replace_all(&text, "");
        let text_len = clean_string.chars().count();
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
