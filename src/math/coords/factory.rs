use super::{PlayerVector3, RailVector3};
use raylib::prelude::Vector3;

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
    pub const fn to_player(self, origin: &RailVector3) -> PlayerVector3 {
        self.to_rail(*origin).to_player()
    }

    #[inline]
    pub const fn to_player_relative(
        self,
        player_pos: &PlayerVector3,
        origin: &RailVector3,
    ) -> Vector3 {
        (self.to_player(origin).minus(*player_pos)).to_vec3()
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

impl const std::ops::Add for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::AddAssign for FactoryVector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(rhs);
    }
}

impl std::ops::Sub for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

impl std::ops::SubAssign for FactoryVector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(rhs);
    }
}

impl std::ops::Mul<i16> for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i16) -> Self::Output {
        self.scale(rhs)
    }
}

impl std::ops::MulAssign<i16> for FactoryVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i16) {
        *self = self.scale(rhs);
    }
}

impl std::ops::Mul for FactoryVector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl std::ops::MulAssign for FactoryVector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(rhs);
    }
}
