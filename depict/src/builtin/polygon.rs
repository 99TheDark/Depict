use crate::{
    component::id::RENDERABLE_ID_FACTORY,
    core::renderable::Renderable,
    engine::{properties::Properties, renderer::RenderBatch},
    graphics::asset::Assets,
};

pub struct Polygon {
    pub(crate) id: u32,
    pub points: Vec<(f32, f32)>,
}

impl Renderable for Polygon {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        todo!()
    }
}

impl Polygon {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Self {
        Self {
            id: unsafe { RENDERABLE_ID_FACTORY.next() },
            points: vec![(x1, y1), (x2, y2), (x3, y3)],
        }
    }

    pub fn with_point(mut self, x: f32, y: f32) -> Self {
        self.points.push((x, y));
        self
    }
}
