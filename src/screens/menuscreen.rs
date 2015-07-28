use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use screens::{GameScreen, ScreenType};
use game::{Update, GameInput, Render};

pub struct MenuScreen;

impl GameScreen for MenuScreen {
    fn new() -> Box<Self> where Self: Sized + GameScreen {Box::new(MenuScreen)}

    fn get_type(&self) -> ScreenType { ScreenType::Menu }
}

impl Update for MenuScreen {
    fn update(&mut self, _args: &UpdateArgs) {
    }
}

impl GameInput for MenuScreen {
    fn input(&mut self, _args: &Input) {
    }
}

impl Render for MenuScreen {
    fn render(&mut self, _gl: &mut GlGraphics, _args: &RenderArgs) {
    }
}

