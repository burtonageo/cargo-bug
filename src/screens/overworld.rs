use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use screens::GameScreen;
use game::{GameInput, Update, Render};

pub struct OverworldScreen;

impl GameScreen for OverworldScreen {
    fn new() -> Box<Self> where Self: Sized + GameScreen {Box::new(OverworldScreen)}
}

impl Update for OverworldScreen {
    fn update(&mut self, _args: &UpdateArgs) {
    }
}

impl GameInput for OverworldScreen {
    fn input(&mut self, _args: &Input) {
    }
}

impl Render for OverworldScreen {
    fn render(&mut self, _args: &RenderArgs, _gl: &mut GlGraphics) {
    }
}
