/// A 2D cardinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Cardinal2D {
    #[default]
    East  = 0,
    North = 2,
    West  = 4,
    South = 6,
}

impl Cardinal2D {
    #[inline]
    pub const fn as_ordinal(self) -> Ordinal2D {
        match self {
            Self::East  => Ordinal2D::East,
            Self::North => Ordinal2D::North,
            Self::West  => Ordinal2D::West,
            Self::South => Ordinal2D::South,
        }
    }

    /// Add rhs to self
    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        let value = self.as_ordinal().plus(rhs.as_ordinal());

        // SAFETY: Cardinal add/sub guaranteed to result in a cardinal
        unsafe { value.as_cardinal_unchecked() }
    }

    /// Subtract rhs from self
    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        let value = self.as_ordinal().minus(rhs.as_ordinal());

        // SAFETY: Cardinal add/sub guaranteed to result in a cardinal
        unsafe { value.as_cardinal_unchecked() }
    }
}

impl std::ops::Sub for Cardinal2D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Ordinal2D {
    #[default]
    East = 0,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

impl Ordinal2D {
    #[inline]
    pub const fn try_as_cardinal(self) -> Option<Cardinal2D> {
        match self {
            Self::East  => Some(Cardinal2D::East ),
            Self::North => Some(Cardinal2D::North),
            Self::West  => Some(Cardinal2D::West ),
            Self::South => Some(Cardinal2D::South),
            _ => None,
        }
    }

    /// # Safety
    /// Must be `East`, `North`, `West`, or `South`
    #[inline]
    pub const unsafe fn as_cardinal_unchecked(self) -> Cardinal2D {
        debug_assert!(matches!(self, Self::East|Self::North|Self::West|Self::South));

        // SAFETY: Caller must uphold safety contract
        unsafe { std::mem::transmute::<Self, Cardinal2D>(self) }
    }

    /// Converts orientation to an angle in radians
    #[inline]
    pub const fn radians(self) -> f32 {
        self as u8 as f32 * std::f32::consts::FRAC_PI_8
    }

    /// Calculates the sine, cosine, and tangent of the orientation simultaneously
    #[inline]
    pub const fn cos_sin_tan(self) -> (f32, f32, f32) {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            Self::East      => (           1.0,            0.0,                0.0),
            Self::Northeast => ( FRAC_1_SQRT_2,  FRAC_1_SQRT_2,                1.0),
            Self::North     => (           0.0,            1.0,  f32::    INFINITY),
            Self::Northwest => (-FRAC_1_SQRT_2,  FRAC_1_SQRT_2, -              1.0),
            Self::West      => (-          1.0,            0.0, -              0.0),
            Self::Southwest => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2,                1.0),
            Self::South     => (           0.0, -          1.0,  f32::NEG_INFINITY),
            Self::Southeast => ( FRAC_1_SQRT_2, -FRAC_1_SQRT_2, -              1.0),
        }
    }

    /// Add rhs to self
    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        // NOTE: only works so easily because the ordinals are modulo 8
        let n = (self as u8 + rhs as u8) & 7;

        // SAFETY: `n` is masked to within enum discriminant range
        unsafe { std::mem::transmute::<u8, Self>(n) }
    }

    /// Subtract rhs from self
    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        // NOTE: only works so easily because the ordinals are modulo 8
        let n = (self as u8).wrapping_sub(rhs as u8) & 7;

        // SAFETY: `n` is masked to within enum discriminant range
        unsafe { std::mem::transmute::<u8, Self>(n) }
    }
}
