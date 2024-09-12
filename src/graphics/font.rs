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

// Could I just make this a bool is_italic?
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontEmphasis {
    Regular,
    Italic,
}

// Is this necessary??
#[derive(Debug)]
pub struct Font {
    pub(crate) id: u32,
    pub styles: HashMap<(FontThickness, FontEmphasis), fontdue::Font>,
}
