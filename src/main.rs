pub mod builtin;
pub mod component;
pub mod core;
pub mod engine;
pub mod graphics;
pub mod input;

use core::properties::Align;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use builtin::{rectangle::Rectangle, text::Text};
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

    text_pos: f32,
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
                    Text::new(
                        ctx.size.width / 2.0,
                        0.0,
                        "Lorem ipsum odor amet, consectetuer adipiscing elit. Accumsan eget dis rhoncus nec consequat id. Facilisi praesent tortor lacinia libero parturient aptent euismod neque nunc. Integer auctor ipsum gravida metus euismod nibh sodales tortor. Mi interdum est ac feugiat rhoncus ultricies blandit. Lacus aliquet sociosqu taciti ad taciti ridiculus lacus conubia ad. Taciti euismod sit consequat nec facilisis class. Interdum lacinia urna tristique sollicitudin fermentum senectus odio sed.

        Ad blandit mollis nullam parturient diam tristique purus nibh. Aliquet fermentum bibendum suspendisse felis pretium fames tempor. Dui magnis ante faucibus justo fames ullamcorper ante malesuada. Euismod semper finibus consequat dictum nec. Accumsan aenean class nisi hendrerit fusce sed tempor. Gravida dignissim montes suspendisse consequat amet dapibus.

        Auctor euismod torquent lobortis volutpat semper feugiat ullamcorper. Dictum maximus lectus nibh vulputate nisi. Urna iaculis urna interdum praesent nisl ad. Ad tempus penatibus pulvinar sit faucibus quisque porta praesent. Maximus bibendum tortor; libero taciti etiam elementum! Sit phasellus class; quis aptent euismod morbi rhoncus. Eget ac magna nulla himenaeos per commodo. Eu etiam maecenas viverra; finibus elementum justo fusce nam. Natoque dis mauris bibendum arcu inceptos sed potenti nunc.

        Libero rutrum ut elit cursus sapien conubia dui arcu. At inceptos lacinia, venenatis fringilla id ut elit. Libero finibus phasellus amet posuere lorem. Iaculis efficitur accumsan primis habitasse facilisi etiam torquent. Pharetra montes tristique mi per id; velit curae. Condimentum ex adipiscing tellus orci sed. Taciti adipiscing laoreet consequat commodo suspendisse ultricies.

        Facilisis sem mus mollis ligula dolor, auctor massa luctus. Blandit a placerat himenaeos nostra vehicula sagittis. Pharetra fusce aliquet turpis dapibus potenti. Enim eu urna venenatis leo natoque blandit odio euismod. Libero condimentum dui cras in duis luctus natoque. Ipsum arcu luctus ornare facilisi curae lobortis. Magnis neque quam duis; magna dignissim potenti ornare. Parturient imperdiet efficitur risus turpis pretium curae elementum. Lacinia lectus elit elit; lacus cubilia mus."
                            .to_string(),
                        self.roboto,
                    )
                    .with_color(Color::WHITE)
                    .with_align(Align::Center)
                    .with_width(ctx.size.width)
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
        })),
    );
    engine.run();
}
