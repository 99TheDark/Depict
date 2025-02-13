use wgpu::{VertexAttribute, VertexFormat};

pub(crate) struct Attributes {
    pub attributes: Vec<VertexAttribute>,
    offset: u64,
}

impl Attributes {
    pub(crate) fn new() -> Self {
        Self {
            attributes: Vec::new(),
            offset: 0,
        }
    }

    pub(crate) fn add(&mut self, format: VertexFormat) {
        let attribute = VertexAttribute {
            offset: self.offset,
            shader_location: self.attributes.len() as u32,
            format,
        };

        self.attributes.push(attribute);
        self.offset += format.size();
    }
}
