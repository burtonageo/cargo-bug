#![allow(dead_code, unused_imports, unused_variables)]

use piston::input::keyboard::Key;
use std::marker::PhantomData;

struct KeyMap <A: Action> {
    ph: PhantomData<A>
}
trait Action: Sized + Copy { }

impl<A: Action> KeyMap<A> {
    fn add_mapping(&mut self, (key, action): (Key, A)) {
        // ...
    }

    fn with_mapping(self, (key, action): (Key, A)) -> KeyMap<A> {
        self
    }

    fn translate(&self, key: &Key) -> Option<A> {
        None
    }
}
