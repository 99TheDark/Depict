use super::context::{Context, PartialContext};

pub trait System<'a> {
    fn init(&mut self, ctx: &mut PartialContext);
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
}
