mod editor;
mod text;

use std::{
    env,
    fs::{self},
};

use crossterm::event;

use crate::editor::Editor;

fn main() {
    let mut editor = Editor::new();

    let mut envs = env::args();
    let path = envs.nth(1).expect("no target file");
    let file = fs::File::open(&path).expect("failed to find file");

    editor.read_file(file);

    loop {
        if let Ok(event::Event::Key(key)) = event::read() {
            editor.update_with(key);
        }
        editor.update_screen();
    }
}
