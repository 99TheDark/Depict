use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontThickness {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontEmphasis {
    Regular,
    Bold,
    Italic,
}

#[derive(Debug)]
pub(crate) struct Font {
    pub(crate) id: u32,
    pub(crate) styles: HashMap<(FontThickness, FontEmphasis), fontdue::Font>,
}
