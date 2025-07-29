//! 2D rendering.

use super::Result;
use raylib::prelude::*;
use std::{marker::PhantomData, ptr::NonNull};

macro_rules! render_args {
    () => {};
}

/// This struct represents a generic "argument" which is taken by [`render_args!()`].
///
/// A placeholder argument contains a function to render the given value. At
/// compile time it is ensured that the function and the value have the correct
/// types, and then this struct is used to canonicalize arguments to one type.
/// Placeholder arguments are essentially an optimized partially applied renderting
/// function, equivalent to `exists T.(&T, fn(&T, &mut Renderer<'_>) -> Result`.
#[derive(Copy, Clone)]
pub struct Argument<'a> {
    // INVARIANT: `renderer` has type `fn(&T, _) -> _` for some `T`, and `value`
    // was derived from a `&'a T`.
    value: NonNull<()>,
    renderer: unsafe fn(NonNull<()>, &mut Renderer<'_>) -> Result,
    _lifetime: PhantomData<&'a ()>,
}

impl Argument<'_> {
    /// Format this placeholder argument.
    ///
    /// # Safety
    ///
    /// This argument must actually be a placeholder argument.
    #[inline]
    unsafe fn draw(&self, d: &mut Renderer<'_>) -> Result {
        let Self {
            renderer, value, ..
        } = *self;
        // SAFETY:
        // Because of the invariant that if `renderer` had the type
        // `fn(&T, _) -> _` then `value` has type `&'b T` where `'b` is
        // the lifetime of the `Argument`, and because references
        // and `NonNull` are ABI-compatible, this is completely equivalent
        // to calling the original function passed to `new` with the
        // original reference, which is sound.
        unsafe { renderer(value, d) }
    }
}

/// This structure represents a safely precompiled version of a render group
/// and its arguments. This cannot be generated at runtime because it cannot
/// safely be done, so no constructors are given and the fields are private
/// to prevent modification.
///
/// The [`render_args!`] macro will safely create an instance of this structure.
/// The macro validates the render group at compile-time so usage of the
/// [`render()`] function can be safely performed.
///
/// You can use the `Arguments<'a>` that [`render_args!`] returns in [`DebugVis`]
/// and [`Draw`] contexts as seen below. The example also shows that [`DebugVis`]
/// and [`Draw`] render to the same thing: the interpolated render group
/// in `render_args!`.
#[derive(Copy, Clone)]
pub struct Arguments<'a> {
    // Dynamic arguments for rendering
    args: &'a [Argument<'a>],
}

/// Takes an output stream and an `Arguments` struct that can be precompiled with
/// the `render_args!` macro.
///
/// The arguments will be rendered according to the specified render string
/// into the output stream provided.
pub fn render(output: &mut dyn Render, args: Arguments<'_>) -> Result {
    let mut renderer = Renderer::new(output, RenderingOptions::new());

    // We can use default formatting parameters for all arguments.
    for arg in args.args {
        // SAFETY: There are no formatting parameters and hence no
        // count arguments.
        unsafe {
            arg.draw(&mut renderer)?;
        }
    }

    Ok(())
}

/// A trait for drawing onto 3D buffers.
pub trait Render {
    /// Draws a line with optional thickness.
    fn draw_line(
        &mut self,
        start_pos: Vector2,
        end_pos: Vector2,
        thick: Option<f32>,
        color: Color,
    ) -> Result;

    /// Draws a triangle.
    fn draw_triangle(&mut self, points: &[Vector2; 3], color: Color) -> Result;

    fn draw(&mut self, args: Arguments<'_>) -> Result;
}

impl<D: RaylibDraw> Render for D {
    fn draw_line(
        &mut self,
        start_pos: Vector2,
        end_pos: Vector2,
        thick: Option<f32>,
        color: Color,
    ) -> Result {
        match thick {
            Some(thick) => self.draw_line_ex(start_pos, end_pos, thick, color),
            None => self.draw_line_v(start_pos, end_pos, color),
        }
        Ok(())
    }

    fn draw_triangle(&mut self, &[v1, v2, v3]: &[Vector2; 3], color: Color) -> Result {
        self.draw_triangle(v1, v2, v3, color);
        Ok(())
    }

    fn draw(&mut self, args: Arguments<'_>) -> Result {
        render(self, args)
    }
}

/// Options for rendering.
///
/// `RenderingOptions` is a [`Renderer`] without an attached [`Render`] trait.
/// It is mainly used to construct `Renderer` instances.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RenderingOptions {
    translation: Vector2,
    /// Degrees
    rotation: f32,
    scale: Vector2,
    tint: Color,
}

impl Default for RenderingOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RenderingOptions {
    /// Construct a new `RenderingOptions` with the following specifier:
    ///
    /// - no translation
    /// - no rotation
    /// - 1x scale
    /// - no tint (white)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            translation: Vector2::ZERO,
            rotation: 0.0,
            scale: Vector2::ONE,
            tint: Color::WHITE,
        }
    }

    /// Sets the translation.
    pub const fn translation(&mut self, translation: Vector2) -> &mut Self {
        self.translation = translation;
        self
    }

    /// Sets the rotation in degrees.
    pub const fn rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    /// Sets the uniform scale.
    pub const fn scale(&mut self, scale: f32) -> &mut Self {
        self.scale = Vector2::new(scale, scale);
        self
    }

    /// Sets the anisotropic scale.
    pub const fn scale_v(&mut self, scale: Vector2) -> &mut Self {
        self.scale = scale;
        self
    }

    /// Sets the tint.
    pub const fn tint(&mut self, tint: Color) -> &mut Self {
        self.tint = tint;
        self
    }

    /// Returns the currnet translation.
    pub const fn get_translation(&mut self) -> Vector2 {
        self.translation
    }

    /// Returns the currnet rotation in degrees.
    pub const fn get_rotation(&mut self) -> f32 {
        self.rotation
    }

    /// Returns the currnet scale.
    pub const fn get_scale(&mut self) -> Vector2 {
        self.scale
    }

    /// Returns the currnet tint.
    pub const fn get_tint(&mut self) -> Color {
        self.tint
    }
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

impl<'a> Renderer<'a> {
    pub fn new(render: &'a mut (dyn Render + 'a), options: RenderingOptions) -> Self {
        Self {
            options,
            buf: render,
        }
    }

    /// Creates a new formatter based on this one with given [`RenderingOptions`].
    pub fn with_options<'b>(&'b mut self, options: RenderingOptions) -> Renderer<'b> {
        Renderer {
            options,
            buf: self.buf,
        }
    }
}

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
