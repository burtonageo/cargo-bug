#![feature(const_fn)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate simplecolor;
extern crate nalgebra as na;

mod game;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;

use game::Game;

fn main() {
    const OPENGL: OpenGL = OpenGL::V3_2;

    let window = GlutinWindow::from(WindowSettings::new("calaxite", (800, 600))
                                        .opengl(OPENGL)
                                        .fullscreen(false)
                                        .exit_on_esc(true));

    let mut game = Game::new(OPENGL, window);

    game.run_loop();
}