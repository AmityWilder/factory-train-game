use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Q48_16(i64);

impl Q48_16 {
    const DECIMAL_MASK: i64 = 0xFFFF;
    const DECIMAL_FACTOR: f64 = Self::DECIMAL_MASK as f64;
    const DECIMAL_INV_FACTOR: f64 = Self::DECIMAL_FACTOR.recip();
    const DECIMAL_BITS: u32 = Self::DECIMAL_MASK.count_ones();

    /// Gives the lowest and highest values `value` may become after conversion
    pub const fn precision(target: f32) -> RangeInclusive<f32> {
        (target as f64 - Q48_16::DECIMAL_INV_FACTOR) as f32..=(target as f64 + Q48_16::DECIMAL_INV_FACTOR) as f32
    }

    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        Self((value as i64) << Self::DECIMAL_BITS)
    }

    #[inline]
    pub fn from_f32(value: f32) -> Self {
        let ipart = (value as i64) << Self::DECIMAL_BITS;
        let fpart = ((value as f64).fract() * Self::DECIMAL_FACTOR) as i64;
        Self(ipart | fpart)
    }

    #[inline]
    pub const fn to_f32(self) -> f32 {
        ((self.0 >> Self::DECIMAL_BITS) as f64 + (self.0 & Self::DECIMAL_MASK) as f64 * Self::DECIMAL_INV_FACTOR) as f32
    }

    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl From<f32> for Q48_16 {
    #[inline]
    fn from(value: f32) -> Self {
        Q48_16::from_f32(value)
    }
}

impl From<Q48_16> for f32 {
    #[inline]
    fn from(value: Q48_16) -> Self {
        value.to_f32()
    }
}

impl Add for Q48_16 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl Sub for Q48_16 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl AddAssign for Q48_16 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs)
    }
}

impl SubAssign for Q48_16 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs)
    }
}

#[cfg(test)]
mod test_fixed_point {
    use super::*;

    #[test]
    fn test_f32_sign() {
        let x = Q48_16::from_f32(-1.0);
        assert!(x.0 < 0, "negative value should produce negative fp");
        const NEG_1: Q48_16 = Q48_16::from_i32(-1);
        assert_eq!(x, NEG_1, "-1 should equal -1\nexpect: {:b}\nactual: {:b}", NEG_1.0, x.0);
        let y = Q48_16::from_i32(-1).to_f32();
        assert_eq!(y, -1.0, "-1 should equal -1\nexpect: {}\nactual: {}", -1.0, y);
    }

    #[test]
    fn test_f32_frac() {
        const EXPECTED: f32 = 0.5;
        const EXPECTED_RANGE: RangeInclusive<f32> = Q48_16::precision(EXPECTED);
        let x = Q48_16::from_f32(EXPECTED).to_f32();
        assert!(EXPECTED_RANGE.contains(&x), "should be symmetric\nexpect: {EXPECTED_RANGE:?}\nactual: {x}");
    }

    #[test]
    fn test_i32_sign() {
        let x = Q48_16::from_i32(-1);
        assert!(x.0 < 0, "negative value should produce negative fp");
    }
}
