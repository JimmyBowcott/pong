use std::collections::HashSet;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Quit,
}

pub struct InputController {
    pressed: HashSet<Key>,
}

impl InputController {
    pub fn new() -> Self {
        Self { pressed: HashSet::new() }
    }

    pub fn poll(&mut self) {
        self.pressed.clear();

        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Up => { self.pressed.insert(Key::Up); }
                    KeyCode::Down => { self.pressed.insert(Key::Down); }
                    KeyCode::Char('q') => { self.pressed.insert(Key::Quit); }
                    _ => {}
                }
            }
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
}
