use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use fontdue::Metrics;
use image::{DynamicImage, RgbaImage};

use super::{atlas::Atlas, glyph::TextRenderingData};
use crate::graphics::font;

#[derive(Debug)]
pub struct FontAsset {
    pub fonts: HashMap<u32, font::Font>,
    pub(crate) data: HashMap<u32, TextRenderingData>,
    pub(crate) atlas: Atlas,
}

impl FontAsset {
    pub(crate) fn update(&mut self) {
        for (_, TextRenderingData { glyphs, metrics }) in &mut self.data {
            'outer: for idx in 0..glyphs.len() {
                for i in 0..idx {
                    if glyphs[idx] == glyphs[i] {
                        glyphs[idx] = glyphs[i].clone();
                        metrics.push(metrics[i]);
                        continue 'outer;
                    }
                }

                let glyph = &mut glyphs[idx];

                // Maybe eventually add form feed & vertical tab?
                if glyph.character == '\n' || glyph.character == '\r' {
                    metrics.push(Metrics::default());
                    continue;
                }

                let font = &self.fonts[&glyph.font_id];
                let font_style = &font.styles[&(glyph.thickness, glyph.emphasis)];

                let (font_metrics, bitmap) = font_style.rasterize(glyph.character, glyph.size);
                metrics.push(font_metrics);

                let width = font_metrics.width;
                let height = font_metrics.height;

                let mut rgba = vec![0; width * height * 4];
                for i in 0..width * height {
                    let idx = i * 4;

                    // Why though?
                    if bitmap[i] == 0 {
                        continue;
                    }

                    rgba[idx] = (glyph.color.red * 255.0) as u8;
                    rgba[idx + 1] = (glyph.color.green * 255.0) as u8;
                    rgba[idx + 2] = (glyph.color.blue * 255.0) as u8;
                    rgba[idx + 3] = (glyph.color.alpha * bitmap[i] as f32) as u8;
                }

                glyph.image_id = self.atlas.add(DynamicImage::ImageRgba8(
                    RgbaImage::from_vec(width as u32, height as u32, rgba).unwrap(),
                ));
            }
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
