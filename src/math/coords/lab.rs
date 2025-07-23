use super::PlayerVector3;
use raylib::prelude::*;

/// Uses floating-point coordinates relative to lab origin (in meters)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LabVector3(Vector3);

impl LabVector3 {
    #[inline]
    pub const fn to_player(self, origin: PlayerVector3) -> PlayerVector3 {
        PlayerVector3::from_vec3(self.0) + origin
    }

    #[inline]
    pub const fn to_player_relative(
        self,
        player_pos: PlayerVector3,
        origin: PlayerVector3,
    ) -> Vector3 {
        (self.to_player(origin).minus(player_pos)).to_vec3()
    }
}
