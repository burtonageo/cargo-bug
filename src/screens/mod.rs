#![allow(dead_code)]

mod menuscreen;
mod overworld;

use game::{Update, GameInput, Render};
use self::menuscreen::MenuScreen;
use opengl_graphics::GlGraphics;
use piston::input::{Input, RenderArgs, UpdateArgs};

pub trait GameScreen: Update + GameInput + Render {
    fn new() -> Box<Self> where Self: Sized + GameScreen;
    fn get_type(&self) -> ScreenType;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScreenType {
    pub Menu,
    pub Overworld,

    NumScreenTypes
}

const NUM_SCREENS: usize = ScreenType::NumScreenTypes as usize;

pub struct GameScreens {
    active_screen: ScreenType,
    screens: Vec<Option<Box<GameScreen>>>
}

impl GameScreens {
    pub fn new() -> GameScreens {
        GameScreens {
            active_screen: ScreenType::Menu,
            screens: {
                let mut v = Vec::with_capacity(NUM_SCREENS);
                v[0] = Some(MenuScreen::new() as Box<GameScreen>);
                for _ in 1..NUM_SCREENS {
                    v.push(None);
                }
                v
            }
        }
    }

    pub fn set_active_screen(&mut self, screen: Box<GameScreen>) {
        self.assert_one_screen_is_active();
        self.screens[self.active_screen as usize] = None;
        self.active_screen = screen.get_type();
        self.screens[self.active_screen as usize] = Some(screen);
    }

    #[inline(always)]
    fn assert_one_screen_is_active(&self) {
        debug_assert!(self.screens.iter().filter(|s| s.is_some()).count() == 1);
    }
}

impl GameInput for GameScreens {
    fn input(&mut self, iput: &Input) {
        self.screens[self.active_screen as usize].as_mut().map(|s| s.input(iput));
    }
}

impl Update for GameScreens {
    fn update(&mut self, args: &UpdateArgs) {
        self.screens[self.active_screen as usize].as_mut().map(|s| s.update(args));
    }
}

impl Render for GameScreens {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.screens[self.active_screen as usize].as_mut().map(|s| s.render(args, gl));
    }
}
