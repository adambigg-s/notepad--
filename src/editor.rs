use std::{
    fmt::{self, Display},
    fs,
    io::{self, BufRead, Write},
    process,
};

use crossterm::event;

use crate::text::{self};

#[derive(Default)]
pub enum EditorMode {
    #[default]
    Visual,
    Insert,
}

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    fn move_cursor(&mut self, dx: isize, dy: isize, text: &text::Text) {
        let nx = (self.col as isize + dx) as usize;
        let ny = (self.row as isize + dy) as usize;

        let num_lines = text.lines.len();
        let num_chars = text.lines.get(self.row).line.len();

        if nx < num_chars {
            self.col = nx;
        }
        if ny < num_lines {
            self.row = ny;
        }
    }
}

#[derive(Default)]
pub struct Editor {
    pub text: text::Text,
    pub cursor: Cursor,
    pub mode: EditorMode,
}

impl Editor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_file(&mut self, file: fs::File) {
        let reader = io::BufReader::new(file);
        reader.lines().map_while(Result::ok).for_each(|line| {
            let mut buff = text::LinkedList::new();
            line.chars().for_each(|char| {
                buff.push_back(char);
            });
            self.text.lines.push_back(text::Line { line: buff });
        });
    }

    pub fn update_screen(&self) {
        println!("{}", self)
    }

    pub fn update_with(&mut self, key: event::KeyEvent) {
        match self.mode {
            | EditorMode::Visual => self.handle_visual(key),
            | EditorMode::Insert => self.handle_insert(key),
        }
    }

    fn handle_visual(&mut self, key: event::KeyEvent) {
        match key.code {
            | event::KeyCode::Char('q') => process::exit(0),
            | event::KeyCode::Char('i') => self.mode = EditorMode::Insert,
            | event::KeyCode::Char('h') => self.cursor.move_cursor(-1, 0, &self.text),
            | event::KeyCode::Char('j') => self.cursor.move_cursor(0, 1, &self.text),
            | event::KeyCode::Char('k') => self.cursor.move_cursor(0, -1, &self.text),
            | event::KeyCode::Char('l') => self.cursor.move_cursor(1, 0, &self.text),
            | _ => {}
        }
    }

    fn handle_insert(&mut self, key: event::KeyEvent) {
        match key.code {
            | event::KeyCode::Esc => self.mode = EditorMode::Visual,
            | event::KeyCode::Char(chr) => {
                self.text.insert_at(self.cursor, chr);
                self.cursor.move_cursor(1, 0, &self.text);
            }
            event::KeyCode::Backspace => {
                self.text.remove_at(self.cursor);
                self.cursor.move_cursor(-1, 0, &self.text);
            }
            | _ => {}
        }
    }
}

impl Display for Editor {
    fn fmt(&self, frm: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("\x1b[0H");
        self.text.lines.iter().for_each(|line| {
            line.line.iter().for_each(|char| {
                output.push(*char);
            });
            output.push('\n');
        });
        write!(frm, "{}", output)?;
        write!(frm, "\x1b[{};{}H#", self.cursor.row + 1, self.cursor.col + 1)?;
        io::stdout().flush().unwrap();
        Ok(())
    }
}
