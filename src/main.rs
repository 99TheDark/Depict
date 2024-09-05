use std::{cell::RefCell, rc::Rc};

use depict_core::core::{
    color::Color,
    context::{InitContext, RenderContext, UpdateContext},
    engine::Engine,
    settings::Settings,
    shapes::Rectangle,
    system::System,
};
use depict_macro::shape;

struct Game {}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut InitContext) {
        println!("Initialized");
    }

    fn update(&mut self, ctx: &mut UpdateContext) {}

    fn render(&mut self, ctx: &mut RenderContext) {
        ctx.draw(&Rectangle::new(0.0, 0.0, 0.1, 0.5));
    }
}

fn main() {
    let mut engine = Engine::new(Settings::default(), Rc::new(RefCell::new(Game {})));
    engine.run();
}
