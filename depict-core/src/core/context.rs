use image::GenericImageView;
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
    SamplerDescriptor, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureViewDescriptor,
};

use crate::{
    engine::{
        properties::{Properties, Size},
        renderer::Renderer,
    },
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::{render::Renderable, texture::TextureSource};

pub struct InitContext<'a> {
    pub(crate) texture_count: usize,
    pub(crate) textures: Vec<TextureSource>,
    pub(crate) device: &'a Device,
    pub(crate) queue: &'a Queue,
    pub size: Size,
}

impl<'a> InitContext<'a> {
    pub fn image(&mut self, bytes: &[u8]) -> TextureSource {
        let image = image::load_from_memory(bytes).unwrap();
        let id = self.texture_count as u32;

        let rgba = image.to_rgba8();
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

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = self.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        self.texture_count += 1;

        TextureSource {
            image,
            id,
            view,
            sampler,
        }
    }
}

// Might want to change to a general context with a step enum or something like that
pub struct UpdateContext<'a> {
    pub mouse: &'a Tracker<Mouse>,
    pub keyboard: &'a Tracker<Keyboard>,
}

pub struct RenderContext<'a> {
    pub(crate) renderer: &'a mut Renderer,
    pub(crate) properties: &'a Properties, // TODO: Remove
}

impl<'a> RenderContext<'a> {
    pub fn draw(&mut self, renderable: &dyn Renderable) {
        let mut batch = self.renderer.batch(true, &self.properties); // TODO: Get rid of properties here
        renderable.render(&mut batch);
        batch.finish();
    }
}
