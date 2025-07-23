//! Fixed point number library

#![warn(missing_docs)]
#![feature(const_ops, const_trait_impl, maybe_uninit_slice)]

use std::{mem::MaybeUninit, ops::*};

macro_rules! define_fp {
    (
        $Name:ident($Repr:ty)
    ) => {
        /// [`FixedPoint`] with 32 integer bits and 32 fractional bits
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $Name($Repr);

        impl std::fmt::Binary for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:b}.{:>02$b}",
                    self.0.cast_unsigned() >> Self::DECIMAL_BITS,
                    self.0.cast_unsigned() & Self::DECIMAL_MASK,
                    Self::DECIMAL_BITS.try_into().unwrap(),
                )
            }
        }

        impl std::fmt::UpperHex for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:X}.{:>02$X}",
                    self.0.cast_unsigned() >> Self::DECIMAL_BITS,
                    self.0.cast_unsigned() & Self::DECIMAL_MASK,
                    (Self::DECIMAL_BITS >> 2).try_into().unwrap(),
                )
            }
        }

        impl std::fmt::LowerHex for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:x}.{:>02$x}",
                    self.0.cast_unsigned() >> Self::DECIMAL_BITS,
                    self.0.cast_unsigned() & Self::DECIMAL_MASK,
                    (Self::DECIMAL_BITS >> 2).try_into().unwrap(),
                )
            }
        }

        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                const MAX_DIGITS: usize = 32;
                let sign = if self.0.is_negative() { "-" } else { "" };
                let ipart = (self.0.cast_unsigned() & Self::INTEGER_MASK)
                    .cast_signed()
                    .unsigned_abs()
                    >> Self::DECIMAL_BITS;
                let mut fbits = self.0.cast_unsigned() & Self::DECIMAL_MASK;
                let mut buf = [MaybeUninit::uninit(); MAX_DIGITS];
                let mut buf_len = 0;
                for digit in buf.iter_mut().take(f.precision().unwrap_or(MAX_DIGITS)) {
                    fbits *= 10;
                    digit.write(
                        b'0' + u8::try_from((fbits / Self::DECIMAL_FACTOR_INT) % 10).unwrap(),
                    );
                    buf_len += 1;
                    fbits &= Self::DECIMAL_MASK;
                    if f.precision().is_none() && fbits == 0 {
                        break;
                    }
                }
                let fpart = unsafe { str::from_utf8_unchecked(buf[0..buf_len].assume_init_ref()) };
                write!(f, "{sign}{ipart}.{fpart}")
            }
        }

        impl $Name {
            /// 0
            pub const ZERO: Self = Self::from_i32(0);
            /// 1
            pub const ONE: Self = Self::from_i32(1);
            /// -1
            pub const NEG_ONE: Self = Self::from_i32(-1);
            /// The minimum expressible value
            pub const MIN: Self = Self(i64::MIN);
            /// The maximum expressible value
            pub const MAX: Self = Self(i64::MAX);

            const DECIMAL_BITS: u32 = 32;
            const DECIMAL_FACTOR_INT: u64 = 1 << Self::DECIMAL_BITS;
            const DECIMAL_FACTOR_ISQRT: u64 = Self::DECIMAL_FACTOR_INT.isqrt();
            const DECIMAL_MASK: u64 = Self::DECIMAL_FACTOR_INT - 1;
            const INTEGER_MASK: u64 = !Self::DECIMAL_MASK;
            const DECIMAL_FACTOR: f64 = Self::DECIMAL_FACTOR_INT as f64;
            const DECIMAL_INV_FACTOR: f64 = Self::DECIMAL_FACTOR.recip();

            /// Gives the lowest and highest values `value` may become after conversion
            pub const fn precision(target: f32) -> RangeInclusive<f32> {
                (target as f64 - $Name::DECIMAL_INV_FACTOR) as f32
                    ..=(target as f64 + $Name::DECIMAL_INV_FACTOR) as f32
            }

            /// Construct a fixed point value from integer and fractional bits
            #[inline]
            pub const fn new(ipart: i32, fpart: u32) -> Self {
                Self(((ipart as i64) << Self::DECIMAL_BITS) | fpart as i64)
            }

            /// Construct an integer fixed point value
            #[inline]
            pub const fn from_i32(value: i32) -> Self {
                Self((value as i64) << Self::DECIMAL_BITS)
            }

            /// Convert a fixed point to an integer, truncrating the fractional part
            #[inline]
            pub const fn to_i32(self) -> i32 {
                (self.0 >> Self::DECIMAL_BITS) as i32
            }

            /// Convert a fixed point to a floating point
            #[inline]
            pub const fn from_f32(value: f32) -> Self {
                Self((value as f64 * Self::DECIMAL_FACTOR) as i64)
            }

            #[inline]
            pub const fn to_f32(self) -> f32 {
                (self.0 as f64 * Self::DECIMAL_INV_FACTOR) as f32
            }

            #[inline]
            pub const fn abs(self) -> Self {
                Self(self.0.abs())
            }

            #[inline]
            pub const fn negate(self) -> Self {
                Self(-self.0)
            }

            #[inline]
            pub const fn plus(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }

            #[inline]
            pub const fn minus(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }

            #[inline]
            pub const fn multiply(self, rhs: Self) -> Self {
                Self(((self.0 as i128 * rhs.0 as i128) >> Self::DECIMAL_BITS) as i64)
            }

            #[inline]
            pub const fn sqrt(self) -> Self {
                Self(self.0.isqrt() * Self::DECIMAL_FACTOR_ISQRT as i64)
            }
        }

        impl From<f32> for $Name {
            #[inline]
            fn from(value: f32) -> Self {
                $Name::from_f32(value)
            }
        }

        impl From<$Name> for f32 {
            #[inline]
            fn from(value: $Name) -> Self {
                value.to_f32()
            }
        }

        impl From<i32> for $Name {
            #[inline]
            fn from(value: i32) -> Self {
                Self::from_i32(value)
            }
        }

        impl Neg for $Name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                self.negate()
            }
        }

        impl const Add for $Name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                self.plus(rhs)
            }
        }

        impl AddAssign for $Name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = self.add(rhs)
            }
        }

        impl Add<i32> for $Name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: i32) -> Self::Output {
                self.add(Self::from(rhs))
            }
        }

        impl Add<$Name> for i32 {
            type Output = $Name;

            #[inline]
            fn add(self, rhs: $Name) -> Self::Output {
                $Name::from_i32(self).add(rhs)
            }
        }

        impl AddAssign<i32> for $Name {
            #[inline]
            fn add_assign(&mut self, rhs: i32) {
                *self = self.add(rhs)
            }
        }

        impl Add<f32> for $Name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: f32) -> Self::Output {
                self.add(Self::from(rhs))
            }
        }

        impl Add<$Name> for f32 {
            type Output = $Name;

            #[inline]
            fn add(self, rhs: $Name) -> Self::Output {
                $Name::from_f32(self).add(rhs)
            }
        }

        impl AddAssign<f32> for $Name {
            #[inline]
            fn add_assign(&mut self, rhs: f32) {
                *self = self.add(rhs)
            }
        }

        impl Sub for $Name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                self.minus(rhs)
            }
        }

        impl SubAssign for $Name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.sub(rhs)
            }
        }

        impl Sub<i32> for $Name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: i32) -> Self::Output {
                self.sub(Self::from(rhs))
            }
        }

        impl Sub<$Name> for i32 {
            type Output = $Name;

            #[inline]
            fn sub(self, rhs: $Name) -> Self::Output {
                $Name::from_i32(self).sub(rhs)
            }
        }

        impl SubAssign<i32> for $Name {
            #[inline]
            fn sub_assign(&mut self, rhs: i32) {
                *self = self.sub(rhs)
            }
        }

        impl Sub<f32> for $Name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: f32) -> Self::Output {
                self.sub(Self::from(rhs))
            }
        }

        impl Sub<$Name> for f32 {
            type Output = $Name;

            #[inline]
            fn sub(self, rhs: $Name) -> Self::Output {
                $Name::from_f32(self).sub(rhs)
            }
        }

        impl SubAssign<f32> for $Name {
            #[inline]
            fn sub_assign(&mut self, rhs: f32) {
                *self = self.sub(rhs)
            }
        }

        impl Mul for $Name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                self.multiply(rhs)
            }
        }

        impl MulAssign for $Name {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = self.mul(rhs)
            }
        }

        impl Mul<i32> for $Name {
            type Output = Self;

            fn mul(self, rhs: i32) -> Self::Output {
                self.mul(Self::from(rhs))
            }
        }

        impl Mul<$Name> for i32 {
            type Output = $Name;

            #[inline]
            fn mul(self, rhs: $Name) -> Self::Output {
                $Name::from_i32(self).sub(rhs)
            }
        }

        impl MulAssign<i32> for $Name {
            #[inline]
            fn mul_assign(&mut self, rhs: i32) {
                *self = self.mul(rhs)
            }
        }

        impl Mul<f32> for $Name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: f32) -> Self::Output {
                self.mul(Self::from(rhs))
            }
        }

        impl Mul<$Name> for f32 {
            type Output = $Name;

            #[inline]
            fn mul(self, rhs: $Name) -> Self::Output {
                $Name::from_f32(self).sub(rhs)
            }
        }

        impl MulAssign<f32> for $Name {
            #[inline]
            fn mul_assign(&mut self, rhs: f32) {
                *self = self.mul(rhs)
            }
        }
    };
}

