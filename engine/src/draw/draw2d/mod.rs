//! Utilities for rendering in 2D.

use crate::ascii_canvas::AsciiCanvas;
use raylib::prelude::*;
use std::{marker::PhantomData, num::NonZeroU32};

mod builders;
mod rt;

pub use builders::DebugVisNode;

pub type Result = std::result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    pub position: Vector2,
    pub texcoords: Vector2,
    pub color: Option<Color>,
}

impl Vertex {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutOfBoundsError(usize);

impl std::fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of bounds. value: {}", self.0)
    }
}

impl std::error::Error for OutOfBoundsError {}

#[derive(Debug, Clone)]
pub struct Shape {
    vertices: Vec<Vertex>,
    /// Indices into `vertices`
    indices: Vec<usize>,
    texture_id: Option<NonZeroU32>,
}

impl Default for Shape {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Shape {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_id: None,
        }
    }

    #[must_use]
    pub fn with_capacity(vertex_capacity: usize, index_capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),
            texture_id: None,
        }
    }

    #[inline]
    pub fn with_texture(&mut self, texture_id: Option<NonZeroU32>) -> &mut Self {
        self.texture_id = texture_id;
        self
    }

    #[inline]
    pub fn with_vertices<T: IntoIterator<Item = Vertex>>(&mut self, verts: T) -> &mut Self {
        self.extend(verts);
        self
    }

    /// Stops and returns [`OutOfBoundsError`] when an out of bounds index is encountered
    #[inline]
    pub fn with_indices<T: IntoIterator<Item = usize>>(
        &mut self,
        indices: T,
    ) -> std::result::Result<&mut Self, OutOfBoundsError> {
        let verts_len = self.vertices.len();
        let mut err = Ok(());
        self.indices.extend(indices.into_iter().map_while(|x| {
            if x < verts_len {
                Some(x)
            } else {
                err = Err(OutOfBoundsError(x));
                None
            }
        }));
        err.map(|()| self)
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn clear_indices(&mut self) {
        self.indices.clear();
    }

    /// Also removes any indices referencing the vertex
    pub fn retain_vertices<F: FnMut(&Vertex) -> bool>(&mut self, mut f: F) {
        let mut position = 0;
        let (vertices, indices) = (&mut self.vertices, &mut self.indices);
        vertices.retain(|v| {
            let result = f(v);
            if !result {
                indices.retain(|&idx| idx != position);
            }
            position += 1;
            result
        });
    }

    /// Also removes any indices referencing the vertex
    pub fn remove_vertex(&mut self, position: usize) {
        self.vertices.remove(position);
        self.indices.retain(|&idx| idx != position);
    }

    pub fn remove_index(&mut self, position: usize) {
        self.indices.remove(position);
    }

    #[inline]
    #[must_use]
    pub const fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    #[inline]
    #[must_use]
    pub const fn vertices_mut(&mut self) -> &mut [Vertex] {
        self.vertices.as_mut_slice()
    }

    #[inline]
    #[must_use]
    pub const fn indices(&self) -> &[usize] {
        self.indices.as_slice()
    }
}

impl Extend<Vertex> for Shape {
    #[inline]
    fn extend<T: IntoIterator<Item = Vertex>>(&mut self, iter: T) {
        self.vertices.extend(iter);
    }
}

pub trait Render {
    fn render_pixels(&mut self, points: &[Vertex]) -> Result;
    fn render_lines(&mut self, points: &[Vertex]) -> Result;
    fn render_triangles(&mut self, points: &[Vertex]) -> Result;
    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result;

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
    fn render_pixels(&mut self, points: &[Vertex]) -> Result {
        (**self).render_pixels(points)
    }

    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        (**self).render_lines(points)
    }

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        (**self).render_triangles(points)
    }

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result {
        (**self).render_quads(texture_id, points)
    }

    fn render(&mut self, args: Arguments<'_>) -> Result {
        (**self).render(args)
    }
}

const fn update_and_unwrap(new_color: Option<Color>, prev_color: &mut Color) -> Color {
    if let Some(color) = new_color {
        *prev_color = color;
    }
    *prev_color
}

