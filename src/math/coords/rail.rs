use super::{FactoryVector3, PlayerCoord, PlayerVector3, TryFromFactoryVectorError};
use raylib::prelude::*;

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
            x: PlayerCoord::from_i32(self.x),
            y: PlayerCoord::from_i32(self.y),
            z: PlayerCoord::from_i32(self.z),
        }
    }

    #[inline]
    pub const fn to_player_relative(self, player_pos: PlayerVector3) -> Vector3 {
        self.to_player().minus(player_pos).to_vec3()
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

impl const std::ops::Add for RailVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::AddAssign for RailVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
    }
}

impl std::ops::Sub for RailVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl std::ops::SubAssign for RailVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs);
    }
}

impl std::ops::Mul<i32> for RailVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        self.scale(rhs)
    }
}

impl std::ops::MulAssign<i32> for RailVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.scale(rhs);
    }
}

impl std::ops::Mul for RailVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl std::ops::MulAssign for RailVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs);
    }
}
