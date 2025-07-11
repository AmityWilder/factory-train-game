use std::ops::{Add, AddAssign, Sub, SubAssign};
use raylib::prelude::Vector3;
use fixed_point::Q48_16;

/// Uses fixed-point coordinates (in meters)
///
/// 48 bits for meter position, 16 bits for sub-meter position
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PlayerVector3 {
    x: Q48_16,
    y: Q48_16,
    z: Q48_16,
}

impl PlayerVector3 {
    const DECIMAL_MASK: i64 = 0xFFFF;
    const DECIMAL_FACTOR: f32 = Self::DECIMAL_MASK as f32;
    const DECIMAL_INV_FACTOR: f32 = Self::DECIMAL_FACTOR.recip();
    const DECIMAL_BITS: u32 = Self::DECIMAL_MASK.count_ones();

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Q48_16::from_i32(x),
            y: Q48_16::from_i32(y),
            z: Q48_16::from_i32(z),
        }
    }

    /// Convert from renderer vector
    #[inline]
    pub fn from_vec3(value: Vector3) -> Self {
        Self {
            x: Q48_16::from_f32(value.x),
            y: Q48_16::from_f32(value.y),
            z: Q48_16::from_f32(value.z),
        }
    }

    /// Convert to renderer vector
    #[inline]
    pub const fn to_vec3(self) -> Vector3 {
        Vector3 {
            x: self.x.to_f32(),
            y: self.y.to_f32(),
            z: self.z.to_f32(),
        }
    }

    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            x: self.x.plus(rhs.x),
            y: self.y.plus(rhs.y),
            z: self.z.plus(rhs.z),
        }
    }

    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        Self {
            x: self.x.minus(rhs.x),
            y: self.y.minus(rhs.y),
            z: self.z.minus(rhs.z),
        }
    }
}

impl From<Vector3> for PlayerVector3 {
    #[inline]
    fn from(value: Vector3) -> Self {
        PlayerVector3::from_vec3(value)
    }
}

impl From<PlayerVector3> for Vector3 {
    #[inline]
    fn from(value: PlayerVector3) -> Self {
        value.to_vec3()
    }
}

impl Add for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl Sub for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl AddAssign for PlayerVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs)
    }
}

impl SubAssign for PlayerVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs)
    }
}

/// Uses integer coordinates relative to factory origin (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FactoryVector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl FactoryVector3 {
    #[inline]
    pub const fn to_rail(self, origin: RailVector3) -> RailVector3 {
        RailVector3 {
            x: origin.x + self.x as i32,
            y: origin.y + self.y as i32,
            z: origin.z + self.z as i32,
        }
    }
}

/// Uses global integer coordinates (in units of [`METERS_PER_RAIL`])
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RailVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl RailVector3 {
    #[inline]
    pub const fn to_player(self) -> PlayerVector3 {
        PlayerVector3 {
            x: Q48_16::from_i32(self.x),
            y: Q48_16::from_i32(self.y),
            z: Q48_16::from_i32(self.z),
        }
    }
}

impl From<RailVector3> for PlayerVector3 {
    #[inline]
    fn from(value: RailVector3) -> Self {
        value.to_player()
    }
}
