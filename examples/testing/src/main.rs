use std::{cell::RefCell, collections::HashMap, rc::Rc};

use depict::{
    builtin::{rectangle::Rectangle, text::Text},
    core::{
        context::{Context, PartialContext},
        engine::Engine,
        properties::{Align, Background},
        settings::Settings,
        system::System,
    },
    graphics::{
        asset::{Asset, Font, Image},
        color::Color,
        font::{FontEmphasis, FontThickness},
    },
};
use winit::keyboard::{KeyCode, PhysicalKey};

struct Game {
    dirt: Asset<Image>,
    grass: Asset<Image>,
    brick: Asset<Image>,

    roboto: Asset<Font>,

    text_pos: f32,
    typed: String,
}

impl<'a> System<'a> for Game {
    fn init(&mut self, ctx: &mut PartialContext) {
        self.dirt = ctx.image(include_bytes!("../res/dirt.png"));
        self.grass = ctx.image(include_bytes!("../res/grass.png"));
        self.brick = ctx.image(include_bytes!("../res/brick.png"));

        // Loads the entire Roboto font
        self.roboto = ctx.font(HashMap::from([
            (
                (FontThickness::Thin, FontEmphasis::Regular),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-Thin.ttf") as &[u8],
            ),
            (
                (FontThickness::Light, FontEmphasis::Regular),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-Light.ttf") as &[u8],
            ),
            (
                (FontThickness::Regular, FontEmphasis::Regular),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-Regular.ttf") as &[u8],
            ),
            (
                (FontThickness::Medium, FontEmphasis::Regular),
                include_bytes!("../res/Roboto/Roboto-Medium.ttf") as &[u8],
            ),
            (
                (FontThickness::Bold, FontEmphasis::Regular),
                include_bytes!("../res/Roboto/Roboto-Bold.ttf") as &[u8],
            ),
            (
                (FontThickness::Black, FontEmphasis::Regular),
                include_bytes!("../res/Roboto/Roboto-Black.ttf") as &[u8],
            ),
            (
                (FontThickness::Thin, FontEmphasis::Italic),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-ThinItalic.ttf") as &[u8],
            ),
            (
                (FontThickness::Light, FontEmphasis::Italic),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-LightItalic.ttf") as &[u8],
            ),
            (
                (FontThickness::Regular, FontEmphasis::Italic),
                // Literally why do I need to cast this
                include_bytes!("../res/Roboto/Roboto-Italic.ttf") as &[u8],
            ),
            (
                (FontThickness::Medium, FontEmphasis::Italic),
                include_bytes!("../res/Roboto/Roboto-MediumItalic.ttf") as &[u8],
            ),
            (
                (FontThickness::Bold, FontEmphasis::Italic),
                include_bytes!("../res/Roboto/Roboto-BoldItalic.ttf") as &[u8],
            ),
            (
                (FontThickness::Black, FontEmphasis::Italic),
                include_bytes!("../res/Roboto/Roboto-BlackItalic.ttf") as &[u8],
            ),
        ]));
    }

    fn update(&mut self, ctx: &mut Context) {
        self.text_pos = ctx.size.width / 2.0 + f32::sin(ctx.time.seconds() as f32) * 100.0;

        for key in ctx.keyboard.just_pressed() {
            match key.physical_key {
                PhysicalKey::Code(KeyCode::Enter) => {
                    self.typed += "\n";
                }
                PhysicalKey::Code(KeyCode::Backspace) => {
                    self.typed.pop();
                }
                _ => {
                    if let Some(text) = &key.text {
                        self.typed += text;
                    }
                }
            }
        }
    }

    fn render(&mut self, ctx: &mut Context) {
        ctx.draw_all(vec![
            Rectangle::new(0.0, 0.0, 200.0, 200.0).with_background(Background::Color(Color::BLUE)),
            Rectangle::new(ctx.size.width - 200.0, 0.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            Rectangle::new(0.0, ctx.size.height - 200.0, 200.0, 200.0)
                .with_background(Background::Color(Color::BLUE)),
            Rectangle::new(
                ctx.size.width - 200.0,
                ctx.size.height - 200.0,
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
            Text::new(0.0, 0.0, self.typed.clone(), self.roboto)
                .with_color(Color::WHITE)
                .with_width(ctx.size.width),
        );

        ctx.draw(
            Text::new(
                self.text_pos,
                ctx.mouse.pos.y,
                "Whereas disregard and contempt for human rights have resulted in barbarous acts."
                    .to_string(),
                self.roboto,
            )
            .with_size(30.0)
            .with_color(Color::RED)
            .with_align(Align::Center)
            .with_width(ctx.size.width * 0.5)
            .with_emphasis(FontEmphasis::Italic)
            .with_thickness(FontThickness::Bold),
        );
    }
}

fn main() {
    let mut engine = Engine::new(
        Settings::default().with_background(Color::BLACK),
        Rc::new(RefCell::new(Game {
            dirt: Asset::default(),
            grass: Asset::default(),
            brick: Asset::default(),

            roboto: Asset::default(),

            text_pos: 0.0,
            typed: "".to_string(),
        })),
    );
    engine.run();
}
