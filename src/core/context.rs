use std::sync::Arc;

use image::DynamicImage;
use winit::window::Window;

use crate::{
    engine::{
        properties::{Properties, Size},
        renderer::Renderer,
    },
    graphics::asset::{Asset, Assets, Image},
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::renderable::Renderable;

pub struct PartialContext {
    pub(crate) sources: Vec<(u32, DynamicImage)>,
    // pub(crate) fonts: Vec<(u32, fontdue::Font)>,
    pub size: Size,
}

impl PartialContext {
    pub fn image(&mut self, bytes: &[u8]) -> Asset<Image> {
        let image = image::load_from_memory(bytes).unwrap();
        let id = self.sources.len() as u32;

        self.sources.push((id, image));

        Asset::new(id)
    }

    /*pub fn font(&mut self, bytes: &[u8]) -> Asset<asset::Font> {
        let roboto_regular =
            fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        roboto_regular.rasterize(character, px)
    }*/
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ContextStep {
    Update,
    Render,
}

pub struct Context<'a> {
    pub(crate) step: ContextStep,
    pub(crate) window: Arc<Window>,
    pub(crate) assets: &'a Assets,
    pub size: Size,
    pub window_size: Size,
    pub mouse: &'a Tracker<Mouse>,
    pub keyboard: &'a Tracker<Keyboard>,
    pub(crate) renderer: Option<&'a mut Renderer>,
    pub(crate) properties: &'a Properties,
}

impl<'a> Context<'a> {
    pub fn draw(&mut self, renderable: &dyn Renderable) {
        if self.step != ContextStep::Render {
            // TODO: Improve errors
            panic!("Can only call renderable methods like draw in the render step, within\n\nfn render(&mut self, ctx: &mut RenderContext)");
        }

        let renderer = self.renderer.as_mut().unwrap();

        let mut batch = renderer.batch(&self.assets, true);
        renderable.render(&mut batch);
        batch.finish();
    }

    pub fn draw_all(&mut self, renderables: &[&dyn Renderable]) {
        if self.step != ContextStep::Render {
            panic!("Can only call renderable methods like draw_all in the render step, within\n\nfn render(&mut self, ctx: &mut RenderContext)");
        }

        let renderer = self.renderer.as_mut().unwrap();

        let mut batch = renderer.batch(&self.assets, true);
        for renderable in renderables {
            renderable.render(&mut batch);
        }
        batch.finish();
    }

    pub fn show_cursor(&mut self) {
        self.window.set_cursor_visible(true);
    }

    pub fn hide_cursor(&mut self) {
        self.window.set_cursor_visible(false)
    }
}
