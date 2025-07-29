use crate::draw2d::{DebugVis, Renderer, Result};

pub struct DebugStruct<'a, 'b: 'a> {
    fmt: &'a mut Renderer<'b>,
    result: Result,
    has_fields: bool,
}
