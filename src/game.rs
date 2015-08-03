use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Event, Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventMap, Events};
use std::cell::RefCell;
use std::rc::Rc;
use screens::GameScreens;

pub type RcWindow = Rc<RefCell<GlutinWindow>>;

pub struct Game {
    screens: GameScreens,
    window: RcWindow,
    gl: GlGraphics
}

pub trait Update {
    fn update(&mut self, _: &UpdateArgs, _: RcWindow);
}

pub trait GameInput {
    fn input(&mut self, _: &Input);
}

pub trait Render {
    fn render(&mut self, _: &RenderArgs, _: &mut GlGraphics);
}

impl Game {
    pub fn new(opengl: OpenGL, window: GlutinWindow) -> Game {
        let window = Rc::new(RefCell::new(window));
        Game {
            screens: GameScreens::new(window.clone()),
            window: window,
            gl: GlGraphics::new(opengl)
        }
    }

    pub fn run_loop(&mut self) {
        for e in self.window.clone().events() {
            match e {
                Event::Render(r) => { self.render(&r); },
                Event::Update(u) => { self.update(&u); },
                Event::Input(i)  => { self.input(&i); },
                _ => { }
            }
        }
    }

    // Different signature because Game owns the GlGraphics
    fn render(&mut self, args: &RenderArgs) {
        self.screens.render(args, &mut self.gl);
    }

    fn input(&mut self, args: &Input) {
        self.screens.input(&args)
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.screens.update(args, self.window.clone());
    }
}
