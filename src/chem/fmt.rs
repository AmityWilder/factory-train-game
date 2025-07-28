use arrayvec::ArrayString;
use std::fmt::Write;

#[const_trait]
pub trait SubSupScript: Sized {
    type Output: Sized;

    /// Convert a unicode character to its superscript equivalent
    ///
    /// Returns [`None`] if the character has no superscript version
    #[must_use]
    fn to_superscript(self) -> Option<Self::Output>;

    /// Convert a unicode character to its subscript equivalent
    ///
    /// Returns [`None`] if the character has no subscript version
    #[must_use]
    fn to_subscript(self) -> Option<Self::Output>;
}

/// Only works for `0`-`9`, `+`, `-`, `=`, `(`, and `)`
impl const SubSupScript for char {
    type Output = char;

    #[inline]
    fn to_superscript(self) -> Option<Self::Output> {
        match self {
            '0' => Some('⁰'),
            '1' => Some('¹'),
            '2' => Some('²'),
            '3' => Some('³'),
            '4' => Some('⁴'),
            '5' => Some('⁵'),
            '6' => Some('⁶'),
            '7' => Some('⁷'),
            '8' => Some('⁸'),
            '9' => Some('⁹'),
            '+' => Some('⁺'),
            '-' => Some('⁻'),
            '=' => Some('⁼'),
            '(' => Some('⁽'),
            ')' => Some('⁾'),
            _ => None,
        }
    }

    #[inline]
    fn to_subscript(self) -> Option<Self::Output> {
        match self {
            '0' => Some('₀'),
            '1' => Some('₁'),
            '2' => Some('₂'),
            '3' => Some('₃'),
            '4' => Some('₄'),
            '5' => Some('₅'),
            '6' => Some('₆'),
            '7' => Some('₇'),
            '8' => Some('₈'),
            '9' => Some('₉'),
            '+' => Some('₊'),
            '-' => Some('₋'),
            '=' => Some('₌'),
            '(' => Some('₍'),
            ')' => Some('₎'),
            _ => None,
        }
    }
}

pub trait Superscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Superscript for char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_superscript()
            .ok_or(std::fmt::Error)
            .and_then(|ch| f.write_char(ch))
    }
}

impl Superscript for str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.chars() {
            Superscript::fmt(&ch, f)?;
        }
        Ok(())
    }
}

pub trait Subscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Subscript for char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_subscript()
            .ok_or(std::fmt::Error)
            .and_then(|ch| f.write_char(ch))
    }
}

impl Subscript for str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.chars() {
            Subscript::fmt(&ch, f)?;
        }
        Ok(())
    }
}

macro_rules! impl_num_scripts {
    ($($T:ty),* $(,)?) => {$(
        impl Superscript for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut buf = ArrayString::<64>::new();
                write!(buf, "{}", self)?;
                Superscript::fmt(buf.as_str(), f)
            }
        }
        impl Subscript for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut buf = ArrayString::<64>::new();
                write!(buf, "{}", self)?;
                Subscript::fmt(buf.as_str(), f)
            }
        }
    )*};
}

impl_num_scripts! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64
}
