//! Utilities for rendering in 2D.

use raylib::prelude::*;
use std::marker::PhantomData;
use std::{iter, result};

// mod builders;
mod rt;

pub type Result = result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

pub trait Render {
    fn render_lines(&mut self, s: &str) -> Result;
    fn render_triangles(&mut self, s: &str) -> Result;
    fn render_quads(&mut self, s: &str) -> Result;

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`write!`] macro itself.
    fn render(&mut self, args: Arguments<'_>) -> Result {
        // We use a specialization for `Sized` types to avoid an indirection
        // through `&mut self`
        trait SpecWriteFmt {
            fn spec_render_fmt(self, args: Arguments<'_>) -> Result;
        }

        impl<R: Render + ?Sized> SpecRenderFmt for &mut R {
            #[inline]
            default fn spec_render_fmt(mut self, args: Arguments<'_>) -> Result {
                render(&mut self, args)
            }
        }

        impl<R: Render> SpecRenderFmt for &mut R {
            #[inline]
            fn spec_render_fmt(self, args: Arguments<'_>) -> Result {
                render(self, args)
            }
        }

        self.spec_render_fmt(args)
    }
}

impl<R: Render + ?Sized> Render for &mut R {
    fn render_lines(&mut self, s: &str) -> Result {
        (**self).render_lines(s)
    }

    fn render_triangles(&mut self, c: char) -> Result {
        (**self).render_triangles(c)
    }

    fn render_quads(&mut self, args: Arguments<'_>) -> Result {
        (**self).render_quads(args)
    }

    fn render(&mut self, args: Arguments<'_>) -> Result {}
}

/// Options for formatting.
///
/// `RenderingOptions` is a [`Renderer`] without an attached [`Render`] trait.
/// It is mainly used to construct `Renderer` instances.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RenderingOptions {
    translation: Vector2,
    rotation: f32,
    scale: Vector2,
    tint: Color,
}

impl RenderingOptions {
    /// Construct a new `RendererBuilder` with the supplied `Render` trait
    /// object for output that is equivalent to the `{}` formatting
    /// specifier:
    ///
    /// - no translation
    /// - no rotation
    /// - 1x scale
    /// - no tint (white)
    pub const fn new() -> Self {
        Self {
            translation: Vector2::ZERO,
            rotation: 0.0,
            scale: Vector2::ONE,
            tint: Color::WHITE,
        }
    }

    pub fn translation(&mut self, translation: Vector2) -> &mut Self {
        self.translation = translation;
        self
    }

    pub fn rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn scale(&mut self, scale: Vector2) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn tint(&mut self, tint: Color) -> &mut Self {
        self.tint = tint;
        self
    }

    pub fn get_translation(&mut self) -> Vector2 {
        self.translation
    }

    pub fn get_rotation(&mut self) -> f32 {
        self.rotation
    }

    pub fn get_scale(&mut self) -> Vector2 {
        self.scale
    }

    pub fn get_tint(&mut self) -> Color {
        self.tint
    }

    pub fn create_renderer<'a>(self, render: &'a mut (dyn Render + 'a)) -> Renderer<'a> {
        Renderer {
            options: self,
            buf: render,
        }
    }
}

impl Default for RenderingOptions {
    /// Same as [`RenderingOptions::new()`].
    fn default() -> Self {
        // The `#[derive(Default)]` implementation would set `scale` to 0x instead of 1x
        // and `tint` to transparent instead of white.
        Self::new()
    }
}

/// Configuration for formatting.
///
/// A `Renderer` represents various options related to formatting. Users do not
/// construct `Renderer`s directly; a mutable reference to one is passed to
/// the `fmt` method of all formatting traits, like [`Debug`] and [`Display`].
///
/// To interact with a `Renderer`, you'll call various methods to change the
/// various options related to formatting. For examples, please see the
/// documentation of the methods defined on `Renderer` below.
#[allow(missing_debug_implementations)]
pub struct Renderer<'a> {
    options: RenderingOptions,

    buf: &'a mut (dyn Render + 'a),
}

impl<'a> Renderer<'a> {
    /// Creates a new formatter with given [`RenderingOptions`].
    ///
    /// If `write` is a reference to a formatter, it is recommended to use
    /// [`Renderer::with_options`] instead as this can borrow the underlying
    /// `write`, thereby bypassing one layer of indirection.
    ///
    /// You may alternatively use [`RenderingOptions::create_formatter()`].
    pub fn new(write: &'a mut (dyn Render + 'a), options: RenderingOptions) -> Self {
        Renderer {
            options,
            buf: write,
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

/// This structure represents a safely precompiled version of a format string
/// and its arguments. This cannot be generated at runtime because it cannot
/// safely be done, so no constructors are given and the fields are private
/// to prevent modification.
///
/// The [`format_args!`] macro will safely create an instance of this structure.
/// The macro validates the format string at compile-time so usage of the
/// [`write()`] and [`format()`] functions can be safely performed.
///
/// You can use the `Arguments<'a>` that [`format_args!`] returns in `Debug`
/// and `Display` contexts as seen below. The example also shows that `Debug`
/// and `Display` format to the same thing: the interpolated format string
/// in `format_args!`.
#[derive(Copy, Clone)]
pub struct Arguments<'a> {
    // Placeholder specs, or `None` if all specs are default (as in "{}{}").
    fmt: Option<&'a [rt::Placeholder]>,

    // Dynamic arguments for interpolation, to be interleaved with string
    // pieces. (Every argument is preceded by a string piece.)
    args: &'a [rt::Argument<'a>],
}

