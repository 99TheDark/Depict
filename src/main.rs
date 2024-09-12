pub mod builtin;
pub mod core;
pub mod engine;
pub mod graphics;
pub mod input;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use builtin::{rectangle::Rectangle, text::Text};
use engine::id::ID_FACTORY;
use graphics::{
    asset::{Asset, Font, Image},
    color::Color,
    font::{FontEmphasis, FontThickness},
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

    roboto: Asset<Font>,
}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut PartialContext) {
        self.dirt = ctx.image(include_bytes!("../res/dirt.png"));
        self.grass = ctx.image(include_bytes!("../res/grass.png"));
        self.brick = ctx.image(include_bytes!("../res/brick.png"));

        self.roboto = ctx.font(HashMap::from([(
            (FontThickness::Regular, FontEmphasis::Regular),
            // Literally why do I need to cast this
            include_bytes!("../res/Roboto/Roboto-Regular.ttf") as &[u8],
        )]));
    }

    fn update(&mut self, ctx: &mut Context) {}

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw_all(vec![
            Rectangle::new(0.0, 0.0, 200.0, 200.0).with_background(Background::Color(Color::BLUE)),
            Rectangle::new(ctx.size.width as f32 - 200.0, 0.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            Rectangle::new(0.0, ctx.size.height as f32 - 200.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            Rectangle::new(
                ctx.size.width as f32 - 200.0,
                ctx.size.height as f32 - 200.0,
                200.0,
                200.0,
            )
            .with_background(Background::Color(Color::BLUE)),
        ]);

        ctx.draw_all(vec![
            Rectangle::new(ctx.mouse.pos.x, ctx.mouse.pos.y, 100.0, 100.0)
                .with_background(Background::Image(self.grass)),
            Rectangle::new(ctx.mouse.pos.x, ctx.mouse.pos.y + 100.0, 100.0, 100.0)
                .with_background(Background::Image(self.dirt)),
            Rectangle::new(ctx.mouse.pos.x + 100.0, ctx.mouse.pos.y, 100.0, 100.0)
                .with_background(Background::Image(self.brick)),
            Rectangle::new(ctx.mouse.pos.x + 200.0, ctx.mouse.pos.y, 100.0, 100.0)
                .with_background(Background::Image(self.brick)),
        ]);

        ctx.draw(
            Text::new(
                100.0,
                500.0,
                "Whereas disregard and contempt\nfor human rights have\nresulted in barbarous acts"
                    .to_string(),
                self.roboto,
            )
            .with_size(60.0),
        );
    }
}

fn main() {
    let mut engine = Engine::new(
        Settings::default().with_background(Color::from_rgb(50, 50, 50)),
        Rc::new(RefCell::new(Game {
            dirt: Asset::default(),
            grass: Asset::default(),
            brick: Asset::default(),

            roboto: Asset::default(),
        })),
    );
    engine.run();
}
