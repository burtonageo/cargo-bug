#![allow(dead_code)]

use na;
use na::{Translate, Pnt2, Vec2};
use opengl_graphics::GlGraphics;
use piston::input::{Button, Input, RenderArgs, UpdateArgs};
use piston::window::{Size, Window};
use screens::GameScreen;
use sc::Rgba;
use game::{Update, GameInput, RcWindow, Render};
use input_map::{Action, InputMap, Translated};

pub struct OverworldScreen {
    hero: Hero,
    bg_color: Rgba<f32>
}

impl GameScreen for OverworldScreen {
    fn new(window: RcWindow) -> Self where Self: Sized + GameScreen {
        OverworldScreen {
            bg_color: Rgba::with_components(0.0, 1.0, 0.0, 1.0),
            hero: Hero::new(
                window.borrow().draw_size(),
                Rgba::with_components(1.0, 0.0, 0.0, 1.0),
                Pnt2::new(0.0, 0.0),
                50.0),
        }
    }
}

impl Update for OverworldScreen {
    fn update(&mut self, args: &UpdateArgs, window: RcWindow) {
        self.hero.update(args, window);
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
    input_map: InputMap<HeroAction>,
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
    fn new(win_size: Size, col: Rgba<f32>, tl: Pnt2<f64>, sz: f64) -> Hero {
        use piston::input::keyboard::Key;
        let mut input_map = InputMap::new(win_size);
        input_map.add_binding(Button::Keyboard(Key::Up), HeroAction::MoveUp);
        input_map.add_binding(Button::Keyboard(Key::W),  HeroAction::MoveUp);

        input_map.add_binding(Button::Keyboard(Key::Down), HeroAction::MoveDown);
        input_map.add_binding(Button::Keyboard(Key::S),    HeroAction::MoveDown);

        input_map.add_binding(Button::Keyboard(Key::Left), HeroAction::MoveLeft);
        input_map.add_binding(Button::Keyboard(Key::A),    HeroAction::MoveLeft);

        input_map.add_binding(Button::Keyboard(Key::Right), HeroAction::MoveRight);
        input_map.add_binding(Button::Keyboard(Key::D),     HeroAction::MoveRight);

        Hero {
            input_map: input_map,
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
    fn update(&mut self, args: &UpdateArgs, _: RcWindow) {
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
        if let Some(t) = self.input_map.translate(iput) {
            match t {
                Translated::Press(act) => {
                    const VELOCITY_INCREMENT: f64 = 500.0;
                    if let HeroAction::MoveUp = act {
                        self.curr_velocity[1] -= VELOCITY_INCREMENT;
                    }
    
                    if let HeroAction::MoveDown = act {
                        self.curr_velocity[1] += VELOCITY_INCREMENT;
                    }
    
                    if let HeroAction::MoveLeft = act {
                        self.curr_velocity[0] -= VELOCITY_INCREMENT;
                    }
    
                    if let HeroAction::MoveRight = act {
                        self.curr_velocity[0] += VELOCITY_INCREMENT;
                    }
                },
                Translated::Release(_) => { self.curr_velocity = na::zero(); },
                _ => { }
            }
        }
    }
}