impl Render for AsciiCanvas {
    fn render_pixels(&mut self, points: &[Vertex]) -> Result {
        let mut prev_color = Color::BLACK;
        for v in points {
            self.draw_pixel_v(v.position, update_and_unwrap(v.color, &mut prev_color));
        }
        Ok(())
    }

    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        let mut prev_color = Color::WHITE;
        for verts in points.array_windows::<2>() {
            let colors = verts.map(|v| update_and_unwrap(v.color, &mut prev_color));
            self.draw_line_v(
                verts[0].position,
                verts[1].position,
                colors[0].lerp(colors[1], 0.5),
            );
        }
        Ok(())
    }

    fn render_triangles(&mut self, points: &[Vertex]) -> Result {
        let mut prev_color = Color::WHITE;
        for verts in points.array_chunks::<3>() {
            self.draw_triangle_ex(
                verts[0].position,
                verts[1].position,
                verts[2].position,
                update_and_unwrap(verts[0].color, &mut prev_color),
                update_and_unwrap(verts[1].color, &mut prev_color),
                update_and_unwrap(verts[2].color, &mut prev_color),
            );
        }
        Ok(())
    }

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result {
        let mut prev_color = Color::WHITE;
        if texture_id.is_none() {
            for verts in points.array_chunks::<4>() {
                self.draw_triangle_ex(
                    verts[0].position,
                    verts[1].position,
                    verts[2].position,
                    update_and_unwrap(verts[0].color, &mut prev_color),
                    update_and_unwrap(verts[1].color, &mut prev_color),
                    update_and_unwrap(verts[2].color, &mut prev_color),
                );
                self.draw_triangle_ex(
                    verts[2].position,
                    verts[0].position,
                    verts[3].position,
                    update_and_unwrap(verts[2].color, &mut prev_color),
                    update_and_unwrap(verts[0].color, &mut prev_color),
                    update_and_unwrap(verts[3].color, &mut prev_color),
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

impl Render for Image {
    fn render_pixels(&mut self, points: &[Vertex]) -> Result {
        let mut prev_color = Color::WHITE;
        for v in points {
            if let Some(color) = v.color {
                prev_color = color;
            }
            self.draw_pixel_v(v.position, prev_color);
        }
        Ok(())
    }

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

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result {
        let mut prev_color = Color::WHITE;
        if texture_id.is_none() {
            for verts in points.array_chunks::<4>() {
                self.draw_triangle_ex(
                    verts[0].position,
                    verts[1].position,
                    verts[2].position,
                    update_and_unwrap(verts[0].color, &mut prev_color),
                    update_and_unwrap(verts[1].color, &mut prev_color),
                    update_and_unwrap(verts[2].color, &mut prev_color),
                );
                self.draw_triangle_ex(
                    verts[2].position,
                    verts[0].position,
                    verts[3].position,
                    update_and_unwrap(verts[2].color, &mut prev_color),
                    update_and_unwrap(verts[0].color, &mut prev_color),
                    update_and_unwrap(verts[3].color, &mut prev_color),
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
    fn render_pixels(&mut self, points: &[Vertex]) -> Result {
        let mut prev_color = Color::BLACK;
        for point in points {
            if let Some(color) = point.color {
                prev_color = color;
            }
            // SAFETY: guaranteed by RaylibDraw
            unsafe {
                ffi::DrawPixelV(point.position.into(), prev_color);
            }
        }
        Ok(())
    }

    fn render_lines(&mut self, points: &[Vertex]) -> Result {
        // SAFETY: guaranteed by RaylibDraw
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            ffi::rlBegin(ffi::RL_LINES as i32);
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            for point in points {
                let &Vertex {
                    position: Vector2 { x, y },
                    texcoords: _,
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
                    texcoords: _,
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

    fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result {
        // SAFETY: guaranteed by RaylibDraw
        unsafe {
            ffi::rlSetTexture(
                texture_id.map_or_else(|| ffi::GetShapesTexture().id, NonZeroU32::get),
            );
            #[allow(clippy::cast_possible_wrap)]
            ffi::rlBegin(ffi::RL_QUADS as i32);
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            for point in points {
                let &Vertex {
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
            fn render_pixels(&mut self, points: &[Vertex]) -> Result {
                RaylibRender(()).render_pixels(points)
            }

            fn render_lines(&mut self, points: &[Vertex]) -> Result {
                RaylibRender(()).render_lines(points)
            }

            fn render_triangles(&mut self, points: &[Vertex]) -> Result {
                RaylibRender(()).render_triangles(points)
            }

            fn render_quads(
                &mut self,
                texture_id: Option<NonZeroU32>,
                points: &[Vertex],
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

    #[inline]
    pub const fn debug_vis_node<'b>(&'b mut self) -> DebugVisNode<'b, 'a> {
        builders::debug_vis_node_new(self)
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
    pub fn render_quads(&mut self, texture_id: Option<NonZeroU32>, points: &[Vertex]) -> Result {
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
    fn render_pixels(&mut self, p: &[Vertex]) -> Result {
        self.buf.render_pixels(p)
    }

    fn render_lines(&mut self, p: &[Vertex]) -> Result {
        self.buf.render_lines(p)
    }

    fn render_triangles(&mut self, p: &[Vertex]) -> Result {
        self.buf.render_triangles(p)
    }

    fn render_quads(&mut self, id: Option<NonZeroU32>, p: &[Vertex]) -> Result {
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
        .map(|p| Vertex::new(self.position + p + d.translation()));

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

impl Draw for Shape {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        // d.render_shape(self)
        todo!()
    }
}

impl Draw for Vector2 {
    fn draw(&self, d: &mut Renderer<'_>) -> Result {
        Vertex::new(*self).draw(d)
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
        .map(|p| Vertex::new(angle.rotate(p * d.scale()) + d.translation()));
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
    use std::mem::ManuallyDrop;

    use crate::{
        ascii_canvas::AsciiCanvasing,
        draw2d::{DebugVis, Draw, Vertex},
        prelude::*,
    };
    use raylib::prelude::*;

    #[repr(transparent)]
    struct ColorAsHex(Color);

    impl std::fmt::Debug for ColorAsHex {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{:02X}{:02X}{:02X}{:02X}",
                self.0.r, self.0.g, self.0.b, self.0.a
            )
        }
    }

    #[test]
    fn test0() {
        const B: Color = Color::BLACK;
        const W: Color = Color::WHITE;
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let mut buf = Image::gen_image_color(5, 5, Color::BLACK);
        render!(&mut buf, {}, Vector2::new(3.0, 2.0)).unwrap();
        let colors = buf.get_image_data();
        for (row, expect) in colors.chunks(5).zip([
            [B, B, B, B, B],
            [B, B, B, B, B],
            [B, B, B, W, B],
            [B, B, B, B, B],
            [B, B, B, B, B],
        ]) {
            assert_eq!(
                row,
                expect,
                "\nexpect: {:?}\nactual: {:?}",
                // SAFETY: ColorAsHex is just a wrapper for Color
                unsafe { &*std::ptr::from_ref(&expect).cast::<[ColorAsHex; 5]>() },
                // SAFETY: ColorAsHex is just a wrapper for Color
                unsafe { &*(std::ptr::from_ref(row) as *const [ColorAsHex]) },
            );
        }
    }

    #[test]
    fn test_debug_vis() {
        struct Foo {
            center: Vector2,
            radius: f32,
            color: Color,
        }

        impl DebugVis for Foo {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                d.render_lines(&[
                    Vertex::new(self.center + Vector2::new(0.0, -1.0) * self.radius)
                        .with_color(Color::MAGENTA),
                    Vertex::new(self.center + Vector2::new(-1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(0.0, 1.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(0.0, -1.0) * self.radius),
                ])
            }
        }

        impl Draw for Foo {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                d.render_triangles(&[
                    Vertex::new(self.center + Vector2::new(0.0, -1.0) * self.radius)
                        .with_color(self.color),
                    Vertex::new(self.center + Vector2::new(-1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(0.0, 1.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(0.0, 1.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(-1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(-1.0, 0.0) * self.radius),
                    Vertex::new(self.center + Vector2::new(0.0, -1.0) * self.radius),
                ])
            }
        }

        struct Bar {
            corner: Vector2,
            size: f32,
            color: Color,
        }

        impl Draw for Bar {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                d.render_quads(
                    None,
                    &[
                        Vertex::new(self.corner).with_color(self.color),
                        Vertex::new(self.corner + Vector2::new(0.0, self.size)),
                        Vertex::new(self.corner + Vector2::new(self.size, self.size)),
                        Vertex::new(self.corner + Vector2::new(self.size, 0.0)),
                    ],
                )
            }
        }

        impl DebugVis for Bar {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                d.render_lines(&[
                    Vertex::new(self.corner).with_color(Color::ORANGE),
                    Vertex::new(self.corner + Vector2::new(0.0, self.size)),
                    Vertex::new(self.corner + Vector2::new(self.size, self.size)),
                    Vertex::new(self.corner + Vector2::new(self.size, 0.0)),
                    Vertex::new(self.corner),
                ])
            }
        }

        struct FooBar(Foo, Bar);

        impl DebugVis for FooBar {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                d.debug_vis_node().child(&self.0).child(&self.1).finish()
            }
        }

        impl Draw for FooBar {
            fn draw(&self, d: &mut super::Renderer<'_>) -> super::Result {
                Draw::draw(&self.0, d).and_then(|()| Draw::draw(&self.1, d))
            }
        }

        let mut canvas = AsciiCanvasing::new_filled(16, 16, Color::BLACK);

        let data = FooBar(
            Foo {
                center: Vector2::new(6.0, 6.0),
                radius: 5.0,
                color: Color::GREEN,
            },
            Bar {
                corner: Vector2::new(7.0, 4.0),
                size: 5.0,
                color: Color::YELLOW,
            },
        );

        render!(canvas.as_mut(), { data }).unwrap();
        println!("{canvas}");
        canvas.clear_background(Color::BLACK);
        render!(canvas.as_mut(), {data:?}).unwrap();
        println!("{canvas}");
    }

    #[test]
    fn test2() {
        #[rustfmt::skip]
        const MASK_DATA: &[u8; 16*16] = &[
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
            0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,
        ];
        // SAFETY: The memory is on the stack and will not be double-freed thanks to ManuallyDrop
        let mask = ManuallyDrop::new(unsafe {
            Image::from_raw(ffi::Image {
                data: MASK_DATA.as_ptr().cast_mut().cast(),
                width: 16,
                height: 16,
                mipmaps: 1,
                format: ffi::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32,
            })
        });
        assert_eq!(mask.get_pixel_data_size(), 16 * 16);
        let mut image = Image::gen_image_gradient_linear(16, 16, 0, Color::GRAY, Color::WHITE);
        image.alpha_mask(&mask);
        let canvas = AsciiCanvasing::from_image(&image).unwrap();
        print!("{canvas}");
    }
}
