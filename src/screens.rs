#![allow(dead_code)]

use std::any::Any;

struct MenuScreen;
struct OverworldScreen;

pub trait GameScreen {
    fn new(_: Vec<Box<Any>>) -> Self;
}

impl GameScreen for MenuScreen {
    fn new(_: Vec<Box<Any>>) -> MenuScreen { MenuScreen }
}
impl GameScreen for OverworldScreen {
    fn new(_: Vec<Box<Any>>) -> OverworldScreen { OverworldScreen }
}

macro_rules! enum_map(
    ($nm:ident : $idx_ty:ty {
        $($brnch:ident($inner_ty:ident) : ($key_nm:ident = $idx:expr)),*
    }) => (
        $(pub const $key_nm: $idx_ty = $idx;)*
        enum_map!( $nm : $idx_ty {
            $($brnch($inner_ty) : $idx),*
        });
    );

    ($nm:ident : $idx_ty:ty {
        $($brnch:ident($inner_ty:ident) : $idx:expr),*
    }) => (
        pub enum $nm {
            $($brnch($inner_ty)),*
        }

        impl $nm {
            pub fn get_current_branch(&self) -> $idx_ty {
                match self {
                    $(&$nm::$brnch($inner_ty) => $idx),*
                }
            }

            pub fn set_branch(&mut self, index: $idx_ty, args: Vec<Box<Any>>) {
                match index {
                    $($idx => *self = $nm::$brnch($inner_ty::new(args))),*
                    , _ => { }
                }
            }
        }
    );
);

enum_map!( GameScreens: usize {
    MainMenu(MenuScreen): (MENU_SCREEN_KEY = 0),
    Overworld(OverworldScreen): (OVERWORLD_SCREEN_KEY = 1)
});

impl GameScreens {
    fn new() -> GameScreens { GameScreens::MainMenu(MenuScreen) }
    fn input(&mut self) {}
    fn update(&mut self) {}
    fn render(&mut self) {}
}
