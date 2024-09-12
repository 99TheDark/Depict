use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use fontdue::Metrics;
use image::{DynamicImage, RgbaImage};

use super::{atlas::Atlas, glyph::Glyph};
use crate::graphics::font;

#[derive(Debug)]
pub struct FontAsset {
    pub fonts: HashMap<u32, font::Font>,
    pub(crate) glyphs: Vec<Glyph>,
    pub(crate) metrics: Vec<Metrics>,
    pub(crate) atlas: Atlas,
}

impl FontAsset {
    pub(crate) fn update(&mut self) {
        for glyph in &self.glyphs {
            let font = &self.fonts[&glyph.font_id];
            let font_style = &font.styles[&(glyph.thickness, glyph.emphasis)];

            let (metrics, bitmap) = font_style.rasterize(glyph.character, glyph.size);
            self.metrics.push(metrics);

            let mut rgba = vec![0; metrics.width * metrics.height * 4];
            for i in 0..metrics.width * metrics.height {
                let idx = i * 4;

                rgba[idx] = (glyph.color.red * 255.0) as u8;
                rgba[idx + 1] = (glyph.color.green * 255.0) as u8;
                rgba[idx + 2] = (glyph.color.blue * 255.0) as u8;
                rgba[idx + 3] = (glyph.color.alpha * bitmap[i] as f32) as u8;
            }

            self.atlas.add(DynamicImage::ImageRgba8(
                RgbaImage::from_vec(metrics.width as u32, metrics.height as u32, rgba).unwrap(),
            ));
        }
    }
}

#[derive(Debug)]
pub struct Assets {
    pub images: Atlas,
    pub fonts: FontAsset,
}

pub trait AssetType {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Image;
impl AssetType for Image {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font;
impl AssetType for Font {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Asset<T: AssetType + Debug + Copy + Clone + PartialEq + Eq> {
    pub(crate) id: u32,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: AssetType + Debug + Copy + Clone + PartialEq + Eq> Asset<T> {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            phantom: PhantomData::default(),
        }
    }

    pub fn valid(&self) -> bool {
        self.id != u32::MAX
    }
}

impl<T: AssetType + Debug + Copy + Clone + PartialEq + Eq> Default for Asset<T> {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            phantom: PhantomData::default(),
        }
    }
}
