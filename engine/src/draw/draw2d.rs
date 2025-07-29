//! 2D rendering.

use super::Result;
use raylib::prelude::*;

/// A trait for drawing onto 3D buffers.
pub trait Render {
    /// Draws a line with optional thickness.
    fn draw_line(&mut self, start_pos: Vector2, end_pos: Vector2, thick: Option<f32>, color: Color);

    /// Draws a triangle.
    fn draw_triangle(&mut self, points: &[Vector2; 3], color: Color);
}

impl<D: RaylibDraw> Render for D {
    fn draw_line(
        &mut self,
        start_pos: Vector2,
        end_pos: Vector2,
        thick: Option<f32>,
        color: Color,
    ) {
        match thick {
            Some(thick) => self.draw_line_ex(start_pos, end_pos, thick, color),
            None => self.draw_line_v(start_pos, end_pos, color),
        }
    }

    fn draw_triangle(&mut self, &[v1, v2, v3]: &[Vector2; 3], color: Color) {
        self.draw_triangle(v1, v2, v3, color);
    }
}

/// Options for rendering.
///
/// `RenderingOptions` is a [`Renderer`] without an attached [`Render`] trait.
/// It is mainly used to construct `Renderer` instances.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RenderingOptions {
    translation: Vector2,
    rotation: f32,
    scale: Vector2,
    tint: Color,
}

/// Configuration for 2D rendering.
///
/// A `Renderer` represents various options related to rendering. Users do not
/// construct `Renderer`s directly; a mutable reference to one is passed to
/// the `draw` method of all rendering traits, like [`DebugVis`] and [`Draw`].
///
/// To interact with a `Renderer`, you'll call various methods to change the
/// various options related to rendering. For examples, please see the
/// documentation of the methods defined on `Renderer` below.
#[allow(missing_debug_implementations)]
pub struct Renderer<'a> {
    options: RenderingOptions,

    buf: &'a mut (dyn Render + 'a),
}

impl RaylibDraw for Renderer<'_> {}

/// `DebugVis` should render the output in a programmer-facing, debugging context.
pub trait DebugVis {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

pub trait Draw {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}
