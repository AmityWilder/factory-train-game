//! Utilities for rendering in 2D.

use raylib::prelude::*;
use std::{
    marker::PhantomData,
    num::{NonZeroI32, NonZeroU32},
};

use crate::draw2d::ascii_canvas::AsciiCanvas;

// mod builders;
mod ascii_canvas;
mod rt;

pub type Result = std::result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    pub position: Vector2,
    pub color: Option<Color>,
}

impl Vertex {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self::new_v(Vector2::new(x, y))
    }

    #[must_use]
    pub const fn new_v(position: Vector2) -> Self {
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
    pub position: Vector2,
    pub texcoords: Vector2,
    pub color: Option<Color>,
}

impl TexVertex {
    #[inline]
    #[must_use]
    pub const fn new_xy(x: f32, y: f32) -> Self {
        Self::new(Vector2::new(x, y))
    }

    #[must_use]
    pub const fn new(position: Vector2) -> Self {
        Self {
            position,
            texcoords: Vector2::new(0.0, 0.0),
            color: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn with_texcoords_uv(self, u: f32, v: f32) -> Self {
        self.with_texcoords(Vector2::new(u, v))
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

pub trait Render {
    fn render_lines(&mut self, points: &[Vertex]) -> Result;
    fn render_triangles(&mut self, points: &[Vertex]) -> Result;
    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result;

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`write!`] macro itself.
    fn render(&mut self, args: Arguments<'_>) -> Result {
        // We use a specialization for `Sized` types to avoid an indirection
        // through `&mut self`
        trait SpecRender {
            fn spec_render(self, args: Arguments<'_>) -> Result;
        }

        impl<R: Render + ?Sized> SpecRender for &mut R {
            #[inline]
            default fn spec_render(mut self, args: Arguments<'_>) -> Result {
                render(&mut self, args)
            }
        }

        impl<R: Render> SpecRender for &mut R {
            #[inline]
            fn spec_render(self, args: Arguments<'_>) -> Result {
                render(self, args)
            }
        }

        self.spec_render(args)
    }
}

impl<R: ?Sized + Render> Render for &mut R {
    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        (**self).render_lines(points)
    }

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        (**self).render_triangles(points)
    }

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result {
        (**self).render_quads(texture_id, points)
    }

    fn render(&mut self, args: Arguments<'_>) -> Result {
        (**self).render(args)
    }
}

impl Render for AsciiCanvas {
    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        for [v0, v1] in points.array_windows::<2>() {
            let color = Color::color_from_normalized(
                [v0.color, v1.color]
                    .iter()
                    .flatten()
                    .map(Color::color_normalize)
                    .map(Vector4::from)
                    .sum::<Vector4>()
                    .into(),
            );
            self.draw_line_v(v0.position, v1.position, color);
        }
        Ok(())
    }

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        let mut last_color = Color::WHITE;
        for [v0, v1, v2] in points.array_chunks::<3>() {
            let [c0, c1, c2] = [v0.color, v1.color, v2.color].map(|c| {
                if let Some(c) = c {
                    last_color = c;
                }
                last_color
            });
            self.draw_triangle_ex(v0.position, v1.position, v2.position, c0, c1, c2);
        }
        Ok(())
    }

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result {
        todo!()
    }
}

impl Render for Image {
    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        for [v0, v1] in points.array_windows::<2>() {
            let color = Color::color_from_normalized(
                [v0.color, v1.color]
                    .iter()
                    .flatten()
                    .map(Color::color_normalize)
                    .map(Vector4::from)
                    .sum::<Vector4>()
                    .into(),
            );
            self.draw_line_v(v0.position, v1.position, color);
        }
        Ok(())
    }

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        let mut last_color = Color::WHITE;
        for [v0, v1, v2] in points.array_chunks::<3>() {
            let [c0, c1, c2] = [v0.color, v1.color, v2.color].map(|c| {
                if let Some(c) = c {
                    last_color = c;
                }
                last_color
            });
            self.draw_triangle_ex(v0.position, v1.position, v2.position, c0, c1, c2);
        }
        Ok(())
    }

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result {
        if texture_id.is_none() {
            for [v0, v1, v2, v3] in points.array_chunks::<4>() {
                let color = Color::color_from_normalized(
                    [v0.color, v1.color, v2.color, v3.color]
                        .iter()
                        .flatten()
                        .map(Color::color_normalize)
                        .map(Vector4::from)
                        .sum::<Vector4>()
                        .into(),
                );
                self.draw_triangle_strip(
                    &mut [v0.position, v1.position, v2.position, v3.position],
                    color,
                );
            }
            Ok(())
        } else {
            // applying texture to Image not implemented
            // TODO: consider ffi::ImageDraw
            Err(Error)
        }
    }
}

pub struct RaylibRender(());

