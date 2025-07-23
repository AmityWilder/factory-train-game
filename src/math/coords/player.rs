use fixed_point::Q32_32;
use raylib::prelude::Vector3;

use super::{FactoryVector3, TryFromFactoryVectorError, VectorConstants, rail::RailVector3};

pub type PlayerCoord = Q32_32;

/// Uses fixed-point coordinates (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

impl const VectorConstants for PlayerVector3 {
    const ZERO: Self = Self::from_i32(0, 0, 0);
    const ONE: Self = Self::from_i32(1, 1, 1);
    const NEG_ONE: Self = Self::from_i32(-1, -1, -1);
    const X: Self = Self::from_i32(1, 0, 0);
    const Y: Self = Self::from_i32(0, 1, 0);
    const Z: Self = Self::from_i32(0, 0, 1);
    const NEG_X: Self = Self::from_i32(-1, 0, 0);
    const NEG_Y: Self = Self::from_i32(0, -1, 0);
    const NEG_Z: Self = Self::from_i32(0, 0, -1);
    const MIN: Self = Self::new(PlayerCoord::MIN, PlayerCoord::MIN, PlayerCoord::MIN);
    const MAX: Self = Self::new(PlayerCoord::MAX, PlayerCoord::MAX, PlayerCoord::MAX);
}

impl PlayerVector3 {
    pub const fn new(x: PlayerCoord, y: PlayerCoord, z: PlayerCoord) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn from_i32(x: i32, y: i32, z: i32) -> Self {
        Self::new(
            PlayerCoord::from_i32(x),
            PlayerCoord::from_i32(y),
            PlayerCoord::from_i32(z),
        )
    }

    #[inline]
    pub const fn from_f32(x: f32, y: f32, z: f32) -> Self {
        Self::new(
            PlayerCoord::from_f32(x),
            PlayerCoord::from_f32(y),
            PlayerCoord::from_f32(z),
        )
    }

    /// Convert from renderer vector
    #[inline]
    pub const fn from_vec3(value: Vector3) -> Self {
        let Vector3 { x, y, z } = value;
        Self::from_f32(x, y, z)
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

    /// Convert to renderer vector
    ///
    /// Note: Truncates the submeter position
    #[inline]
    pub const fn to_rail(self) -> RailVector3 {
        RailVector3 {
            x: self.x.to_i32(),
            y: self.y.to_i32(),
            z: self.z.to_i32(),
        }
    }

    /// Convert to renderer vector
    #[inline]
    pub const fn to_factory(
        self,
        origin: &RailVector3,
    ) -> Result<FactoryVector3, TryFromFactoryVectorError> {
        self.to_rail().to_factory(origin)
    }

    /// Componentwise absolute value
    #[inline]
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Negate a vector
    #[inline]
    pub const fn negate(self) -> Self {
        Self {
            x: self.x.negate(),
            y: self.y.negate(),
            z: self.z.negate(),
        }
    }

    /// Add a vector
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

    /// The sum of the components
    #[inline]
    pub const fn sum(self) -> PlayerCoord {
        self.x.plus(self.y).plus(self.z)
    }

    /// The product of the components
    #[inline]
    pub const fn product(self) -> PlayerCoord {
        self.x.multiply(self.y).multiply(self.z)
    }

    /// Calculate the dot product between two vectors
    #[inline]
    pub const fn dot(self, rhs: Self) -> PlayerCoord {
        self.multiply(rhs).sum()
    }

    /// Calculate the taxicab magnitude of a vector, which is cheaper
    /// than the Euclidian length but does not represent a single straight line
    /// and depends on the rotation of the grid
    #[inline]
    pub const fn length_taxi(self) -> PlayerCoord {
        self.abs().sum()
    }

    /// Calculate the square of the Euclidian magnitude of a vector,
    /// which is cheaper than the length due to not needing to sqrt
    /// but is only useful for comparisons and not amounts
    #[inline]
    pub const fn length_sqr(self) -> PlayerCoord {
        self.dot(self)
    }

    /// Calculate the Euclidian magnitude of a vector
    ///
    /// See also [`Self::length_squared`]
    #[inline]
    pub const fn length(self) -> PlayerCoord {
        self.length_sqr().sqrt()
    }

    /// Calculate the taxicab distance between two vectors, which is cheaper
    /// than the Euclidian distance but does not represent a single straight line
    /// and depends on the rotation of the grid
    #[inline]
    pub const fn distance_taxi(self, other: Self) -> PlayerCoord {
        self.minus(other).length_taxi()
    }

    /// Calculate the square of the Euclidian distance between two vectors,
    /// which is cheaper than the distance due to not needing to sqrt
    #[inline]
    pub const fn distance_sqr(self, other: Self) -> PlayerCoord {
        self.minus(other).length_sqr()
    }

    /// Calculate the Euclidian distance between two vectors
    ///
    /// See also [`Self::distance_squared`]
    #[inline]
    pub const fn distance(self, other: Self) -> PlayerCoord {
        self.minus(other).length()
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

impl std::ops::Neg for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl const std::ops::Add for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::AddAssign for PlayerVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
    }
}

impl std::ops::Sub for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl std::ops::SubAssign for PlayerVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs);
    }
}

impl std::ops::Mul<PlayerCoord> for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: PlayerCoord) -> Self::Output {
        self.scale(rhs)
    }
}

impl std::ops::MulAssign<PlayerCoord> for PlayerVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: PlayerCoord) {
        *self = self.scale(rhs);
    }
}

impl std::ops::Mul for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl std::ops::MulAssign for PlayerVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs);
    }
}
