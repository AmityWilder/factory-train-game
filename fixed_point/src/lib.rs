//! Fixed point number library

#![warn(missing_docs)]
#![feature(const_ops, const_trait_impl, maybe_uninit_slice)]

use std::{mem::MaybeUninit, ops::*};

macro_rules! define_fp {
    (
        ibits: $IBITS:literal,
        fbits: $FBITS:literal,
        ipart: $IPart:ty,
        fpart: $FPart:ty,
        repr: $Repr:ty,
        urepr: $URepr:ty,
    ) => {
        paste::paste!{
            const _: () = {
                assert!(std::mem::size_of::<$Repr>()*8 == $IBITS + $FBITS);
            };

            #[doc = concat!(r"Fixed point with ", $IBITS, " integer bits and ", $FBITS, " fractional bits")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            pub struct [<Q $IBITS _ $FBITS>]($Repr);

            impl std::fmt::Binary for [<Q $IBITS _ $FBITS>] {
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

            impl std::fmt::UpperHex for [<Q $IBITS _ $FBITS>] {
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

            impl std::fmt::LowerHex for [<Q $IBITS _ $FBITS>] {
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

            impl std::fmt::Display for [<Q $IBITS _ $FBITS>] {
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

            impl [<Q $IBITS _ $FBITS>] {
                /// 0
                pub const ZERO: Self = Self::[<from_ $IPart>](0);
                /// 1
                pub const ONE: Self = Self::[<from_ $IPart>](1);
                /// -1
                pub const NEG_ONE: Self = Self::[<from_ $IPart>](-1);
                /// The minimum expressible value
                pub const MIN: Self = Self($Repr::MIN);
                /// The maximum expressible value
                pub const MAX: Self = Self($Repr::MAX);

                const DECIMAL_BITS: u32 = $FBITS;
                const DECIMAL_FACTOR_INT: $URepr = 1 << Self::DECIMAL_BITS;
                const DECIMAL_FACTOR_ISQRT: $URepr = Self::DECIMAL_FACTOR_INT.isqrt();
                const DECIMAL_MASK: $URepr = Self::DECIMAL_FACTOR_INT - 1;
                const INTEGER_MASK: $URepr = !Self::DECIMAL_MASK;
                const DECIMAL_FACTOR: f64 = Self::DECIMAL_FACTOR_INT as f64;
                const DECIMAL_INV_FACTOR: f64 = Self::DECIMAL_FACTOR.recip();

                /// Construct a fixed point value from integer and fractional bits
                #[inline]
                pub const fn new(ipart: $IPart, fpart: $FPart) -> Self {
                    Self(((ipart as $Repr) << Self::DECIMAL_BITS) | fpart as $Repr)
                }

                /// Construct an integer fixed point value
                #[inline]
                pub const fn [<from_ $IPart>](value: $IPart) -> Self {
                    Self((value as $Repr) << Self::DECIMAL_BITS)
                }

                /// Convert a fixed point to an integer, truncrating the fractional part
                #[inline]
                pub const fn [<to_ $IPart>](self) -> $IPart {
                    (self.0 >> Self::DECIMAL_BITS) as $IPart
                }

                /// Construct a fixed point from a floating point
                #[inline]
                pub const fn from_f32(value: f32) -> Self {
                    Self((value as f64 * Self::DECIMAL_FACTOR) as $Repr)
                }

                /// Convert a fixed point to a floating point
                #[inline]
                pub const fn to_f32(self) -> f32 {
                    (self.0 as f64 * Self::DECIMAL_INV_FACTOR) as f32
                }

                /// Get the absolute value of `self`
                #[inline]
                pub const fn abs(self) -> Self {
                    Self(self.0.abs())
                }

                /// Get the negative of `self`
                #[inline]
                pub const fn negate(self) -> Self {
                    Self(-self.0)
                }

                /// Add `rhs` to `self`
                #[inline]
                pub const fn plus(self, rhs: Self) -> Self {
                    Self(self.0 + rhs.0)
                }

                /// Subtract `rhs` from `self`
                #[inline]
                pub const fn minus(self, rhs: Self) -> Self {
                    Self(self.0 - rhs.0)
                }

                /// Multiply `self` by `rhs`
                #[inline]
                pub const fn multiply(self, rhs: Self) -> Self {
                    Self(((self.0 as i128 * rhs.0 as i128) >> Self::DECIMAL_BITS) as $Repr)
                }

                /// Calculate the square root of `self`
                #[inline]
                pub const fn sqrt(self) -> Self {
                    Self(self.0.isqrt() * Self::DECIMAL_FACTOR_ISQRT as $Repr)
                }
            }

            impl Neg for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self::Output {
                    self.negate()
                }
            }

            impl const Add for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    self.plus(rhs)
                }
            }

            impl AddAssign for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    *self = self.add(rhs)
                }
            }

            impl Add<$IPart> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $IPart) -> Self::Output {
                    self.add(Self::[<from_ $IPart>](rhs))
                }
            }

            impl Add<[<Q $IBITS _ $FBITS>]> for $IPart {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn add(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::[<from_ $IPart>](self).add(rhs)
                }
            }

            impl AddAssign<$IPart> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn add_assign(&mut self, rhs: $IPart) {
                    *self = self.add(rhs)
                }
            }

            impl Add<f32> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn add(self, rhs: f32) -> Self::Output {
                    self.add(Self::from_f32(rhs))
                }
            }

            impl Add<[<Q $IBITS _ $FBITS>]> for f32 {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn add(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::from_f32(self).add(rhs)
                }
            }

            impl AddAssign<f32> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn add_assign(&mut self, rhs: f32) {
                    *self = self.add(rhs)
                }
            }

            impl Sub for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    self.minus(rhs)
                }
            }

            impl SubAssign for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    *self = self.sub(rhs)
                }
            }

            impl Sub<$IPart> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $IPart) -> Self::Output {
                    self.sub(Self::[<from_ $IPart>](rhs))
                }
            }

            impl Sub<[<Q $IBITS _ $FBITS>]> for $IPart {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn sub(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::[<from_ $IPart>](self).sub(rhs)
                }
            }

            impl SubAssign<$IPart> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn sub_assign(&mut self, rhs: $IPart) {
                    *self = self.sub(rhs)
                }
            }

            impl Sub<f32> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: f32) -> Self::Output {
                    self.sub(Self::from_f32(rhs))
                }
            }

            impl Sub<[<Q $IBITS _ $FBITS>]> for f32 {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn sub(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::from_f32(self).sub(rhs)
                }
            }

            impl SubAssign<f32> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn sub_assign(&mut self, rhs: f32) {
                    *self = self.sub(rhs)
                }
            }

            impl Mul for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    self.multiply(rhs)
                }
            }

            impl MulAssign for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn mul_assign(&mut self, rhs: Self) {
                    *self = self.mul(rhs)
                }
            }

            impl Mul<$IPart> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                fn mul(self, rhs: $IPart) -> Self::Output {
                    self.mul(Self::[<from_ $IPart>](rhs))
                }
            }

            impl Mul<[<Q $IBITS _ $FBITS>]> for $IPart {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn mul(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::[<from_ $IPart>](self).sub(rhs)
                }
            }

            impl MulAssign<$IPart> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn mul_assign(&mut self, rhs: $IPart) {
                    *self = self.mul(rhs)
                }
            }

            impl Mul<f32> for [<Q $IBITS _ $FBITS>] {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: f32) -> Self::Output {
                    self.mul(Self::from_f32(rhs))
                }
            }

            impl Mul<[<Q $IBITS _ $FBITS>]> for f32 {
                type Output = [<Q $IBITS _ $FBITS>];

                #[inline]
                fn mul(self, rhs: [<Q $IBITS _ $FBITS>]) -> Self::Output {
                    [<Q $IBITS _ $FBITS>]::from_f32(self).sub(rhs)
                }
            }

            impl MulAssign<f32> for [<Q $IBITS _ $FBITS>] {
                #[inline]
                fn mul_assign(&mut self, rhs: f32) {
                    *self = self.mul(rhs)
                }
            }
        }
    };
}

define_fp!(
    ibits: 16,
    fbits: 16,
    ipart: i16,
    fpart: u16,
    repr: i32,
    urepr: u32,
);

define_fp!(
    ibits: 32,
    fbits: 32,
    ipart: i32,
    fpart: u32,
    repr: i64,
    urepr: u64,
);

define_fp!(
    ibits: 48,
    fbits: 16,
    ipart: i32,
    fpart: u32,
    repr: i64,
    urepr: u64,
);

define_fp!(
    ibits: 64,
    fbits: 64,
    ipart: i64,
    fpart: u64,
    repr: i128,
    urepr: u128,
);

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
        let epsilon = 0.001;
        let expected: f32 = 0.5;
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            (x - expected).abs() <= epsilon,
            "should be symmetric\nexpect: {expected}±{epsilon}\nactual: {x}"
        );

        let expected: f32 = 2.2;
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            (x - expected).abs() <= epsilon,
            "should be symmetric\nexpect: {expected}±{epsilon}\nactual: {x}"
        );

        let expected: f32 = -2.2;
        let x = Q32_32::from_f32(expected).to_f32();
        assert!(
            (x - expected).abs() <= epsilon,
            "should be symmetric\nexpect: {expected}±{epsilon}\nactual: {x}"
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
