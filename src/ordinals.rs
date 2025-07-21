#![rustfmt::skip]
use raylib::prelude::*;

/// A 2D cardinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

    #[allow(non_snake_case)]
    #[inline]
    pub const fn as_3D(self) -> Cardinal3D {
        match self {
            Self::East  => Cardinal3D::East,
            Self::North => Cardinal3D::North,
            Self::West  => Cardinal3D::West,
            Self::South => Cardinal3D::South,
        }
    }

    /// Calculates the sine, cosine, and tangent of the orientation simultaneously
    #[inline]
    pub const fn cos_sin_tan(self) -> (f32, f32, f32) {
        match self {
            Self::East  => ( 1.0,  0.0,                0.0),
            Self::North => ( 0.0,  1.0,  f32::    INFINITY),
            Self::West  => (-1.0,  0.0, -              0.0),
            Self::South => ( 0.0, -1.0,  f32::NEG_INFINITY),
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
            Self::East  => Some(Cardinal2D::East),
            Self::North => Some(Cardinal2D::North),
            Self::West  => Some(Cardinal2D::West),
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

    #[allow(non_snake_case)]
    #[inline]
    pub const fn as_3D(self) -> Ordinal3D {
        match self {
            Self::East      => Ordinal3D::East,
            Self::Northeast => Ordinal3D::Northeast,
            Self::North     => Ordinal3D::North,
            Self::Northwest => Ordinal3D::Northwest,
            Self::West      => Ordinal3D::West,
            Self::Southwest => Ordinal3D::Southwest,
            Self::South     => Ordinal3D::South,
            Self::Southeast => Ordinal3D::Southeast,
        }
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

    /// The direction of the ordinal
    #[inline]
    pub const fn direction(self) -> Vector2 {
        let (x, y, _) = self.cos_sin_tan();
        Vector2::new(x, y)
    }

    /// The direction of the ordinal
    #[inline]
    pub const fn direction3(self) -> Vector3 {
        let (x, z, _) = self.cos_sin_tan();
        Vector3::new(x, 0.0, z)
    }

    #[inline]
    pub const fn matrix(self) -> Matrix {
        let (cos, sin, _) = self.cos_sin_tan();
        Matrix {
            m0:  cos, m4: 0.0,  m8: sin, m12: 0.0,
            m1:  0.0, m5: 1.0,  m9: 0.0, m13: 0.0,
            m2: -sin, m6: 0.0, m10: cos, m14: 0.0,
            m3:  0.0, m7: 0.0, m11: 0.0, m15: 1.0,
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

/// A 3D cardinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Cardinal3D {
    Down,
    #[default]
    East,
    North,
    West,
    South,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Ordinal3D {
    Down,
    EastDown,
    NorthDown,
    WestDown,
    SouthDown,
    #[default]
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
    EastUp,
    NorthUp,
    WestUp,
    SouthUp,
    Up,
}

impl Ordinal3D {
    /// The direction of the ordinal
    #[inline]
    pub const fn direction(self) -> Vector3 {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            Self::Down      => Vector3::new(           0.0, -          1.0,            0.0),
            Self::EastDown  => Vector3::new( FRAC_1_SQRT_2, -FRAC_1_SQRT_2,            0.0),
            Self::NorthDown => Vector3::new(           0.0, -FRAC_1_SQRT_2,  FRAC_1_SQRT_2),
            Self::WestDown  => Vector3::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2,            0.0),
            Self::SouthDown => Vector3::new(           0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Self::East      => Vector3::new(           1.0,            0.0,            0.0),
            Self::Northeast => Vector3::new( FRAC_1_SQRT_2,            0.0,  FRAC_1_SQRT_2),
            Self::North     => Vector3::new(           0.0,            0.0,            1.0),
            Self::Northwest => Vector3::new(-FRAC_1_SQRT_2,            0.0,  FRAC_1_SQRT_2),
            Self::West      => Vector3::new(-          1.0,            0.0,            0.0),
            Self::Southwest => Vector3::new(-FRAC_1_SQRT_2,            0.0, -FRAC_1_SQRT_2),
            Self::South     => Vector3::new(           0.0,            0.0, -          1.0),
            Self::Southeast => Vector3::new( FRAC_1_SQRT_2,            0.0, -FRAC_1_SQRT_2),
            Self::EastUp    => Vector3::new( FRAC_1_SQRT_2,  FRAC_1_SQRT_2,            0.0),
            Self::NorthUp   => Vector3::new(           0.0,  FRAC_1_SQRT_2,  FRAC_1_SQRT_2),
            Self::WestUp    => Vector3::new(-FRAC_1_SQRT_2,  FRAC_1_SQRT_2,            0.0),
            Self::SouthUp   => Vector3::new(           0.0,  FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Self::Up        => Vector3::new(           0.0,            1.0,            0.0),
        }
    }
}