#[allow(clippy::multiple_unsafe_ops_per_block)]
impl Render for RaylibRender {
    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        // SAFETY: guaranteed by RaylibDraw
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            ffi::rlBegin(ffi::RL_LINES as i32);
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

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        // SAFETY: guaranteed by RaylibDraw
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            ffi::rlBegin(ffi::RL_TRIANGLES as i32);
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

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result {
        // SAFETY: guaranteed by RaylibDraw
        unsafe {
            ffi::rlSetTexture(
                texture_id.map_or_else(|| ffi::GetShapesTexture().id, NonZeroU32::get),
            );
            #[allow(clippy::cast_possible_wrap)]
            ffi::rlBegin(ffi::RL_QUADS as i32);
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
                let (u, v) = if texture_id.is_none() {
                    let tex = ffi::GetShapesTexture();
                    let rec = ffi::GetShapesTextureRectangle();
                    #[allow(clippy::cast_precision_loss)]
                    (
                        (rec.x + u * rec.width) / tex.width as f32,
                        (rec.y + v * rec.height) / tex.height as f32,
                    )
                } else {
                    (u, v)
                };
                ffi::rlTexCoord2f(u, v);
                ffi::rlVertex2f(x, y);
            }
            ffi::rlEnd();
            ffi::rlSetTexture(0);
        }
        Ok(())
    }
}

macro_rules! impl_rl_render {
    ($(impl$(($($gen:tt)*))? Render for $D:ty {})*) => {
        $(
        impl$(<$($gen)*>)? Render for $D {
            fn render_lines(&mut self, points: &[Vertex]) -> Result {
                RaylibRender(()).render_lines(points)
            }

            fn render_triangles(&mut self, points: &[Vertex]) -> Result {
                RaylibRender(()).render_triangles(points)
            }

            fn render_quads(
                &mut self,
                texture_id: Option<NonZeroU32>,
                points: &[TexVertex],
            ) -> Result {
                RaylibRender(()).render_quads(texture_id, points)
            }

            fn render(&mut self, args: Arguments<'_>) -> Result {
                RaylibRender(()).render(args)
            }
        }
        )*
    };
}

impl_rl_render! {
    impl Render for RaylibDrawHandle<'_> {}
    impl('a, T: 'a) Render for RaylibTextureMode<'a, '_, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibVRMode<'a, '_, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibMode2D<'a, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibMode3D<'a, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibShaderMode<'a, '_, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibBlendMode<'a, T> {}
    impl('a, T: 'a + RaylibDraw) Render for RaylibScissorMode<'a, T> {}
}

/// Options for formatting.
///
/// `RenderingOptions` is a [`Renderer`] without an attached [`Render`] trait.
/// It is mainly used to construct [`Renderer`] instances.
///
/// Transformations are applied in the order of:
/// 1. scale
/// 2. rotation
/// 3. translation
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RenderingOptions {
    translation: Vector2,
    rotation: f32,
    scale: Vector2,
    tint: Color,
}

impl RenderingOptions {
    /// Construct a new [`Renderer`] with the supplied `Render` trait
    /// object for output that is equivalent to the `{}` formatting
    /// specifier:
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

    pub const fn translation(&mut self, translation: Vector2) -> &mut Self {
        self.translation = translation;
        self
    }

    pub const fn rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub const fn scale(&mut self, scale: Vector2) -> &mut Self {
        self.scale = scale;
        self
    }

    pub const fn tint(&mut self, tint: Color) -> &mut Self {
        self.tint = tint;
        self
    }

    #[must_use]
    pub const fn get_translation(&self) -> Vector2 {
        self.translation
    }

    #[must_use]
    pub const fn get_rotation(&self) -> f32 {
        self.rotation
    }

    #[must_use]
    pub const fn get_scale(&self) -> Vector2 {
        self.scale
    }

    #[must_use]
    pub const fn get_tint(&self) -> Color {
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
/// the `fmt` method of all formatting traits, like [`DebugVis`] and [`Draw`].
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

    /// Creates a new renderer based on this one with given [`RenderingOptions`].
    pub fn with_options(&mut self, options: RenderingOptions) -> Renderer<'_> {
        Renderer {
            options,
            buf: self.buf,
        }
    }
}

/// This structure represents a safely precompiled version of a render string
/// and its arguments. This cannot be generated at runtime because it cannot
/// safely be done, so no constructors are given and the fields are private
/// to prevent modification.
///
/// The [`render_args!`] macro will safely create an instance of this structure.
/// The macro validates the render string at compile-time so usage of the
/// [`render()`] functions can be safely performed.
///
/// You can use the `Arguments<'a>` that [`render_args!`] returns in `Debug`
/// and `Display` contexts as seen below. The example also shows that `Debug`
/// and `Display` render to the same thing: the interpolated render string
/// in `render_args!`.
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
        render(d.buf, *self)
    }
}