define_fp!(Q32_32(i64));

#[cfg(test)]
mod test_fixed_point {
    use super::*;

    #[test]
    fn test_f32_sign() {
        let x = Q32_32::from_f32(-1.0);
        assert!(x.0 < 0, "negative value should produce negative fp");
        const NEG_1: Q32_32 = Q32_32::from_i32(-1);
        assert_eq!(
            x, NEG_1,
            "-1 should equal -1\nexpect: {:b}\nactual: {:b}",
            NEG_1.0, x.0
        );
        let y = Q32_32::from_i32(-1).to_f32();
        assert_eq!(
            y, -1.0,
            "-1 should equal -1\nexpect: {}\nactual: {}",
            -1.0, y
        );
    }

    #[test]
    fn test_f32_frac() {
        let expected: f32 = 0.5;
        let expected_range: RangeInclusive<f32> = Q32_32::precision(expected);
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            expected_range.contains(&x),
            "should be symmetric\nexpect: {expected_range:?}\nactual: {x}"
        );

        let expected: f32 = 2.2;
        let expected_range: RangeInclusive<f32> = Q32_32::precision(expected);
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            expected_range.contains(&x),
            "should be symmetric\nexpect: {expected_range:?}\nactual: {x}"
        );

        let expected: f32 = -2.2;
        let expected_range: RangeInclusive<f32> = Q32_32::precision(expected);
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            expected_range.contains(&x),
            "should be symmetric\nexpect: {expected_range:?}\nactual: {x}"
        );
    }

    #[test]
    fn test_i32_sign() {
        let x = Q32_32::from_i32(-1);
        assert!(x.0 < 0, "negative value should produce negative fp");
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Q32_32::from_i32(-5) * Q32_32::from_i32(-5),
            Q32_32::from_i32(25)
        );
        assert_eq!(
            Q32_32::from_f32(-0.5) * Q32_32::from_f32(-0.5),
            Q32_32::from_f32(0.25)
        );
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(Q32_32::from_i32(100).sqrt(), Q32_32::from_i32(10));
    }

    #[test]
    fn test_fmt() {
        for ((ipart, fpart), expect) in [
            ((0, 0), "0.0"),
            ((100, 0), "100.0"),
            ((-100, 0), "-100.0"),
            ((5, (Q32_32::DECIMAL_FACTOR_INT / 2) as u32), "5.5"),
            ((1, 1), "1.00000000023283064365386962890625"),
            ((-100, 645566574), "-100.1503076809458434581756591796875"),
        ] {
            let actual = Q32_32::new(ipart, fpart).to_string();
            assert_eq!(&actual, expect);
        }
        let actual = format!("{:.3}", Q32_32::new(-100, 645566574));
        assert_eq!(&actual, "-100.150");
    }
}
