use crate::{
    engine::{properties::Properties, renderer::RenderBatch},
    graphics::asset::Assets,
};

pub trait Renderable {
    fn request(&self, assets: &mut Assets, properties: &Properties);
    fn render(&self, batch: &mut RenderBatch, properties: &Properties);
}
