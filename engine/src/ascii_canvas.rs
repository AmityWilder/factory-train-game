use raylib::prelude::*;
use std::{marker::PhantomData, ops::Range, ptr::NonNull};

#[cfg(target_pointer_width = "16")]
type TargetUHalf = u8;
#[cfg(target_pointer_width = "32")]
type TargetUHalf = u16;
#[cfg(target_pointer_width = "64")]
type TargetUHalf = u32;

///  Unsigned integer half the size of a [`usize`].
///
///  Despite not implementing [`From`], this type can be `as usize`'d losslessly.
#[allow(non_camel_case_types)]
pub type uhalf = TargetUHalf;

/// [`AsciiCanvasing`] is to [`AsciiCanvas`] as [`String`] is to [`str`]
#[derive(Debug, PartialEq, Eq)]
pub struct AsciiCanvasing {
    capacity: usize,
    canvas: AsciiCanvas,
}

impl Default for AsciiCanvasing {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AsciiCanvasing {
    fn drop(&mut self) {
        drop(self.data_vec());
    }
}

impl Clone for AsciiCanvasing {
    fn clone(&self) -> Self {
        // SAFETY: data_slice is guaranteed to have a len of self.width * self.height
        unsafe { Self::from_vec_unchecked(self.width, self.height, self.data_slice().to_vec()) }
    }
}

impl std::ops::Deref for AsciiCanvasing {
    type Target = AsciiCanvas;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl std::ops::DerefMut for AsciiCanvasing {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}

impl AsRef<AsciiCanvas> for AsciiCanvasing {
    #[inline]
    fn as_ref(&self) -> &AsciiCanvas {
        self
    }
}

impl AsMut<AsciiCanvas> for AsciiCanvasing {
    #[inline]
    fn as_mut(&mut self) -> &mut AsciiCanvas {
        self
    }
}

impl std::fmt::Display for AsciiCanvasing {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

/// `width` * `height`
#[inline]
#[must_use]
const fn area(w: uhalf, h: uhalf) -> usize {
    w as usize * h as usize
}

impl AsciiCanvasing {
    #[must_use = "dropping the returned vec will leave `self.canvas.data` dangling"]
    fn data_vec(&mut self) -> Vec<u8> {
        // SAFETY: reassembling the vec from before
        unsafe { Vec::from_parts(self.canvas.data, self.canvas.data_len(), self.capacity) }
    }

    #[must_use]
    pub const fn new() -> Self {
        // SAFETY: Vec::new().len() == 0 * 0
        unsafe { Self::from_vec_unchecked(0, 0, Vec::new()) }
    }

    /// # Safety
    ///
    /// `vec.len` must equal `width * height`
    #[must_use]
    pub const unsafe fn from_vec_unchecked(width: uhalf, height: uhalf, mut vec: Vec<u8>) -> Self {
        // SAFETY: Vec pointer is guaranteed to be non-null
        let data = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };
        let capacity = vec.capacity();
        std::mem::forget(vec); // We'll free it in the destructor
        Self {
            capacity,
            canvas: AsciiCanvas {
                width,
                height,
                data,
                _marker: PhantomData,
            },
        }
    }

    /// Returns [`Err`] if `vec`'s length does not equal `width * height`
    pub const fn from_vec(width: uhalf, height: uhalf, vec: Vec<u8>) -> Result<Self, Vec<u8>> {
        if area(width, height) == vec.len() {
            // SAFETY: just checked
            Ok(unsafe { Self::from_vec_unchecked(width, height, vec) })
        } else {
            Err(vec)
        }
    }

    #[must_use]
    pub fn new_filled(width: uhalf, height: uhalf, clear_fill: Color) -> Self {
        let clear_fill = AsciiCanvas::color_to_value(clear_fill);
        // SAFETY: Vec constructed from width * height
        unsafe { Self::from_vec_unchecked(width, height, vec![clear_fill; area(width, height)]) }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            canvas: AsciiCanvas {
                width: 0,
                height: 0,
                data: Vec::with_capacity(capacity).into_parts().0,
                _marker: PhantomData,
            },
        }
    }

