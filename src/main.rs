pub mod core;
pub mod engine;
pub mod graphics;
pub mod input;

use std::{cell::RefCell, rc::Rc};

use graphics::{
    asset::{Asset, Image},
    color::Color,
    shapes::Rectangle,
};

use crate::core::{
    context::{Context, PartialContext},
    engine::Engine,
    properties::Background,
    settings::Settings,
    system::System,
};

struct Game {
    dirt: Asset<Image>,
    grass: Asset<Image>,
    brick: Asset<Image>,
}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut PartialContext) {
        self.dirt = ctx.image(include_bytes!("../res/dirt.png"));
        self.grass = ctx.image(include_bytes!("../res/grass.png"));
        self.brick = ctx.image(include_bytes!("../res/brick.png"));
    }

    fn update(&mut self, ctx: &mut Context) {}

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw_all(&[
            &Rectangle::new(0.0, 0.0, 200.0, 200.0).with_background(Background::Color(Color::BLUE)),
            &Rectangle::new(ctx.size.width as f32 - 200.0, 0.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            &Rectangle::new(0.0, ctx.size.height as f32 - 200.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            &Rectangle::new(
                ctx.size.width as f32 - 200.0,
                ctx.size.height as f32 - 200.0,
                200.0,
                200.0,
            )
            .with_background(Background::Color(Color::BLUE)),
        ]);

        ctx.draw_all(&[
            &Rectangle::new(ctx.mouse.pos.x, ctx.mouse.pos.y, 200.0, 200.0)
                .with_background(Background::Image(self.grass)),
            &Rectangle::new(ctx.mouse.pos.x, ctx.mouse.pos.y + 200.0, 200.0, 200.0)
                .with_background(Background::Image(self.dirt)),
            &Rectangle::new(ctx.mouse.pos.x + 200.0, ctx.mouse.pos.y, 200.0, 200.0)
                .with_background(Background::Image(self.brick)),
            &Rectangle::new(ctx.mouse.pos.x + 400.0, ctx.mouse.pos.y, 200.0, 200.0)
                .with_background(Background::Image(self.brick)),
        ]);
    }
}

fn main() {
    let mut engine = Engine::new(
        Settings::default(),
        Rc::new(RefCell::new(Game {
            dirt: Asset::default(),
            grass: Asset::default(),
            brick: Asset::default(),
        })),
    );
    engine.run();
}