pub trait DebugVis {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

pub trait Draw {
    fn draw(&self, d: &mut Renderer<'_>) -> Result;
}

pub fn render(output: &mut dyn Render, args: Arguments<'_>) -> Result {
    let mut formatter = Renderer::new(output, RenderingOptions::new());

    match args.fmt {
        None => {
            // We can use default formatting parameters for all arguments.
            for arg in args.args {
                // SAFETY: There are no formatting parameters and hence no
                // count arguments.
                unsafe {
                    arg.fmt(&mut formatter)?;
                }
            }
        }
        Some(fmt) => {
            // Every spec has a corresponding argument that is preceded by
            // a string piece.
            for arg in fmt {
                // SAFETY: arg and args.args come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                unsafe { run(&mut formatter, arg, args.args) }?;
            }
        }
    }

    Ok(())
}

unsafe fn run(fmt: &mut Renderer<'_>, arg: &rt::Placeholder, args: &[rt::Argument<'_>]) -> Result {
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

impl Renderer<'_> {
    /// Renders some data to the underlying buffer contained within this renderer.
    pub fn render_lines(&mut self, points: &[Vertex]) -> Result {
        self.buf.render_lines(points)
    }

    /// Renders some data to the underlying buffer contained within this renderer.
    ///
    /// Provide `points` in counter-clockwise order.
    pub fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        self.buf.render_triangles(points)
    }

    /// Renders some data to the underlying buffer contained within this renderer.
    ///
    /// Provide `points` in counter-clockwise order.
    pub fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[TexVertex]) -> Result {
        self.buf.render_quads(texture_id, points)
    }

    /// Glue for usage of the [`render!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`render!`] macro itself.
    ///
    /// Renders some formatted information into this instance.
    #[inline]
    pub fn render(&mut self, args: Arguments<'_>) -> Result {
        render(self.buf, args)
    }

    #[must_use]
    pub const fn translation(&self) -> Vector2 {
        self.options.translation
    }

    #[must_use]
    pub const fn rotation(&self) -> f32 {
        self.options.rotation
    }

    #[must_use]
    pub const fn scale(&self) -> Vector2 {
        self.options.scale
    }

    #[must_use]
    pub const fn tint(&self) -> Color {
        self.options.tint
    }

    /// Returns the rendering options this renderer corresponds to.
    #[must_use]
    pub const fn options(&self) -> RenderingOptions {
        self.options
    }
}

impl Render for Renderer<'_> {
    fn render_lines(&mut self, p: &[Vertex]) -> Result {
        self.buf.render_lines(p)
    }

    fn render_triangles(&mut self, p: &[Vertex]) -> Result {
        self.buf.render_triangles(p)
    }

    fn render_quads(&mut self, id: Option<NonZeroU32>, p: &[TexVertex]) -> Result {
        self.buf.render_quads(id, p)
    }

    #[inline]
    fn render(&mut self, args: Arguments<'_>) -> Result {
        render(self.buf, args)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt("an error occurred when formatting an argument", f)
    }
}

impl Draw for Error {
    fn draw(&self, _d: &mut Renderer<'_>) -> Result {
        // Draw::draw("an error occurred when formatting an argument", f)
        todo!()
    }
}

// Implementations of the core formatting traits

macro_rules! draw_refs {
    ($($tr:ident),*) => {
        $(
        impl<T: ?Sized + $tr> $tr for &T {
            fn draw(&self, d: &mut Renderer<'_>) -> Result { $tr::draw(&**self, d) }
        }
        impl<T: ?Sized + $tr> $tr for &mut T {
            fn draw(&self, d: &mut Renderer<'_>) -> Result { $tr::draw(&**self, d) }
        }
        )*
    }
}

draw_refs! { DebugVis, Draw }

impl Draw for Vertex {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        let tint = self.color.unwrap_or(Color::WHITE).tint(d.tint());
        let [p0, p1, p2, p3] = [
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(1.0, 0.0),
        ]
        .map(|p| TexVertex::new(self.position + p + d.translation()));

        d.render_quads(
            None,
            &[
                p0.with_texcoords_uv(0.0, 0.0).with_color(tint),
                p1.with_texcoords_uv(0.0, 1.0),
                p2.with_texcoords_uv(1.0, 1.0),
                p3.with_texcoords_uv(1.0, 0.0),
            ],
        )
    }
}

impl Draw for Vector2 {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        Vertex::new_v(*self).draw(d)
    }
}

impl Draw for Texture2D {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        #[allow(clippy::cast_precision_loss)]
        let (width, height) = (self.width as f32, self.height as f32);
        let angle = Vector2::from_angle(d.rotation());
        let [p0, p1, p2, p3] = [
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, height),
            Vector2::new(width, height),
            Vector2::new(width, 0.0),
        ]
        .map(|p| TexVertex::new(angle.rotate(p * d.scale()) + d.translation()));
        let points = [
            p0.with_texcoords_uv(0.0, 1.0).with_color(d.tint()),
            p1.with_texcoords_uv(0.0, 0.0),
            p2.with_texcoords_uv(1.0, 0.0),
            p3.with_texcoords_uv(1.0, 1.0),
        ];
        d.render_quads(NonZeroU32::new(self.id), &points)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use raylib::prelude::*;

    #[test]
    fn test0() {
        const WIDTH: usize = 32;
        const HEIGHT: usize = 16;
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let mut buf = Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK);
        render!(&mut buf, Vector2::new(5.0, 9.0)).unwrap();
        let colors = buf.get_image_data();
        println!();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                assert_eq!(
                    colors[WIDTH * y + x],
                    if y == 9 && x == 5 {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    }
                );
            }
        }
    }
}
