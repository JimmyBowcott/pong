use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use std::io::{Write, stdout};

pub trait Renderer {
    fn clear(&self);
    fn present(&mut self);
    fn put_char(&mut self, x: usize, y: usize, ch: char);
}

pub struct TerminalRenderer {
    buffer: Vec<(u16, u16, char)>,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&self) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }

    fn put_char(&mut self, x: usize, y: usize, ch: char) {
        self.buffer.push((x as u16, y as u16, ch));
    }

    fn present(&mut self) {
        let mut stdout = stdout();
        for (x, y, ch) in &self.buffer {
            execute!(stdout, cursor::MoveTo(*x, *y)).unwrap();
            print!("{}", ch);
        }
        stdout.flush().unwrap();
        self.buffer.clear(); // clear after presenting
    }
}
