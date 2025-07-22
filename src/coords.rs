use fixed_point::Q32_32;
use raylib::prelude::Vector3;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const FORWARD: Vector3 = Vector3::NEG_Z;
pub const BACKWARD: Vector3 = Vector3::Z;
pub const RIGHT: Vector3 = Vector3::X;
pub const LEFT: Vector3 = Vector3::NEG_X;
pub const UP: Vector3 = Vector3::Y;
pub const DOWN: Vector3 = Vector3::NEG_Y;

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

impl PlayerVector3 {
    pub const ZERO: Self = Self::from_i32(0, 0, 0);
    pub const ONE: Self = Self::from_i32(1, 1, 1);
    pub const NEG_ONE: Self = Self::from_i32(-1, -1, -1);
    pub const X: Self = Self::from_i32(1, 0, 0);
    pub const Y: Self = Self::from_i32(0, 1, 0);
    pub const Z: Self = Self::from_i32(0, 0, 1);
    pub const NEG_X: Self = Self::from_i32(-1, 0, 0);
    pub const NEG_Y: Self = Self::from_i32(0, -1, 0);
    pub const NEG_Z: Self = Self::from_i32(0, 0, -1);
    pub const MIN: Self = Self::new(PlayerCoord::MIN, PlayerCoord::MIN, PlayerCoord::MIN);
    pub const MAX: Self = Self::new(PlayerCoord::MAX, PlayerCoord::MAX, PlayerCoord::MAX);

    pub const FORWARD: Self = Self::NEG_Z;
    pub const BACKWARD: Self = Self::Z;
    pub const RIGHT: Self = Self::X;
    pub const LEFT: Self = Self::NEG_X;
    pub const UP: Self = Self::Y;
    pub const DOWN: Self = Self::NEG_Y;

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
        self.x.add(self.y).add(self.z)
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

impl Neg for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl const Add for PlayerVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for PlayerVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
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
        *self = self.minus(rhs);
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
        *self = self.scale(rhs);
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
        *self = self.multiply(rhs);
    }
}

/// Uses integer coordinates relative to factory origin (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FactoryVector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl FactoryVector3 {
    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const ONE: Self = Self::new(1, 1, 1);
    pub const NEG_ONE: Self = Self::new(-1, -1, -1);
    pub const X: Self = Self::new(1, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0);
    pub const Z: Self = Self::new(0, 0, 1);
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    pub const NEG_Y: Self = Self::new(0, -1, 0);
    pub const NEG_Z: Self = Self::new(0, 0, -1);
    pub const MIN: Self = Self::new(i16::MIN, i16::MIN, i16::MIN);
    pub const MAX: Self = Self::new(i16::MAX, i16::MAX, i16::MAX);

    pub const FORWARD: Self = Self::NEG_Z;
    pub const BACKWARD: Self = Self::Z;
    pub const RIGHT: Self = Self::X;
    pub const LEFT: Self = Self::NEG_X;
    pub const UP: Self = Self::Y;
    pub const DOWN: Self = Self::NEG_Y;

    #[inline]
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn to_rail(self, origin: RailVector3) -> RailVector3 {
        RailVector3 {
            x: origin.x + self.x as i32,
            y: origin.y + self.y as i32,
            z: origin.z + self.z as i32,
        }
    }

    #[inline]
    pub const fn to_player(self, player_pos: &PlayerVector3, origin: &RailVector3) -> Vector3 {
        (self.to_rail(*origin).to_player().minus(*player_pos)).to_vec3()
    }

    #[inline]
    pub const fn to_player_relative(
        self,
        player_pos: &PlayerVector3,
        origin: &RailVector3,
    ) -> Vector3 {
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

impl const Add for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for FactoryVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
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
        *self = self.minus(rhs);
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
        *self = self.scale(rhs);
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
        *self = self.multiply(rhs);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromFactoryVectorError(());

impl std::fmt::Display for TryFromFactoryVectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "i32 Rail coordinates were out of bounds of i16 Factory coordinates".fmt(f)
    }
}

impl std::error::Error for TryFromFactoryVectorError {}

/// Uses global integer coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RailVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl RailVector3 {
    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const ONE: Self = Self::new(1, 1, 1);
    pub const NEG_ONE: Self = Self::new(-1, -1, -1);
    pub const X: Self = Self::new(1, 0, 0);
    pub const Y: Self = Self::new(0, 1, 0);
    pub const Z: Self = Self::new(0, 0, 1);
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    pub const NEG_Y: Self = Self::new(0, -1, 0);
    pub const NEG_Z: Self = Self::new(0, 0, -1);
    pub const MIN: Self = Self::new(i32::MIN, i32::MIN, i32::MIN);
    pub const MAX: Self = Self::new(i32::MAX, i32::MAX, i32::MAX);

    pub const FORWARD: Self = Self::NEG_Z;
    pub const BACKWARD: Self = Self::Z;
    pub const RIGHT: Self = Self::X;
    pub const LEFT: Self = Self::NEG_X;
    pub const UP: Self = Self::Y;
    pub const DOWN: Self = Self::NEG_Y;

    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn to_factory(
        self,
        origin: &Self,
    ) -> Result<FactoryVector3, TryFromFactoryVectorError> {
        const I16_MIN: i32 = i16::MIN as i32;
        const I16_MAX: i32 = i16::MAX as i32;
        match self.minus(*origin) {
            Self {
                x: x @ I16_MIN..=I16_MAX,
                y: y @ I16_MIN..=I16_MAX,
                z: z @ I16_MIN..=I16_MAX,
            } =>
            {
                #[allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    reason = "just checked and components do not truncate or wrap"
                )]
                Ok(FactoryVector3 {
                    x: x as i16,
                    y: y as i16,
                    z: z as i16,
                })
            }
            _ => Err(TryFromFactoryVectorError(())),
        }
    }

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

impl const Add for RailVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl AddAssign for RailVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
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
        *self = self.minus(rhs);
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
        *self = self.scale(rhs);
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
        *self = self.multiply(rhs);
    }
}
