use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use super::{
    atlas::Atlas,
    font::{FontEmphasis, FontThickness},
};

#[derive(Debug)]
pub(crate) struct FontAsset {
    pub fonts: HashMap<(FontThickness, FontEmphasis), Font>,
    pub atlas: Atlas,
}

#[derive(Debug)]
pub(crate) struct Assets {
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
