#![allow(dead_code, unused_variables)]

use piston::input::{Input, Button, Motion};
use std::collections::HashMap;
use std::hash::Hash;

pub trait Action: Copy + Eq + Hash + PartialEq { }

#[derive(Debug, Copy, Clone)]
pub enum Translated<A: Action> {
    Press(A),
    Release(A),
    Move(Motion)
}

pub struct InputMap<A: Action> {
    keymap: KeyMap<A>,
    mouse_translator: MouseTranslator
}

impl<A: Action> InputMap<A> {
    pub fn new() -> Self {
        InputMap {
            keymap: KeyMap::new(),
            mouse_translator: MouseTranslator::new()
        }
    }

    pub fn translate(&self, input: &Input) -> Option<Translated<A>> {
        macro_rules! translate_button(
            ($but_state:ident, $but_var:ident) => ({
                match self.keymap.translate($but_var) {
                    Some(act) => Some(Translated::$but_state(act)),
                    None => None
                }
            });
        );

        match input {
            &Input::Press(button)   => translate_button!(Press, button),
            &Input::Release(button) => translate_button!(Release, button),
            &Input::Move(motion)    => {
                Some(Translated::Move(self.mouse_translator.translate(motion)))
            },
            _ => None
        }
    }

    pub fn rebind_button(&mut self, but: Button, act: A) {
        
    }

    pub fn add_binding(&mut self, but: Button, act: A) {
        
    }

    pub fn get_bindings_for_action(&self, act: A) -> Vec<Button> {
        Vec::new()
    }
}

struct MouseTranslator {
    pub x_axis_inverted: bool,
    pub y_axis_inverted: bool
}

impl MouseTranslator {
    fn new() -> Self {
        MouseTranslator {
            x_axis_inverted: false,
            y_axis_inverted: false
        }
    }

    fn translate(&self, motion: Motion) -> Motion {
        motion
    }
}

struct KeyMap<A: Action> {
    btn_map: HashMap<A, Vec<Button>>
}

impl<A: Action> KeyMap<A> {
    fn new() -> Self {
        KeyMap {
            btn_map: HashMap::new()
        }
    }

    fn add_mapping(&mut self, button: Button, action: A) {
        // ...
    }

    fn with_mapping(self, button: Button, action: A) -> Self {
        self
    }

    fn translate(&self, button: Button) -> Option<A> {
        None
    }
}