impl DebugVis for Arguments<'_> {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        Draw::draw(self, d)
    }
}

impl Draw for Arguments<'_> {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        write(d.buf, *self)
    }
}

pub trait DebugVis: ?Sized {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

pub trait Draw: ?Sized {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

pub fn render(output: &mut dyn Render, args: Arguments<'_>) -> Result {
    let mut formatter = Renderer::new(output, RenderingOptions::new());
    let mut idx = 0;

    match args.fmt {
        None => {
            // We can use default formatting parameters for all arguments.
            for (i, arg) in args.args.iter().enumerate() {
                // SAFETY: args.args and args.pieces come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                let piece = unsafe { args.pieces.get_unchecked(i) };
                if !piece.is_empty() {
                    formatter.buf.write_str(*piece)?;
                }

                // SAFETY: There are no formatting parameters and hence no
                // count arguments.
                unsafe {
                    arg.fmt(&mut formatter)?;
                }
                idx += 1;
            }
        }
        Some(fmt) => {
            // Every spec has a corresponding argument that is preceded by
            // a string piece.
            for (i, arg) in fmt.iter().enumerate() {
                // SAFETY: fmt and args.pieces come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                let piece = unsafe { args.pieces.get_unchecked(i) };
                if !piece.is_empty() {
                    formatter.buf.write_str(*piece)?;
                }
                // SAFETY: arg and args.args come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                unsafe { run(&mut formatter, arg, args.args) }?;
                idx += 1;
            }
        }
    }

    // There can be only one trailing string piece left.
    if let Some(piece) = args.pieces.get(idx) {
        formatter.buf.write_str(*piece)?;
    }

    Ok(())
}

unsafe fn run(fmt: &mut Renderer<'_>, arg: &rt::Placeholder, args: &[rt::Argument<'_>]) -> Result {
    // let (width, precision) =
    //     // SAFETY: arg and args come from the same Arguments,
    //     // which guarantees the indexes are always within bounds.
    //     unsafe { (getcount(args, &arg.width), getcount(args, &arg.precision)) };

    let options = RenderingOptions {
        translation: arg.translation,
        rotation: arg.rotation,
        scale: arg.scale,
        tint: arg.tint,
    };

    // Extract the correct argument
    debug_assert!(arg.position < args.len());
    // SAFETY: arg and args come from the same Arguments,
    // which guarantees its index is always within bounds.
    let value = unsafe { args.get_unchecked(arg.position) };

    // Set all the formatting options.
    fmt.options = options;

    // Then actually do some printing
    // SAFETY: this is a placeholder argument.
    unsafe { value.fmt(fmt) }
}

// unsafe fn getcount(args: &[rt::Argument<'_>], cnt: &rt::Count) -> u16 {
//     match *cnt {
//         rt::Count::Is(n) => n,
//         rt::Count::Implied => 0,
//         rt::Count::Param(i) => {
//             debug_assert!(i < args.len());
//             // SAFETY: cnt and args come from the same Arguments,
//             // which guarantees this index is always within bounds.
//             unsafe { args.get_unchecked(i).as_u16().unwrap_unchecked() }
//         }
//     }
// }

impl<'a> Renderer<'a> {
    /// Writes some data to the underlying buffer contained within this
    /// formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo;
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         formatter.write_str("Foo")
    ///         // This is equivalent to:
    ///         // write!(formatter, "Foo")
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{Foo}"), "Foo");
    /// assert_eq!(format!("{Foo:0>8}"), "Foo");
    /// ```
    pub fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`write!`] macro itself.
    ///
    /// Writes some formatted information into this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         formatter.write_fmt(format_args!("Foo {}", self.0))
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{}", Foo(-1)), "Foo -1");
    /// assert_eq!(format!("{:0>8}", Foo(2)), "Foo 2");
    /// ```
    #[inline]
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {
        if let Some(s) = fmt.as_statically_known_str() {
            self.buf.write_str(s)
        } else {
            write(self.buf, fmt)
        }
    }

    #[must_use]
    pub fn translation(&self) -> Vector2 {
        self.options.translation
    }

    #[must_use]
    pub fn rotation(&self) -> f32 {
        self.options.rotation
    }

    #[must_use]
    pub fn scale(&self) -> Vector2 {
        self.options.scale
    }

    #[must_use]
    pub fn tint(&self) -> Color {
        self.options.tint
    }

    /// Returns the rendering options this renderer corresponds to.
    pub const fn options(&self) -> RenderingOptions {
        self.options
    }
}

impl Render for Renderer<'_> {
    fn draw_lines(&mut self, s: &str) -> Result {
        self.buf.draw_lines(s)
    }

    fn draw_triangles(&mut self, c: char) -> Result {
        self.buf.draw_triangles(c)
    }

    fn draw_quads(&mut self, c: char) -> Result {
        self.buf.draw_quads(c)
    }

    #[inline]
    fn draw(&mut self, args: Arguments<'_>) -> Result {
        write(self.buf, args)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt("an error occurred when formatting an argument", f)
    }
}

impl Draw for Error {
    fn draw(&self, f: &mut Renderer<'_>) -> Result {
        // Draw::draw("an error occurred when formatting an argument", f)
    }
}
