use glutin_window::GlutinWindow;
use na;
use na::{Translate, Pnt2, Vec2};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Event, Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventMap, Events};
use sc::Rgba;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Game {
    gl: GlGraphics,
    bg_color: Rgba<f32>,
    hero: Hero,
    window: Rc<RefCell<GlutinWindow>>
}

impl Game {
    pub fn new(opengl: OpenGL, win: GlutinWindow) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            bg_color: Rgba::with_components(0.0, 1.0, 0.0, 1.0),
            hero: Hero::new(
                Rgba::with_components(1.0, 0.0, 0.0, 1.0),
                Pnt2::new(0.0, 0.0),
                50.0),
            window: Rc::new(RefCell::new(win))
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
        use graphics::clear;

        let bg_col = self.bg_color.to_slice();
        self.gl.draw(args.viewport(), |_, gl| { clear(bg_col, gl); });
        self.hero.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.hero.update(args);
    }

    fn input(&mut self, args: &Input) {
        self.hero.input(&args)
    }
}

pub trait Entity {
    fn input(&mut self, _: &Input) { }
    fn update(&mut self, _: &UpdateArgs) { }
    fn render(&mut self, _: &mut GlGraphics, _: &RenderArgs) { }
}

pub struct Hero {
    color: Rgba<f32>,
    topleft: Pnt2<f64>,
    curr_velocity: Vec2<f64>,
    max_velocity: Vec2<f64>,
    size: f64,
    rotation: f64
}

impl Hero {
    fn new(col: Rgba<f32>, tl: Pnt2<f64>, sz: f64) -> Hero {
        Hero {
            color: col,
            topleft: tl,
            curr_velocity: na::zero(),
            max_velocity: Vec2::new(5.0, 5.0),
            size: sz,
            rotation: 0.0
        }
    }
}

impl Entity for Hero {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        let square = rectangle::square(self.topleft[0], self.topleft[1], self.size);
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y)
                                       .rot_rad(self.rotation)
                                       .trans(-(self.size / 2.0), -(self.size / 2.0));

            rectangle(self.color.to_slice(), square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        use std::ops::Neg;
        fn clamp_velocity<F: PartialOrd + na::BaseFloat + Neg>(vel: &Vec2<F>, max: &Vec2<F>) -> Vec2<F> {
            use na::clamp;
            Vec2::new(clamp(vel[0], max[0].neg(), max[0]),
                      clamp(vel[1], max[1].neg(), max[1]))
        }

        self.topleft = clamp_velocity(&(self.curr_velocity * args.dt), &self.max_velocity)
            .translate(&self.topleft);
    }

    fn input(&mut self, iput: &Input) {
        use piston::input::{Input, Button};
        use piston::input::keyboard::Key;

        match iput {
            &Input::Press(Button::Keyboard(key)) => {
                const VELOCITY_INCREMENT: f64 = 500.0;
                if let Key::Up = key {
                    self.curr_velocity[1] -= VELOCITY_INCREMENT;
                }

                if let Key::Down = key {
                    self.curr_velocity[1] += VELOCITY_INCREMENT;
                }

                if let Key::Left = key {
                    self.curr_velocity[0] -= VELOCITY_INCREMENT;
                }

                if let Key::Right = key {
                    self.curr_velocity[0] += VELOCITY_INCREMENT;
                }
            },
            &Input::Release(Button::Keyboard(_)) => { self.curr_velocity = na::zero(); },
            _ => { }
        }
    }
}
