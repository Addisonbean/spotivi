use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub type KeyBindings = HashMap<KeyEvent, KeyBinding>;

#[derive(Copy, Clone)]
pub enum KeyBinding {
    Up,
    Down,
    // Left,
    // Right,
    Quit,
}

fn char_event(c: char, mods: Option<KeyModifiers>) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), mods.unwrap_or(KeyModifiers::NONE))
}

pub fn default_keybindings(keys: &mut KeyBindings) {
    keys.insert(char_event('q', None), KeyBinding::Quit);
    keys.insert(char_event('k', None), KeyBinding::Up);
    keys.insert(char_event('j', None), KeyBinding::Down);
}
