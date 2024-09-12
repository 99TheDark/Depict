use std::collections::{BTreeMap, HashMap};

use image::{DynamicImage, GenericImageView, RgbaImage};
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
pub struct Atlas {
    pub(crate) sources: Vec<(u32, DynamicImage)>,
    pub(crate) images: HashMap<u32, Image>,
    pub size: u32,
    // pub max_size: u32,
    pub extent: Extent3d,
    pub(crate) texture: Texture,
    pub(crate) view: TextureView,
    pub(crate) sampler: Sampler,
    pub(crate) edited: bool,
}

impl Atlas {
    pub(crate) fn new(device: &Device, size: u32) -> Self {
        let extent = Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        });

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
            sources: Vec::new(),
            images: HashMap::new(),
            size,
            extent,
            texture,
            view,
            sampler,
            edited: false,
        }
    }

    pub(crate) fn update(&mut self, queue: &Queue) {
        if !self.edited {
            return;
        }

        // Eventually start with smaller images
        let mut rectangles = GroupedRectsToPlace::<u32>::new();
        for (id, image) in &self.sources {
            let (width, height) = image.dimensions();
            rectangles.push_rect(*id, None, RectToInsert::new(width, height, 1));
        }

        let mut bins = BTreeMap::new();
        bins.insert(0, TargetBin::new(self.size, self.size, 1));

        // Eventually don't panic with no space
        let placements = pack_rects(
            &rectangles,
            &mut bins,
            &volume_heuristic,
            &contains_smallest_box,
        )
        .unwrap();

        let mut rgba = vec![0; (self.size * self.size * 4) as usize];
        for (id, image) in &self.sources {
            let (_bin_id, location) = placements.packed_locations().get(&id).unwrap();
            self.images.insert(
                *id,
                Image {
                    id: *id,
                    u: location.x() as f32 / self.size as f32,
                    v: location.y() as f32 / self.size as f32,
                    width: location.width() as f32 / self.size as f32,
                    height: location.height() as f32 / self.size as f32,
                },
            );

            let bytes = image.clone().into_rgba8().to_vec();
            for y in 0..location.height() {
                for x in 0..location.width() {
                    let src_idx = (x + y * location.width()) as usize * 4;
                    let dst_idx =
                        ((location.x() + x) + (location.y() + y) * self.size) as usize * 4;

                    rgba[dst_idx + 0] = bytes[src_idx + 0];
                    rgba[dst_idx + 1] = bytes[src_idx + 1];
                    rgba[dst_idx + 2] = bytes[src_idx + 2];
                    rgba[dst_idx + 3] = bytes[src_idx + 3];
                }
            }
        }

        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.size * 4),
                rows_per_image: Some(self.size),
            },
            self.extent,
        );

        self.edited = false;

        DynamicImage::ImageRgba8(RgbaImage::from_vec(self.size, self.size, rgba).unwrap())
            .save("res/out/font.png")
            .unwrap();
    }

    pub fn get(&self, id: u32) -> &Image {
        &self.images[&id]
    }

    pub fn add(&mut self, source: DynamicImage) {
        // Breaks if images are removed
        let id = self.sources.len() as u32;
        self.sources.push((id, source));

        self.edited = true;
    }
}