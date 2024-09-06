use image::DynamicImage;
use wgpu::{Sampler, TextureView};

#[derive(Debug)]
pub struct Image {
    pub image: DynamicImage,
    pub(crate) id: u32,
    pub(crate) view: TextureView,
    pub(crate) sampler: Sampler,
}
