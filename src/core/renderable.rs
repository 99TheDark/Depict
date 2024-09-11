use crate::{engine::renderer::RenderBatch, graphics::asset::Assets};

pub trait Renderable {
    fn request(&self, assets: &mut Assets);
    fn render(&self, batch: &mut RenderBatch);
}
