#![allow(dead_code)]

use na;
use na::{Translate, Pnt2, Vec2};
use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};
use screens::GameScreen;
use sc::Rgba;
use game::{GameInput, Update, Render};
use input_map::{Action, InputMap};
use piston::window::Size;

pub struct OverworldScreen {
    hero: Hero,
    hero_inmap: InputMap<HeroAction>,
    bg_color: Rgba<f32>
}

impl GameScreen for OverworldScreen {
    fn new() -> Self where Self: Sized + GameScreen {
        OverworldScreen {
            bg_color: Rgba::with_components(0.0, 1.0, 0.0, 1.0),
            hero_inmap: InputMap::new(Size {width: 800, height: 600}),
            hero: Hero::new(
                Rgba::with_components(1.0, 0.0, 0.0, 1.0),
                Pnt2::new(0.0, 0.0),
                50.0),
        }
    }
}

impl Update for OverworldScreen {
    fn update(&mut self, args: &UpdateArgs) {
    self.hero.update(args);
    
    }
}

impl GameInput for OverworldScreen {
    fn input(&mut self, input: &Input) {
        self.hero.input(input);
    }
}

impl Render for OverworldScreen {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::clear;
        gl.draw(args.viewport(), |_, gl| { clear(self.bg_color.to_slice(), gl); });
        self.hero.render(args, gl);
    }
}

trait Entity: Render + Update + GameInput {
    fn update(&mut self, _: &UpdateArgs) { }
    fn input(&mut self, _: &Input) { }
    fn render(&mut self, _: &RenderArgs, _: &mut GlGraphics) { }
}

struct Hero {
    color: Rgba<f32>,
    topleft: Pnt2<f64>,
    curr_velocity: Vec2<f64>,
    max_velocity: Vec2<f64>,
    size: f64,
    rotation: f64
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum HeroAction {
    MoveUp,
    MoveLeft,
    MoveRight,
    MoveDown
}
impl Action for HeroAction {}

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

impl Render for Hero {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
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
}

impl Update for Hero {
    fn update(&mut self, args: &UpdateArgs) {
        use std::ops::Neg;
        fn clamp_velocity<F: PartialOrd + na::BaseFloat + Neg>(vel: &Vec2<F>, max: &Vec2<F>) -> Vec2<F> {
            use na::clamp;
            Vec2::new(clamp(vel[0], max[0].neg(), max[0]),
                      clamp(vel[1], max[1].neg(), max[1]))
        }

        self.topleft = clamp_velocity(&(self.curr_velocity * args.dt),
                                      &self.max_velocity)
                           .translate(&self.topleft);
    }
}

impl GameInput for Hero {
    fn input(&mut self, iput: &Input) {
        use piston::input::Button;
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
