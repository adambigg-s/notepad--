use std::{
    fmt::{self, Display},
    fs,
    io::{self, BufRead, Write},
    process,
};

use crossterm::event;

use crate::text::{self, Text};

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Default)]
pub enum EditorMode {
    #[default]
    Visual,
    Insert,
}

#[derive(Default)]
pub struct Editor {
    pub text: Text,
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
            | event::KeyCode::Char('h') => todo!(),
            | event::KeyCode::Char('j') => todo!(),
            | event::KeyCode::Char('k') => todo!(),
            | event::KeyCode::Char('l') => todo!(),
            | _ => {}
        }
    }

    fn handle_insert(&mut self, key: event::KeyEvent) {
        match key.code {
            | event::KeyCode::Esc => self.mode = EditorMode::Visual,
            | event::KeyCode::Char(chr) => {
                self.text.insert_at(self.cursor, chr);
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
        io::stdout().flush().unwrap();
        Ok(())
    }
}
