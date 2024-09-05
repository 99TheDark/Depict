use super::context::{InitContext, RenderContext, UpdateContext};

pub trait System<'a> {
    fn init(&mut self, ctx: &mut InitContext);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn render(&mut self, ctx: &mut RenderContext);
}
