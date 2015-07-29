#![allow(dead_code)]

mod menuscreen;
mod overworld;

use game::{Update, GameInput, Render};
use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use self::menuscreen::MenuScreen;
use self::overworld::OverworldScreen;
use std::any::Any;

pub trait GameScreen: Update + GameInput + Render {
    fn new() -> Self where Self: Sized + GameScreen;

    fn with_args(_: Vec<Box<Any>>) -> Option<Self> where Self: Sized + GameScreen {
        None
    }
}

macro_rules! enum_map(
    ($nm:ident : $idx_ty:ty => $key_ty:ty {
        $(($key_nm:ident = $idx:expr) => $brnch:ident($inner_ty:ident)),*
    }) => (
        $(pub const $key_nm: $idx_ty = $idx;)*
        enum_map!( $nm : $idx_ty => $key_ty {
            $($idx => $brnch($inner_ty)),*
        });
    );

    ($nm:ident : $idx_ty:ty => $key_ty:ty {
        $($idx:expr => $brnch:ident($inner_ty:ident)),*
    }) => (
        enum $nm {
            $($brnch($inner_ty)),*
        }

        impl $nm {
            fn get_current_branch_index(&self) -> $idx_ty {
                match self {
                    $(&$nm::$brnch(_) => $idx),*
                }
            }

            fn get_current_branch(&self) -> &$key_ty {
                match self {
                    $(&$nm::$brnch(ref inner) => &*inner as &$key_ty),*
                }
            }

            fn get_current_branch_mut(&mut self) -> &mut $key_ty {
                match self {
                    $(&mut $nm::$brnch(ref mut inner) => &mut *inner as &mut $key_ty),*
                }
            }

            fn set_branch_with_args(&mut self, index: &$idx_ty, args: Vec<Box<Any>>) {
                match index {
                    $(&$idx => *self = $nm::$brnch($inner_ty::with_args(args).unwrap())),*
                    , _ => { }
                }
            }

            fn set_branch(&mut self, index: &$idx_ty) {
                match index {
                    $(&$idx => *self = $nm::$brnch($inner_ty::new())),*
                    , _ => { }
                }
            }
        }
    );
);

enum_map!( GameScreensInner: usize => GameScreen {
    (MENU_SCREEN_KEY = 0) => MainMenu(MenuScreen),
    (OVERWORLD_SCREEN_KEY = 1) => Overworld(OverworldScreen)
});

pub struct GameScreens {
    inner: GameScreensInner
}

impl GameScreens {
    pub fn new() -> GameScreens {
        GameScreens { inner: GameScreensInner::MainMenu(MenuScreen::new()) }
    }

    pub fn get_current_screen_index(&self) -> usize {
        self.inner.get_current_branch_index()
    }

    pub fn get_current_screen(&self) -> &GameScreen {
        self.inner.get_current_branch()
    }

    pub fn get_current_screen_mut(&mut self) -> &mut GameScreen {
        self.inner.get_current_branch_mut()
    }

    pub fn set_screen(&mut self, index: usize) {
        self.inner.set_branch(&index);
    }

    pub fn set_screen_with_args(&mut self, index: usize, args: Vec<Box<Any>>) {
        self.inner.set_branch_with_args(&index, args);
    }
}

impl GameInput for GameScreens {
    fn input(&mut self, iput: &Input) {
        use piston::input::Button;
        use piston::input::keyboard::Key;

        self.get_current_screen_mut().input(iput);
        if let &Input::Press(Button::Keyboard(Key::Space)) = iput {
            let other_branch = match self.get_current_screen_index() {
                                   MENU_SCREEN_KEY => OVERWORLD_SCREEN_KEY,
                                   OVERWORLD_SCREEN_KEY => MENU_SCREEN_KEY,
                                   _ => 0
                               };
            self.set_screen(other_branch);
        }
    }
}

impl Update for GameScreens {
    fn update(&mut self, args: &UpdateArgs) {
        self.get_current_screen_mut().update(args);
    }
}

impl Render for GameScreens {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.get_current_screen_mut().render(args, gl);
    }
}
