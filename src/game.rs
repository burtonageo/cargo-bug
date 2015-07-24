use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Event, Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventMap, Events};
use std::cell::RefCell;
use std::rc::Rc;
use world::World;

pub struct Game {
    gl: GlGraphics,
    window: Rc<RefCell<GlutinWindow>>,
    world: World
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

    fn render(&mut self, args: &RenderArgs) {
        self.world.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.update(args);
    }

    fn input(&mut self, args: &Input) {
        self.world.input(&args)
    }
}
