use std::{cell::RefCell, rc::Rc};

use depict_core::core::{
    color::Color,
    context::{Context, PartialContext},
    engine::Engine,
    settings::Settings,
    shapes::Rectangle,
    system::System,
};

struct Game {}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut PartialContext) {
        println!("Initialized!");
    }

    fn update(&mut self, ctx: &mut Context) {}

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw(&Rectangle::new(0.0, 0.0, 0.1, 0.5).with_fill(Color::BLUE));
    }
}

fn main() {
    let mut engine = Engine::new(Settings::default(), Rc::new(RefCell::new(Game {})));
    engine.run();
}
