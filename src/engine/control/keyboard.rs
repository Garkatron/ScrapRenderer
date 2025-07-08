use std::{cell::RefCell, rc::Rc};

use minifb::{Key, Window, KeyRepeat};

pub struct KeyboardController {
    window: Rc<RefCell<Window>>,
}

impl KeyboardController {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        KeyboardController { window }
    }

    pub fn get_pressed_keys(&self) -> Vec<Key> {
        self.window.borrow().get_keys()
    }

    pub fn get_newly_pressed_keys(&self) -> Vec<Key> {
        self.window.borrow().get_keys_pressed(KeyRepeat::No)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.borrow().is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.get_newly_pressed_keys().contains(&key)
    }
}