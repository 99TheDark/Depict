use std::collections::{BTreeMap, HashMap};

use image::{DynamicImage, GenericImageView};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
    Sampler, SamplerDescriptor, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

use super::image::Image;

#[derive(Debug)]
pub(crate) struct Atlas {
    pub images: HashMap<u32, Image>,
    pub width: u32,
    pub height: u32,
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl Atlas {
    pub fn new(
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        image_sources: Vec<(u32, DynamicImage)>,
    ) -> Self {
        // Eventually start with smaller images
        let mut rectangles = GroupedRectsToPlace::<u32>::new();
        for (id, image) in &image_sources {
            let (width, height) = image.dimensions();
            rectangles.push_rect(*id, None, RectToInsert::new(width, height, 1));
        }

        let mut bins = BTreeMap::new();
        bins.insert(0, TargetBin::new(width, height, 1));

        // Eventually don't panic with no space
        let placements = pack_rects(
            &rectangles,
            &mut bins,
            &volume_heuristic,
            &contains_smallest_box,
        )
        .unwrap();

        let mut rgba = vec![0; (width * height * 4) as usize];
        let mut images = HashMap::new();
        for (id, image) in &image_sources {
            let (_bin_id, location) = placements.packed_locations().get(&id).unwrap();
            images.insert(
                *id,
                Image {
                    id: *id,
                    u: location.x() as f32 / width as f32,
                    v: location.y() as f32 / height as f32,
                    width: location.width() as f32 / width as f32,
                    height: location.height() as f32 / height as f32,
                },
            );

            let bytes = image.clone().into_rgba8().to_vec();
            for y in 0..location.height() {
                for x in 0..location.width() {
                    let src_idx = (x + y * location.width()) as usize * 4;
                    let dst_idx = ((location.x() + x) + (location.y() + y) * width) as usize * 4;

                    rgba[dst_idx + 0] = bytes[src_idx + 0];
                    rgba[dst_idx + 1] = bytes[src_idx + 1];
                    rgba[dst_idx + 2] = bytes[src_idx + 2];
                    rgba[dst_idx + 3] = bytes[src_idx + 3];
                }
            }
        }

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            images,
            width,
            height,
            texture,
            view,
            sampler,
        }
    }

    pub fn get(&self, id: u32) -> &Image {
        &self.images[&id]
    }
}
