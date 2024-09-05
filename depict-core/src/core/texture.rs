use image::DynamicImage;
use wgpu::{Sampler, TextureView};

pub struct TextureSource {
    pub image: DynamicImage,
    pub(crate) id: u32,
    pub(crate) view: TextureView,
    pub(crate) sampler: Sampler,
}
