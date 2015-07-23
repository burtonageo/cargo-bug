use glutin_window::GlutinWindow;
use na;
use na::{Translate, Pnt2, Vec2};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventMap, Events};
use simplecolor::Rgba;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Game {
    gl: GlGraphics,
    bg_color: Rgba<f32>,
    entity: Entity,
    window: Rc<RefCell<GlutinWindow>>
}

impl Game {
    // A window must have been constructed before we initialise the
    // GlGraphics object, so we pass in an unused window parameter
    // to ensure this invariant is upheld
    pub fn new(opengl: OpenGL, win: GlutinWindow) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            bg_color: Rgba::with_components(0.0, 1.0, 0.0, 1.0),
            entity: Entity::new(
                Rgba::with_components(1.0, 0.0, 0.0, 1.0),
                Pnt2::new(0.0, 0.0),
                50.0),
            window: Rc::new(RefCell::new(win))
        }
    }

    pub fn run_loop(&mut self) {
        for e in self.window.clone().events() {
            use piston::input::Event;

            if let Event::Render(r) = e {
                self.render(&r);
            }

            if let Event::Update(u) = e {
                self.update(&u);
            }

            if let Event::Input(i) = e {
                self.input(&i);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::clear;

        let bg_col = self.bg_color.to_slice();
        self.gl.draw(args.viewport(), |_, gl| { clear(bg_col, gl); });
        self.entity.render(&mut self.gl, args);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.entity.update(args);
    }

    pub fn input(&mut self, args: &Input) {
        self.entity.input(&args)
    }

}

struct Entity {
    color: Rgba<f32>,
    topleft: Pnt2<f64>,
    velocity: Vec2<f64>,
    size: f64,
    rotation: f64
}

impl Entity {
    fn new(col: Rgba<f32>, tl: Pnt2<f64>, sz: f64) -> Entity {
        Entity {
            color: col,
            topleft: tl,
            velocity: na::zero(),
            size: sz,
            rotation: 0.0
        }
    }

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
        self.velocity = self.velocity * args.dt;
        // self.rotation += 2.0 * args.dt;
        self.topleft = self.velocity.translate(&self.topleft);
    }

    fn input(&mut self, iput: &Input) {
        use piston::input::{Input, Button};
        use piston::input::keyboard::Key;

        match iput {
            &Input::Press(Button::Keyboard(key)) => {
                const MAX_VELOCITY: f64 = 500.0;
                if let Key::Up = key {
                    self.velocity[1] -= MAX_VELOCITY;
                }

                if let Key::Down = key {
                    self.velocity[1] += MAX_VELOCITY;
                }

                if let Key::Left = key {
                    self.velocity[0] -= MAX_VELOCITY;
                }

                if let Key::Right = key {
                    self.velocity[0] += MAX_VELOCITY;
                }
            },
            &Input::Release(Button::Keyboard(_)) => { self.velocity = na::zero(); },
            _ => { }
        }
    }
}
