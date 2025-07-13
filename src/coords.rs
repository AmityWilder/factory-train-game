use std::ops::*;
use raylib::prelude::Vector3;
use fixed_point::Q32_32;

pub type PlayerCoord = Q32_32;

/// Uses fixed-point coordinates (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PlayerVector3 {
    pub x: PlayerCoord,
    pub y: PlayerCoord,
    pub z: PlayerCoord,
}

impl std::fmt::UpperHex for PlayerVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:X}, {:X}, {:X})", self.x, self.y, self.z)
    }
}

impl PlayerVector3 {
    /// Construct a position from integers
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Q32_32::from_i32(x),
            y: Q32_32::from_i32(y),
            z: Q32_32::from_i32(z),
        }
    }

    /// Convert from renderer vector
    #[inline]
    pub fn from_vec3(value: Vector3) -> Self {
        Self {
            x: Q32_32::from_f32(value.x),
            y: Q32_32::from_f32(value.y),
            z: Q32_32::from_f32(value.z),
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

    /// Add a vectors
    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            x: self.x.plus(rhs.x),
            y: self.y.plus(rhs.y),
            z: self.z.plus(rhs.z),
        }
    }

    /// Subtract a vector
    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        Self {
            x: self.x.minus(rhs.x),
            y: self.y.minus(rhs.y),
            z: self.z.minus(rhs.z),
        }
    }

    /// Multiply all components by a single value
    #[inline]
    pub const fn scale(self, rhs: PlayerCoord) -> Self {
        Self {
            x: self.x.multiply(rhs),
            y: self.y.multiply(rhs),
            z: self.z.multiply(rhs),
        }
    }

    /// Multiply vectors component-wise
    #[inline]
    pub const fn multiply(self, rhs: Self) -> Self {
        Self {
            x: self.x.multiply(rhs.x),
            y: self.y.multiply(rhs.y),
            z: self.z.multiply(rhs.z),
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

impl AddAssign for PlayerVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs)
    }
}

impl Sub for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl SubAssign for PlayerVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs)
    }
}

impl Mul<PlayerCoord> for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: PlayerCoord) -> Self::Output {
        self.scale(rhs)
    }
}

impl MulAssign<PlayerCoord> for PlayerVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: PlayerCoord) {
        *self = self.scale(rhs)
    }
}

impl Mul for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl MulAssign for PlayerVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs)
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

    pub const fn to_player_relative(self, player_pos: &PlayerVector3, origin: &RailVector3) -> Vector3 {
        (self.to_rail(*origin).to_player().minus(*player_pos)).to_vec3()
    }

    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    #[inline]
    pub const fn scale(self, rhs: i16) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    #[inline]
    pub const fn multiply(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for FactoryVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs)
    }
}

impl Sub for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl SubAssign for FactoryVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs)
    }
}

impl Mul<i16> for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i16) -> Self::Output {
        self.scale(rhs)
    }
}

impl MulAssign<i16> for FactoryVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i16) {
        *self = self.scale(rhs)
    }
}

impl Mul for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl MulAssign for FactoryVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs)
    }
}

/// Uses global integer coordinates
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
            x: Q32_32::from_i32(self.x),
            y: Q32_32::from_i32(self.y),
            z: Q32_32::from_i32(self.z),
        }
    }

    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    #[inline]
    pub const fn scale(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    #[inline]
    pub const fn multiply(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl From<RailVector3> for PlayerVector3 {
    #[inline]
    fn from(value: RailVector3) -> Self {
        value.to_player()
    }
}

impl Add for RailVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for RailVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs)
    }
}

impl Sub for RailVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl SubAssign for RailVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs)
    }
}

impl Mul<i32> for RailVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self.scale(rhs)
    }
}

impl MulAssign<i32> for RailVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.scale(rhs)
    }
}

impl Mul for RailVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl MulAssign for RailVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs)
    }
}
