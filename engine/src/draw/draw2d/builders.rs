use crate::draw2d::{DebugVis, Renderer, Result};

pub struct DebugStructure<'a, 'b: 'a> {
    renderer: &'a mut Renderer<'b>,
    result: Result,
}

impl<'a, 'b: 'a> DebugStructure<'a, 'b> {
    pub fn child(&mut self, value: &dyn DebugVis) -> &mut Self {
        self.child_with(|f| value.draw(f))
    }

    pub fn child_with<F>(&mut self, child_renderer: F) -> &mut Self
    where
        F: FnOnce(&mut Renderer<'_>) -> Result,
    {
        self.result = self.result.and_then(|()| child_renderer(self.renderer));
        self
    }

    pub const fn finish(&mut self) -> Result {
        self.result
    }
}
