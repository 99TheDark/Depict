use std::sync::Arc;

use image::DynamicImage;
use wgpu::{Device, Queue};
use winit::window::Window;

use crate::{
    engine::{
        properties::{Properties, Size},
        renderer::Renderer,
    },
    graphics::{
        asset::{self, Asset},
        font::Font,
    },
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::render::Renderable;

pub struct PartialContext<'a> {
    pub(crate) sources: Vec<(u32, DynamicImage)>,
    // pub(crate) fonts: Vec<Font>,
    pub(crate) device: &'a Device,
    pub(crate) queue: &'a Queue,
    pub size: Size,
}

impl<'a> PartialContext<'a> {
    pub fn image(&mut self, bytes: &[u8]) -> Asset<asset::Image> {
        let image = image::load_from_memory(bytes).unwrap();
        let id = self.sources.len() as u32;

        self.sources.push((id, image));

        /*let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        });

        self.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        /*let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = self.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });*/

        self.images.push(Image {
            image,
            id,
            // view,
            // sampler,
        });

        Asset::new(id)*/

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

        let mut batch = renderer.batch(true, &self.properties); // TODO: Get rid of properties here
        renderable.render(&mut batch);
        batch.finish();
    }

    pub fn draw_all(&mut self, renderables: &[&dyn Renderable]) {
        if self.step != ContextStep::Render {
            panic!("Can only call renderable methods like draw_all in the render step, within\n\nfn render(&mut self, ctx: &mut RenderContext)");
        }

        let renderer = self.renderer.as_mut().unwrap();

        let mut batch = renderer.batch(true, &self.properties); // TODO: Get rid of properties here too
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