    pub fn resize(&mut self, width: uhalf, height: uhalf, clear_fill: Color) {
        let new_size = area(width, height);
        let mut vec = self.data_vec();
        vec.resize(new_size, AsciiCanvas::color_to_value(clear_fill));
        for y in (0..(self.height as usize)).rev() {
            vec.copy_within(
                (y * self.width as usize)..((y + 1) * self.width as usize),
                y * width as usize,
            );
        }
        // SAFETY: vec has been resized to width * height
        *self = unsafe { Self::from_vec_unchecked(width, height, vec) };
    }

    pub fn from_image(image: &Image) -> Result<Self, std::num::TryFromIntError> {
        let width = image.width.try_into()?;
        let height = image.height.try_into()?;
        let vec = image
            .get_image_data()
            .iter()
            .copied()
            .map(AsciiCanvas::color_to_value)
            .collect::<Vec<u8>>();
        // SAFETY: get_image_data is guaranteed to return a slice of len == width * height
        Ok(unsafe { Self::from_vec_unchecked(width, height, vec) })
    }

    #[must_use]
    pub fn to_image(&self) -> Option<Image> {
        let width = self.width.try_into().ok()?;
        let height = self.height.try_into().ok()?;
        let mut image = Image::gen_image_color(width, height, Color::BLACK);
        image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE);
        let image_data = NonNull::new(image.data)?;
        // SAFETY: Separate allocations cannot overlap. gen_image_color is guaranteed to return a valid,
        // aligned pointer if it is not null.
        unsafe {
            image_data
                .cast::<u8>()
                .copy_from_nonoverlapping(self.data, self.canvas.data_len());
        };
        Some(image)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AsciiCanvas {
    data: NonNull<u8>,
    width: uhalf,
    height: uhalf,
    _marker: PhantomData<[u8]>,
}

// AsciiCanvas should be isomorphic to a fat pointer (i would like for it to be)
const _: () = {
    assert!(std::mem::size_of::<AsciiCanvas>() == std::mem::size_of::<&[u8]>());
    assert!(std::mem::align_of::<AsciiCanvas>() == std::mem::align_of::<&[u8]>());
    assert!(std::mem::offset_of!(AsciiCanvas, data) == 0);
};

impl std::ops::Index<(uhalf, uhalf)> for AsciiCanvas {
    type Output = u8;

    #[inline]
    fn index(&self, (x, y): (uhalf, uhalf)) -> &Self::Output {
        self.get(x, y).unwrap()
    }
}

impl std::ops::IndexMut<(uhalf, uhalf)> for AsciiCanvas {
    #[inline]
    fn index_mut(&mut self, (x, y): (uhalf, uhalf)) -> &mut Self::Output {
        self.get_mut(x, y).unwrap()
    }
}

impl std::ops::Index<(Range<uhalf>, uhalf)> for AsciiCanvas {
    type Output = [u8];

    #[inline]
    fn index(&self, (x, y): (Range<uhalf>, uhalf)) -> &Self::Output {
        self.get_range(x, y).unwrap()
    }
}

impl std::ops::IndexMut<(Range<uhalf>, uhalf)> for AsciiCanvas {
    #[inline]
    fn index_mut(&mut self, (x, y): (Range<uhalf>, uhalf)) -> &mut Self::Output {
        self.get_range_mut(x, y).unwrap()
    }
}

impl std::fmt::Display for AsciiCanvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

pub type Rows<'a> = std::iter::Map<std::slice::ChunksExact<'a, u8>, fn(&[u8]) -> &str>;

impl AsciiCanvas {
    pub const RAMP: &'static [u8] =
        br#" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#;

    const fn value(intensity: f32) -> u8 {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        Self::RAMP[(intensity.clamp(0.0, 1.0) * (Self::RAMP.len() - 1) as f32) as usize]
    }

    const fn color_to_value(color: Color) -> u8 {
        Self::value(
            0.299 * color.r as f32 / 255.0
                + 0.587 * color.g as f32 / 255.0
                + 0.114 * color.b as f32 / 255.0,
        )
    }

    const fn index_of(&self, x: uhalf, y: uhalf) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(area(y, self.width) + x as usize)
        } else {
            None
        }
    }

    const fn index_of_range(&self, x: Range<uhalf>, y: uhalf) -> Option<Range<usize>> {
        match (self.index_of(x.start, y), self.index_of(x.end - 1, y)) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
    }

