use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Event, Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventMap, Events};
use std::cell::RefCell;
use std::rc::Rc;
use screens::GameScreens;
use world::World;

pub struct Game {
    gl: GlGraphics,
    window: Rc<RefCell<GlutinWindow>>,
    world: World
}

pub trait Update {
    fn update(&mut self, _: &UpdateArgs);
}

pub trait GameInput {
    fn input(&mut self, _: &Input);
}

pub trait Render {
    fn render(&mut self, _: &mut GlGraphics, _: &RenderArgs);
}

impl Game {
    pub fn new(opengl: OpenGL, win: GlutinWindow) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            window: Rc::new(RefCell::new(win)),
            world: World::new()
        }
    }

    pub fn run_loop(&mut self) {
        for e in self.window.clone().events() {
            match e {
                Event::Render(r) => {
                    self.render(&r);
                },
                Event::Update(u) => {
                    self.update(&u);
                },
                Event::Input(i) => {
                    self.input(&i);
                },
                _ => { }
            }
        }
    }

    // Different signature because Game owns the GlGraphics
    fn render(&mut self, args: &RenderArgs) {
        self.world.render(&mut self.gl, args);
    }
}

impl Update for Game {
    fn update(&mut self, args: &UpdateArgs) {
        self.world.update(args);
    }
}

impl GameInput for Game {
    fn input(&mut self, args: &Input) {
        self.world.input(&args)
    }
}

