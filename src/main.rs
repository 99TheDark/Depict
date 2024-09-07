pub mod core;
pub mod engine;
pub mod input;

use std::{cell::RefCell, rc::Rc};

use crate::core::{
    asset::Asset,
    context::{Context, PartialContext},
    engine::Engine,
    properties::Background,
    settings::Settings,
    shapes::Rectangle,
    system::System,
};

struct Game {
    grass: Asset,
}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut PartialContext) {
        self.grass = ctx.image(include_bytes!("../res/grass.png"))
    }

    fn update(&mut self, ctx: &mut Context) {}

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw(
            &Rectangle::new(0.0, 0.0, 500.0, 500.0).with_background(Background::Image(self.grass)),
        );
    }
}

fn main() {
    let mut engine = Engine::new(
        Settings::default(),
        Rc::new(RefCell::new(Game {
            grass: Asset::new(),
        })),
    );
    engine.run();
}
