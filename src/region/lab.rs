use raylib::prelude::*;

use crate::{
    math::{
        bounds::{Bounds, LabBounds},
        coords::{LabVector3, PlayerVector3},
    },
    player::Player,
    resource::Resources,
};

// Lab is not grid aligned, and small enough that I don't care about floating point error

pub trait LabEquipment: Bounds<Vector3, BoundingBox = BoundingBox> + std::fmt::Debug {}

#[derive(Debug)]
pub struct PeriodicTable {
    pub position: LabVector3,
}

impl PeriodicTable {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        _resources: &Resources,
        player_pos: &PlayerVector3,
        origin: &PlayerVector3,
    ) {
        d.draw_cube_v(
            self.position.to_player_relative(*player_pos, *origin) + Vector3::new(1.0, 1.0, 2.0),
            Vector3::new(1.0, 1.0, 2.0),
            Color::BLUE,
        );
    }
}

impl Bounds<Vector3> for PeriodicTable {
    type BoundingBox = BoundingBox;

    fn bounds(&self) -> Self::BoundingBox {
        todo!()
    }
}

impl LabEquipment for PeriodicTable {}

#[derive(Debug)]
pub struct Laboratory {
    pub origin: PlayerVector3,
    pub bounds: LabBounds,
    pub periodic_table: Option<PeriodicTable>,
}

impl Laboratory {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
    ) {
        let origin = &self.origin;
        let player_pos = &player.position;

        if let Some(periodic_table) = &self.periodic_table {
            periodic_table.draw(d, thread, resources, player_pos, origin);
        }

        let bbox = self.bounds;
        let bbox = BoundingBox {
            min: bbox.min.to_player_relative(*player_pos, *origin),
            max: bbox.max.to_player_relative(*player_pos, *origin),
        };
        d.draw_bounding_box(bbox, Color::BLUEVIOLET);
    }
}
