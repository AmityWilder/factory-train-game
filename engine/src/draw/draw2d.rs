//! 2D rendering.
//!
//! ```
//! use raylib::prelude::*;
//! use engine::{render, draw2d::*};
//! ```

#[test]
fn test0() {
    use crate::render;

    let (mut rl, thread) = init().build();

    let line = Line {
        start_pos: Vector2::new(80.0, 5.0),
        end_pos: Vector2::new(32.0, 200.0),
    };

    let triangle = Triangle {
        points: [
            Vector2::new(50.0, 20.0),
            Vector2::new(100.0, 200.0),
            Vector2::new(300.0, 60.0),
        ],
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        render!(&mut d, &line).unwrap();
        render!(&mut d, &triangle).unwrap();
    }
}

use super::Result;
use raylib::prelude::*;
use std::{marker::PhantomData, num::NonZeroU32, ptr::NonNull};

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

macro_rules! argument_new {
    ($t:ty, $x:expr, $f:expr) => {
        // INVARIANT: this creates an `Argument<'a>` from a `&'a T` and
        // a `fn(&T, ...)`, so the invariant is maintained.
        Argument {
            value: NonNull::<$t>::from_ref($x).cast(),
            renderer: {
                let f: fn(&$t, &mut Renderer<'_>) -> Result = $f;
                // SAFETY: This is only called with `value`, which has the right type.
                unsafe { std::mem::transmute(f) }
            },
            _lifetime: PhantomData,
        }
    };
}

impl Argument<'_> {
    #[inline]
    pub const fn new_draw<T: Draw>(x: &T) -> Argument<'_> {
        argument_new!(T, x, <T as Draw>::draw)
    }
    #[inline]
    pub const fn new_debug_vis<T: DebugVis>(x: &T) -> Argument<'_> {
        argument_new!(T, x, <T as DebugVis>::draw)
    }

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

    #[inline]
    pub const fn none() -> [Self; 0] {
        []
    }
}

// /// This struct represents the unsafety of constructing an [`Arguments`].
// /// It exists, rather than an unsafe function, in order to simplify the expansion
// /// of [`render_args!`] and reduce the scope of the `unsafe` block.
// pub struct UnsafeArg {
//     _private: (),
// }

// impl UnsafeArg {
//     /// See documentation where [`UnsafeArg`] is required to know when it is safe to
//     /// create and use [`UnsafeArg`].
//     #[inline]
//     pub const unsafe fn new() -> Self {
//         Self { _private: () }
//     }
// }

// #[derive(Copy, Clone)]
// pub struct Arguments<'a> {
//     args: &'a [Argument<'a>],
// }

// impl<'a> Arguments<'a> {
//     /// Specifies nonstandard formatting parameters.
//     ///
//     /// An [`UnsafeArg`] is required because the following invariants must be held
//     /// in order for this function to be safe:
//     /// 1. The `pieces` slice must be at least as long as `draw`.
//     /// 2. Every `Placeholder::position` value within `draw` must be a valid index of `args`.
//     /// 3. Every `Count::Param` within `draw` must contain a valid index of `args`.
//     ///
//     /// This function should _not_ be const, to make sure we don't accept
//     /// [`render_args!`] and panic!() with arguments in const, even when not evaluated
//     #[inline]
//     pub fn new(
//         args: &'a [Argument<'a>],
//         draw: &'a [Placeholder],
//         _unsafe_arg: UnsafeArg,
//     ) -> Arguments<'a> {
//         Arguments {
//             pieces,
//             fmt: Some(fmt),
//             args,
//         }
//     }
// }

