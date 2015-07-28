use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use screens::GameScreen;
use game::{Update, GameInput, Render};

pub struct MenuScreen;

impl GameScreen for MenuScreen {
    fn new() -> Self where Self: Sized + GameScreen {MenuScreen}
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
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::clear;
        gl.draw(args.viewport(), |_, gl| { clear([0.0, 0.0, 1.0, 1.0], gl); });
    }
}
