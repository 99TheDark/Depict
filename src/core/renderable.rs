use crate::engine::renderer::RenderBatch;

pub trait Renderable {
    fn render(&self, batch: &mut RenderBatch);
}
