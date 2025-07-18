#![feature(const_ops, const_trait_impl)]

use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Q32_32(i64);

impl PartialEq<i32> for Q32_32 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.eq(&Self::from_i32(*other))
    }
}

impl PartialOrd<i32> for Q32_32 {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&Self::from_i32(*other))
    }
}

impl PartialEq<f32> for Q32_32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.eq(&Self::from_f32(*other))
    }
}

impl PartialOrd<f32> for Q32_32 {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&Self::from_f32(*other))
    }
}

impl PartialEq<Q32_32> for i32 {
    #[inline]
    fn eq(&self, other: &Q32_32) -> bool {
        Q32_32::from_i32(*self).eq(other)
    }
}

impl PartialOrd<Q32_32> for i32 {
    #[inline]
    fn partial_cmp(&self, other: &Q32_32) -> Option<std::cmp::Ordering> {
        Q32_32::from_i32(*self).partial_cmp(other)
    }
}

impl PartialEq<Q32_32> for f32 {
    #[inline]
    fn eq(&self, other: &Q32_32) -> bool {
        Q32_32::from_f32(*self).eq(other)
    }
}

impl PartialOrd<Q32_32> for f32 {
    #[inline]
    fn partial_cmp(&self, other: &Q32_32) -> Option<std::cmp::Ordering> {
        Q32_32::from_f32(*self).partial_cmp(other)
    }
}

impl std::fmt::UpperHex for Q32_32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>08X}.{:>08X}",
            self.0.cast_unsigned() >> Self::DECIMAL_BITS,
            self.0.cast_unsigned() & Self::DECIMAL_MASK,
        )
    }
}

impl Q32_32 {
    const DECIMAL_BITS: u32 = 32;
    const DECIMAL_FACTOR_INT: u64 = 1 << Self::DECIMAL_BITS;
    const DECIMAL_MASK: u64 = Self::DECIMAL_FACTOR_INT - 1;
    const DECIMAL_FACTOR: f64 = Self::DECIMAL_FACTOR_INT as f64;
    const DECIMAL_INV_FACTOR: f64 = Self::DECIMAL_FACTOR.recip();

    /// Gives the lowest and highest values `value` may become after conversion
    pub const fn precision(target: f32) -> RangeInclusive<f32> {
        (target as f64 - Q32_32::DECIMAL_INV_FACTOR) as f32
            ..=(target as f64 + Q32_32::DECIMAL_INV_FACTOR) as f32
    }

    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        Self((value as i64) << Self::DECIMAL_BITS)
    }

    #[inline]
    pub fn from_f32(value: f32) -> Self {
        Self((value as f64 * Self::DECIMAL_FACTOR) as i64)
    }

    #[inline]
    pub const fn to_f32(self) -> f32 {
        (self.0 as f64 * Self::DECIMAL_INV_FACTOR) as f32
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
}

impl From<f32> for Q32_32 {
    #[inline]
    fn from(value: f32) -> Self {
        Q32_32::from_f32(value)
    }
}

impl From<Q32_32> for f32 {
    #[inline]
    fn from(value: Q32_32) -> Self {
        value.to_f32()
    }
}

impl From<i32> for Q32_32 {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl const Add for Q32_32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for Q32_32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Add<i32> for Q32_32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        self.add(Self::from(rhs))
    }
}

impl Add<Q32_32> for i32 {
    type Output = Q32_32;

    #[inline]
    fn add(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_i32(self).add(rhs)
    }
}

impl AddAssign<i32> for Q32_32 {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs)
    }
}

impl Add<f32> for Q32_32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self.add(Self::from(rhs))
    }
}

impl Add<Q32_32> for f32 {
    type Output = Q32_32;

    #[inline]
    fn add(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_f32(self).add(rhs)
    }
}

impl AddAssign<f32> for Q32_32 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = self.add(rhs)
    }
}

impl Sub for Q32_32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl SubAssign for Q32_32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl Sub<i32> for Q32_32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        self.sub(Self::from(rhs))
    }
}

impl Sub<Q32_32> for i32 {
    type Output = Q32_32;

    #[inline]
    fn sub(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_i32(self).sub(rhs)
    }
}

impl SubAssign<i32> for Q32_32 {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = self.sub(rhs)
    }
}

impl Sub<f32> for Q32_32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self.sub(Self::from(rhs))
    }
}

impl Sub<Q32_32> for f32 {
    type Output = Q32_32;

    #[inline]
    fn sub(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_f32(self).sub(rhs)
    }
}

impl SubAssign<f32> for Q32_32 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = self.sub(rhs)
    }
}

impl Mul for Q32_32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl MulAssign for Q32_32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

impl Mul<i32> for Q32_32 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.mul(Self::from(rhs))
    }
}

impl Mul<Q32_32> for i32 {
    type Output = Q32_32;

    #[inline]
    fn mul(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_i32(self).sub(rhs)
    }
}

impl MulAssign<i32> for Q32_32 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.mul(rhs)
    }
}

impl Mul<f32> for Q32_32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.mul(Self::from(rhs))
    }
}

impl Mul<Q32_32> for f32 {
    type Output = Q32_32;

    #[inline]
    fn mul(self, rhs: Q32_32) -> Self::Output {
        Q32_32::from_f32(self).sub(rhs)
    }
}

impl MulAssign<f32> for Q32_32 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs)
    }
}

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
}