/// Takes an output stream and an `Arguments` struct that can be precompiled with
/// the `render_args!` macro.
///
/// The arguments will be rendered according to the specified render string
/// into the output stream provided.
pub fn render(output: &mut dyn Render, arg: Argument<'_>) -> Result {
    let mut renderer = Renderer::new(output, RenderingOptions::new());

    // SAFETY: There are no formatting parameters and hence no
    // count arguments.
    unsafe {
        arg.draw(&mut renderer)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! render_args {
    ($arg:expr) => {
        Argument::new_draw($arg)
    };
}

#[macro_export]
macro_rules! render {
    ($d:expr, $($args:tt)*) => {
        $crate::draw2d::render($d, $crate::render_args!($($args)*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    position: Vector2,
    color: Option<Color>,
}

impl Vertex {
    #[must_use]
    pub const fn new(position: Vector2) -> Self {
        Self {
            position,
            color: None,
        }
    }

    #[must_use]
    pub const fn with_color(self, color: Color) -> Self {
        Self {
            position: self.position,
            color: Some(color),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TexVertex {
    position: Vector2,
    texcoords: Vector2,
    color: Option<Color>,
}

impl TexVertex {
    #[must_use]
    pub const fn new(position: Vector2) -> Self {
        Self {
            position,
            texcoords: Vector2::ZERO,
            color: None,
        }
    }

    #[must_use]
    pub const fn with_texcoords(self, texcoords: Vector2) -> Self {
        Self {
            position: self.position,
            texcoords,
            color: self.color,
        }
    }

    #[must_use]
    pub const fn with_color(self, color: Color) -> Self {
        Self {
            position: self.position,
            texcoords: self.texcoords,
            color: Some(color),
        }
    }
}

/// A trait for drawing onto 3D buffers.
pub trait Render {
    /// Draw lines.
    fn draw_lines(&mut self, points: &[Vertex]) -> Result;

    /// Draw triangles. Arguments should be provided in counter-clockwise order.
    fn draw_triangles(&mut self, points: &[Vertex]) -> Result;

    /// Draw textured quads. Arguments should be provided in counter-clockwise order.
    fn draw_quads(&mut self, points: &[TexVertex], texture_id: NonZeroU32) -> Result;

    /// Draw anything that implements Draw
    fn draw(&mut self, args: Argument<'_>) -> Result;
}

impl<D: RaylibDraw> Render for D {
    fn draw_lines(&mut self, points: &[Vertex]) -> Result {
        // SAFETY: Borrowing self (a RaylibDraw) mutably ensures rlgl drawing is safe.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            ffi::rlBegin(ffi::RL_LINES.cast_signed());
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            for point in points {
                let &Vertex {
                    position: Vector2 { x, y },
                    color,
                } = point;
                if let Some(Color { r, g, b, a }) = color {
                    ffi::rlColor4ub(r, g, b, a);
                }
                ffi::rlVertex2f(x, y);
            }
            ffi::rlEnd();
        }
        Ok(())
    }

    fn draw_triangles(&mut self, points: &[Vertex]) -> Result {
        // SAFETY: Borrowing self (a RaylibDraw) mutably ensures rlgl drawing is safe.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            ffi::rlBegin(ffi::RL_TRIANGLES.cast_signed());
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            for point in points {
                let &Vertex {
                    position: Vector2 { x, y },
                    color,
                } = point;
                if let Some(Color { r, g, b, a }) = color {
                    ffi::rlColor4ub(r, g, b, a);
                }
                ffi::rlVertex2f(x, y);
            }
            ffi::rlEnd();
        }
        Ok(())
    }

    fn draw_quads(&mut self, points: &[TexVertex], texture_id: NonZeroU32) -> Result {
        // SAFETY: Borrowing self (a RaylibDraw) mutably ensures rlgl drawing is safe.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            ffi::rlSetTexture(texture_id.get());
            ffi::rlBegin(ffi::RL_QUADS.cast_signed());
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            for point in points {
                let &TexVertex {
                    position: Vector2 { x, y },
                    texcoords: Vector2 { x: u, y: v },
                    color,
                } = point;
                if let Some(Color { r, g, b, a }) = color {
                    ffi::rlColor4ub(r, g, b, a);
                }
                ffi::rlTexCoord2f(u, v);
                ffi::rlVertex2f(x, y);
            }
            ffi::rlEnd();
            ffi::rlSetTexture(0);
        }
        Ok(())
    }

    fn draw(&mut self, args: Argument<'_>) -> Result {
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
    pub const fn new(render: &'a mut (dyn Render + 'a), options: RenderingOptions) -> Self {
        Self {
            options,
            buf: render,
        }
    }

    /// Creates a new formatter based on this one with given [`RenderingOptions`].
    pub const fn with_options(&mut self, options: RenderingOptions) -> Renderer<'_> {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape<'a> {
    pub points: &'a [Vector2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start_pos: Vector2,
    pub end_pos: Vector2,
}

impl Draw for Line {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        d.buf.draw_lines(&[
            Vertex::new(self.start_pos + d.options.translation).with_color(d.options.tint),
            Vertex::new(self.end_pos + d.options.translation),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub points: [Vector2; 3],
}

impl Draw for Triangle {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        d.buf.draw_triangles(&[
            Vertex::new(self.points[0] + d.options.translation).with_color(d.options.tint),
            Vertex::new(self.points[1] + d.options.translation),
            Vertex::new(self.points[2] + d.options.translation),
        ])
    }
}
