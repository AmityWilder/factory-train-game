//! 3D rendering.

use super::Result;
use raylib::prelude::*;

/// A trait for drawing onto 3D buffers.
pub trait Render {
    /// Draws a ling with optional thickness.
    fn draw_line(&mut self, start_pos: Vector3, end_pos: Vector3, thick: Option<f32>, color: Color);

    /// Draws a triangle.
    fn draw_triangle(&mut self, points: &[Vector3; 3], color: Color);

    /// Draw a 3D mesh with material and transform.
    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material, transform: &Matrix);
}

impl<D: RaylibDraw3D> Render for D {
    fn draw_line(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        thick: Option<f32>,
        color: Color,
    ) {
        match thick {
            Some(thick) => self.draw_capsule(start_pos, end_pos, thick * 0.5, 10, 5, color),
            None => self.draw_line3D(start_pos, end_pos, color),
        }
    }

    fn draw_triangle(&mut self, &[v1, v2, v3]: &[Vector3; 3], color: Color) {
        self.draw_triangle3D(v1, v2, v3, color);
    }

    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material, transform: &Matrix) {
        // SAFETY: Because `material` is a reference to a `Material`, it is guaranteed
        // not to be freed during this method and this copy will not exist after it returns.
        let material = unsafe { WeakMaterial::from_raw(*material.as_ref()) };
        self.draw_mesh(mesh, material, transform);
    }
}

pub struct RenderingOptions {
    offset: Vector3,
    rotation: Quaternion,
    tint: Color,
}

/// Configuration for 3D rendering.
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
    #[doc = include_str!("draw_trait_method_doc.md")]
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

/// Render trait for a typical render.
///
/// `Draw` is similar to [`DebugVis`], but `Draw` is for user-facing output.
pub trait Draw {
    #[doc = include_str!("draw_trait_method_doc.md")]
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}
