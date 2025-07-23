use super::{PlayerCoord, PlayerVector3};
use fixed_point::Q16_16;
use raylib::prelude::*;

pub type LabCoord = Q16_16;

/// Uses floating-point coordinates relative to lab origin (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LabVector3 {
    pub x: LabCoord,
    pub y: LabCoord,
    pub z: LabCoord,
}

impl LabVector3 {
    #[inline]
    pub const fn new(x: LabCoord, y: LabCoord, z: LabCoord) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn from_i16(x: i16, y: i16, z: i16) -> Self {
        Self::new(
            LabCoord::from_i16(x),
            LabCoord::from_i16(y),
            LabCoord::from_i16(z),
        )
    }

    #[inline]
    pub const fn from_f32(x: f32, y: f32, z: f32) -> Self {
        Self::new(
            LabCoord::from_f32(x),
            LabCoord::from_f32(y),
            LabCoord::from_f32(z),
        )
    }

    #[inline]
    pub fn to_player(self, origin: PlayerVector3) -> PlayerVector3 {
        PlayerVector3 {
            x: PlayerCoord::from(self.x),
            y: PlayerCoord::from(self.y),
            z: PlayerCoord::from(self.z),
        } + origin
    }

    #[inline]
    pub fn to_player_relative(self, player_pos: PlayerVector3, origin: PlayerVector3) -> Vector3 {
        (self.to_player(origin).minus(player_pos)).to_vec3()
    }
}
