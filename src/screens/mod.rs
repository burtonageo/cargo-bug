#![allow(dead_code)]

mod menuscreen;
mod overworld;

use game::{Update, GameInput, Render};
use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use self::menuscreen::MenuScreen;
use std::any::Any;

pub trait GameScreen: Update + GameInput + Render {
    fn new() -> Box<Self> where Self: Sized + GameScreen;

    fn with_args(_: Vec<Box<Any>>) -> Option<Box<Self>> where Self: Sized + GameScreen {
        None
    }
}

pub struct GameScreens {
    screen: Box<GameScreen>
}

impl GameScreens {
    pub fn new() -> GameScreens {
        GameScreens {
            screen: MenuScreen::new() as Box<GameScreen>,
        }
    }

    pub fn set_active_screen(&mut self, screen: Box<GameScreen>) {
        self.screen = screen;
    }
}

impl GameInput for GameScreens {
    fn input(&mut self, iput: &Input) {
        self.screen.input(iput);
    }
}

impl Update for GameScreens {
    fn update(&mut self, args: &UpdateArgs) {
        self.screen.update(args);
    }
}

impl Render for GameScreens {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.screen.render(args, gl);
    }
}