    const fn data_len(&self) -> usize {
        area(self.width, self.height)
    }

    const fn data_nonnull_slice(&self) -> NonNull<[u8]> {
        NonNull::slice_from_raw_parts(self.data, self.data_len())
    }

    const fn data_slice(&self) -> &[u8] {
        // SAFETY: data is guaranteed to be a reference
        unsafe { self.data_nonnull_slice().as_ref() }
    }

    const fn data_slice_mut(&mut self) -> &mut [u8] {
        // SAFETY: data is guaranteed to be a reference
        unsafe { self.data_nonnull_slice().as_mut() }
    }

    #[inline]
    #[must_use]
    pub const fn get(&self, x: uhalf, y: uhalf) -> Option<&u8> {
        match self.index_of(x, y) {
            Some(idx) => Some(&self.data_slice()[idx]),
            None => None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn get_mut(&mut self, x: uhalf, y: uhalf) -> Option<&mut u8> {
        match self.index_of(x, y) {
            Some(idx) => Some(&mut self.data_slice_mut()[idx]),
            None => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn get_range(&self, x: Range<uhalf>, y: uhalf) -> Option<&[u8]> {
        match self.index_of_range(x, y) {
            Some(range) => Some(&self.data_slice()[range]),
            None => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn get_range_mut(&mut self, x: Range<uhalf>, y: uhalf) -> Option<&mut [u8]> {
        match self.index_of_range(x, y) {
            Some(idx) => Some(&mut self.data_slice_mut()[idx]),
            None => None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> uhalf {
        self.width
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> uhalf {
        self.height
    }

    #[inline]
    pub fn rows(&self) -> Rows<'_> {
        self.data_slice()
            .chunks_exact(self.width as usize)
            .map(|row|
            // SAFETY: ASCII is guaranteed compatible with UTF-8
            unsafe { str::from_utf8_unchecked(row) })
    }

    pub fn clear_background(&mut self, fill_color: Color) {
        self.data_slice_mut().fill(Self::color_to_value(fill_color));
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into())
            && let Some(pixel) = self.get_mut(x, y)
        {
            *pixel = Self::color_to_value(color);
        }
    }

    pub fn draw_pixel_v(&mut self, pos: Vector2, color: Color) {
        #![allow(clippy::cast_possible_truncation)]
        self.draw_pixel((pos.x + 0.5) as i32, (pos.y + 0.5) as i32, color);
    }

    pub fn draw_line(
        &mut self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: Color,
    ) {
        // Calculate differences in coordinates
        let mut short_len = end_pos_y - start_pos_y;
        let mut long_len = end_pos_x - start_pos_x;
        // Determine if the line is more vertical than horizontal
        let y_longer = short_len.abs() > long_len.abs();

        if y_longer {
            // Swap the lengths if the line is more vertical
            std::mem::swap(&mut short_len, &mut long_len);
        }

        // Initialize variables for drawing loop
        let end_val = long_len;

        // Adjust direction increment based on long_len sign
        let sgn_inc = if long_len < 0 { -1 } else { 1 };
        long_len *= sgn_inc;

        // Calculate fixed-point increment for shorter length
        let dec_inc = (short_len << 16).checked_div(long_len).unwrap_or_default();

        // Draw the line pixel by pixel
        if y_longer {
            // If line is more vertical, iterate over y-axis
            let mut i = 0;
            let mut j = 0;
            while i != end_val {
                // Calculate pixel position and draw it
                self.draw_pixel(start_pos_x + (j >> 16), start_pos_y + i, color);
                i += sgn_inc;
                j += dec_inc;
            }
        } else {
            // If line is more horizontal, iterate over x-axis
            let mut i = 0;
            let mut j = 0;
            while i != end_val {
                // Calculate pixel position and draw it
                self.draw_pixel(start_pos_x + i, start_pos_y + (j >> 16), color);
                i += sgn_inc;
                j += dec_inc;
            }
        }
    }

    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        #![allow(clippy::cast_possible_truncation)]
        // Round start and end positions to nearest integer coordinates
        let x1 = (start.x + 0.5) as i32;
        let y1 = (start.y + 0.5) as i32;
        let x2 = (end.x + 0.5) as i32;
        let y2 = (end.y + 0.5) as i32;

        // Draw a vertical line using ImageDrawLine function
        self.draw_line(x1, y1, x2, y2, color);
    }

    pub fn draw_triangle_ex(
        &mut self,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        c1: Color,
        c2: Color,
        c3: Color,
    ) {
        #![allow(clippy::similar_names, reason = "i disagree")]

        // Calculate the 2D bounding box of the triangle
        // Determine the minimum and maximum x and y coordinates of the triangle vertices

        #[allow(clippy::cast_possible_truncation)]
        let (x_min, y_min, x_max, y_max) = (
            (v1.x.min(v2.x).min(v3.x) as i32).clamp(0, self.width.try_into().unwrap_or(i32::MAX)),
            (v1.y.min(v2.y).min(v3.y) as i32).clamp(0, self.height.try_into().unwrap_or(i32::MAX)),
            (v1.x.max(v2.x).max(v3.x) as i32).clamp(0, self.width.try_into().unwrap_or(i32::MAX)),
            (v1.y.max(v2.y).max(v3.y) as i32).clamp(0, self.height.try_into().unwrap_or(i32::MAX)),
        );

        // Check the order of the vertices to determine if it's a front or back face
        // NOTE: if signedArea is equal to 0, the face is degenerate
        let signed_area = (v2.x - v1.x) * (v3.y - v1.y) - (v3.x - v1.x) * (v2.y - v1.y);
        let is_back_face = signed_area > 0.0;

        // Barycentric interpolation setup
        // Calculate the step increments for the barycentric coordinates
        #[allow(clippy::cast_possible_truncation)]
        let (
            mut w1_x_step,
            mut w1_y_step,
            mut w2_x_step,
            mut w2_y_step,
            mut w3_x_step,
            mut w3_y_step,
        ) = (
            (v3.y - v2.y) as i32,
            (v2.x - v3.x) as i32,
            (v1.y - v3.y) as i32,
            (v3.x - v1.x) as i32,
            (v2.y - v1.y) as i32,
            (v1.x - v2.x) as i32,
        );

        // If the triangle is a back face, invert the steps
        if is_back_face {
            w1_x_step = -w1_x_step;
            w1_y_step = -w1_y_step;
            w2_x_step = -w2_x_step;
            w2_y_step = -w2_y_step;
            w3_x_step = -w3_x_step;
            w3_y_step = -w3_y_step;
        }

        // Calculate the initial barycentric coordinates for the top-left point of the bounding box
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let (mut w1_row, mut w2_row, mut w3_row) = (
            ((x_min as f32 - v2.x) * w1_x_step as f32 + w1_y_step as f32 * (y_min as f32 - v2.y))
                as i32,
            ((x_min as f32 - v3.x) * w2_x_step as f32 + w2_y_step as f32 * (y_min as f32 - v3.y))
                as i32,
            ((x_min as f32 - v1.x) * w3_x_step as f32 + w3_y_step as f32 * (y_min as f32 - v1.y))
                as i32,
        );

        // Calculate the inverse of the sum of the barycentric coordinates for normalization
        // NOTE 1: Here, we act as if we multiply by 255 the reciprocal, which avoids additional
        //         calculations in the loop. This is acceptable because we are only interpolating colors
        // NOTE 2: This sum remains constant throughout the triangle
        #[allow(clippy::cast_precision_loss)]
        let w_inv_sum = 255.0 / (w1_row + w2_row + w3_row) as f32;

        // Rasterization loop
        // Iterate through each pixel in the bounding box
        for y in y_min..=y_max {
            let mut w1 = w1_row;
            let mut w2 = w2_row;
            let mut w3 = w3_row;

            for x in x_min..=x_max {
                // Check if the pixel is inside the triangle using barycentric coordinates
                if (w1 | w2 | w3) >= 0 {
                    // Compute the normalized barycentric coordinates
                    #[allow(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        clippy::cast_precision_loss
                    )]
                    let (a_w1, a_w2, a_w3) = (
                        (w1 as f32 * w_inv_sum) as u32,
                        (w2 as f32 * w_inv_sum) as u32,
                        (w3 as f32 * w_inv_sum) as u32,
                    );

                    // Interpolate the color using the barycentric coordinates
                    let final_color = Color {
                        r: ((u32::from(c1.r) * a_w1
                            + u32::from(c2.r) * a_w2
                            + u32::from(c3.r) * a_w3)
                            / 255)
                            .clamp(0, 255) as u8,
                        g: ((u32::from(c1.g) * a_w1
                            + u32::from(c2.g) * a_w2
                            + u32::from(c3.g) * a_w3)
                            / 255)
                            .clamp(0, 255) as u8,
                        b: ((u32::from(c1.b) * a_w1
                            + u32::from(c2.b) * a_w2
                            + u32::from(c3.b) * a_w3)
                            / 255)
                            .clamp(0, 255) as u8,
                        a: ((u32::from(c1.a) * a_w1
                            + u32::from(c2.a) * a_w2
                            + u32::from(c3.a) * a_w3)
                            / 255)
                            .clamp(0, 255) as u8,
                    };

                    // Draw the pixel with the interpolated color
                    self.draw_pixel(x, y, final_color);
                }

                // Increment the barycentric coordinates for the next pixel
                w1 += w1_x_step;
                w2 += w2_x_step;
                w3 += w3_x_step;
            }

            // Move to the next row in the bounding box
            w1_row += w1_y_step;
            w2_row += w2_y_step;
            w3_row += w3_y_step;
        }
    }

    pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        // Security check to avoid program crash
        if self.width == 0 || self.height == 0 {
            return;
        }

        #[allow(clippy::cast_precision_loss)]
        let (self_width_f, self_height_f) = (self.width as f32, self.height as f32);

        // Security check to avoid drawing out of bounds in case of bad user data
        // and clamp the size the the image bounds
        let x_min = rec.x.max(0.0);
        let y_min = rec.y.max(0.0);
        let x_max = (rec.x + rec.width).min(self_width_f);
        let y_max = (rec.y + rec.height).min(self_height_f);

        // Check if the rect is even inside the image
        if (x_max <= 0.0)
            || (y_max <= 0.0)
            || (x_min >= self_width_f)
            || (y_min >= self_height_f)
            || x_max <= x_min
            || y_max <= y_min
        {
            return;
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (sx, sy, ex, ey) = (
            x_min as uhalf,
            y_min as uhalf,
            x_max as uhalf,
            y_max as uhalf,
        );

        // Fill in the first pixel of the first row based on image format
        self[(sx, sy)] = Self::color_to_value(color);

        let p_src_pixel = self[(sx, sy)];

        // Repeat the first pixel data throughout the row
        for x in (sx + 1)..ex {
            self[(x, sy)] = p_src_pixel;
        }

        // Repeat the first row data for all other rows
        for y in (sy + 1)..ey {
            let src_range = (sy as usize * self.width as usize + sx as usize)
                ..(sy as usize * self.width as usize + ex as usize);
            let dst_start = y as usize * self.width as usize + sx as usize;
            self.data_slice_mut().copy_within(src_range, dst_start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_pixel() {
        let mut canvas = AsciiCanvasing::new();
        canvas.resize(4, 4, Color::GRAY);
        canvas.draw_pixel(2, 3, Color::WHITE);
        assert_eq!(
            &canvas.to_string(),
            "\
            xxxx\n\
            xxxx\n\
            xxxx\n\
            xx$x\n\
            "
        );
    }

    #[test]
    fn test_draw_triangle() {
        let mut canvas = AsciiCanvasing::new();
        canvas.resize(12, 8, Color::BLACK);
        canvas.draw_triangle_ex(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 7.0),
            Vector2::new(std::f32::consts::SQRT_2 * 8.0, 3.5),
            Color::RED,
            Color::GREEN,
            Color::BLUE,
        );
        print!("{canvas}");
    }

    #[test]
    fn test_draw_rectangle() {
        let mut canvas = AsciiCanvasing::new();
        canvas.resize(8, 8, Color::GRAY);
        canvas.draw_rectangle_rec(Rectangle::new(2.0, 1.0, 6.0, 3.0), Color::WHITE);
        assert_eq!(
            &canvas.to_string(),
            "\
            xxxxxxxx\n\
            xx$$$$$$\n\
            xx$$$$$$\n\
            xx$$$$$$\n\
            xxxxxxxx\n\
            xxxxxxxx\n\
            xxxxxxxx\n\
            xxxxxxxx\n\
            "
        );
    }
}
