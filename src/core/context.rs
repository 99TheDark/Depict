use std::{collections::HashMap, sync::Arc};

use fontdue::FontSettings;
use image::DynamicImage;
use wgpu::Queue;
use winit::window::Window;

use crate::{
    engine::{dimension::Dimension, properties::Size, renderer::Renderer},
    graphics::{
        asset::{Asset, Assets, Font, Image},
        font::{FontEmphasis, FontThickness},
    },
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::renderable::Renderable;

pub struct PartialContext {
    pub(crate) img_sources: Vec<(u32, DynamicImage)>,
    pub(crate) font_sources: Vec<(u32, HashMap<(FontThickness, FontEmphasis), fontdue::Font>)>,
    pub size: Dimension<f32>,
}

impl PartialContext {
    pub fn image(&mut self, bytes: &[u8]) -> Asset<Image> {
        let id = self.img_sources.len() as u32;
        let image = image::load_from_memory(bytes).unwrap();

        self.img_sources.push((id, image));

        Asset::new(id)
    }

    pub fn font(
        &mut self,
        style_bytes: HashMap<(FontThickness, FontEmphasis), &[u8]>,
    ) -> Asset<Font> {
        let id = self.font_sources.len() as u32;
        let styles: HashMap<_, _> = style_bytes
            .iter()
            .map(|x| {
                (
                    *x.0,
                    fontdue::Font::from_bytes(*x.1, FontSettings::default()).unwrap(),
                )
            })
            .collect();

        self.font_sources.push((id, styles));

        Asset::new(id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ContextStep {
    Update,
    Render,
}

pub struct Context<'a> {
    pub(crate) step: ContextStep,
    pub(crate) window: Arc<Window>,
    pub(crate) assets: &'a mut Assets,
    pub size: Dimension<f32>,
    pub window_size: Size,
    pub mouse: &'a Tracker<Mouse>,
    pub keyboard: &'a Tracker<Keyboard>,
    pub(crate) renderer: Option<&'a mut Renderer>,
    pub(crate) renderables: Vec<Box<dyn Renderable + 'static>>,
}

impl<'a> Context<'a> {
    pub fn draw(&mut self, renderable: impl Renderable + Clone + 'static) {
        if self.step != ContextStep::Render {
            // TODO: Improve errors
            panic!("Can only call renderable methods like draw in the render step, within\n\nfn render(&mut self, ctx: &mut RenderContext)");
        }

        self.renderables.push(Box::new(renderable));
    }

    pub fn draw_all(&mut self, renderables: Vec<impl Renderable + Clone + 'static>) {
        if self.step != ContextStep::Render {
            panic!("Can only call renderable methods like draw_all in the render step, within\n\nfn render(&mut self, ctx: &mut RenderContext)");
        }

        self.renderables.extend(
            renderables
                .iter()
                .map(|r| -> Box<dyn Renderable> { Box::new(r.clone()) }),
        );
    }

    pub(crate) fn render(&mut self, queue: &Queue) {
        let renderer = self.renderer.as_mut().unwrap();

        let mut batch = renderer.batch(&mut self.assets, true);

        // Not the way to go at all (need to cache and delete the ones not used again or something)
        batch.assets.fonts.atlas.sources.clear();
        batch.assets.fonts.atlas.images.clear();
        batch.assets.fonts.data.clear();

        for renderable in &self.renderables {
            renderable.request(&mut batch.assets);
        }

        batch.assets.fonts.update();

        batch.assets.images.update(&queue);
        batch.assets.fonts.atlas.update(&queue);

        for renderable in &self.renderables {
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
