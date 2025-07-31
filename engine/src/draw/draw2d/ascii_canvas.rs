#![allow(dead_code)]

use raylib::prelude::*;
use std::ptr::NonNull;

#[cfg(target_pointer_width = "16")]
#[allow(non_camel_case_types)]
type uhalf = u8;

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
type uhalf = u16;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type uhalf = u32;

/// [`AsciiCanvasing`] is to [`AsciiCanvas`] as [`String`] is to [`str`]
#[derive(Debug, PartialEq, Eq)]
pub struct AsciiCanvasing {
    capacity: usize,
    canvas: AsciiCanvas,
}

impl Drop for AsciiCanvasing {
    fn drop(&mut self) {
        drop(self.data_vec());
    }
}

impl std::ops::Deref for AsciiCanvasing {
    type Target = AsciiCanvas;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl std::ops::DerefMut for AsciiCanvasing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}

impl AsRef<AsciiCanvas> for AsciiCanvasing {
    fn as_ref(&self) -> &AsciiCanvas {
        self
    }
}

impl AsMut<AsciiCanvas> for AsciiCanvasing {
    fn as_mut(&mut self) -> &mut AsciiCanvas {
        self
    }
}

impl std::fmt::Display for AsciiCanvasing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl AsciiCanvasing {
    #[must_use = "dropping the returned vec will make `self.canvas.data` dangle"]
    fn data_vec(&mut self) -> Vec<u8> {
        let size = self.canvas.width as usize * self.canvas.height as usize;
        // SAFETY: reassembling the vec from before
        unsafe { Vec::from_parts(self.canvas.data, size, self.capacity) }
    }

    pub const fn new() -> Self {
        let mut vec = Vec::new();
        // SAFETY: Vec pointer is guaranteed to be non-null
        let data = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };
        std::mem::forget(vec); // We'll free it in the destructor
        Self {
            capacity: 0,
            canvas: AsciiCanvas {
                width: 0,
                height: 0,
                data,
            },
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            canvas: AsciiCanvas {
                width: 0,
                height: 0,
                data: Vec::with_capacity(capacity).into_parts().0,
            },
        }
    }

    pub fn resize(&mut self, new_width: uhalf, new_height: uhalf, fill_with: u8) {
        let new_size = new_width as usize * new_height as usize;
        if let Some(additional) = new_size.checked_sub(self.capacity) {
            let mut vec = self.data_vec();
            vec.resize(additional, fill_with);
            (self.data, _, self.capacity) = vec.into_parts();
        }
        for y in (0..(self.height as usize)).rev() {
            // SAFETY: idk
            let src = unsafe { self.canvas.data.add(y * self.width as usize) };
            // SAFETY: idk
            let dst = unsafe { self.canvas.data.add(y * new_width as usize) };
            // SAFETY: idk
            unsafe {
                dst.copy_from(src, self.width as usize);
            }
        }
        self.width = new_width;
        self.height = new_height;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AsciiCanvas {
    width: uhalf,
    height: uhalf,
    data: NonNull<u8>,
}

impl std::ops::Index<(uhalf, uhalf)> for AsciiCanvas {
    type Output = u8;

    fn index(&self, (x, y): (uhalf, uhalf)) -> &Self::Output {
        self.get(x, y).unwrap()
    }
}

impl std::ops::IndexMut<(uhalf, uhalf)> for AsciiCanvas {
    fn index_mut(&mut self, (x, y): (uhalf, uhalf)) -> &mut Self::Output {
        self.get_mut(x, y).unwrap()
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
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }

    const fn data_len(&self) -> usize {
        self.width as usize * self.height as usize
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

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into())
            && let Some(pixel) = self.get_mut(x, y)
        {
            *pixel = Self::color_to_value(color);
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut canvas = AsciiCanvasing::new();
        canvas.resize(4, 4, b' ');
        canvas.draw_pixel(2, 3, Color::WHITE);
        // print!("{canvas}");
        for (y, row) in canvas.rows().enumerate() {
            for (x, col) in row.bytes().enumerate() {
                assert_eq!(col, if x == 2 && y == 3 { b'$' } else { b' ' });
            }
        }
    }
}
