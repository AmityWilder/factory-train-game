use raylib::prelude::*;

use crate::{
    chem::Element,
    math::{
        bounds::{Bounds, LabBounds, SpacialBounds},
        coords::{LabVector3, PlayerCoord, PlayerVector3},
    },
    player::Player,
    resource::Resources,
    rl_helpers::DynRaylibDraw3D,
};

use super::{PlayerOverlap, Region};

// Lab is not grid aligned, and small enough that I don't care about floating point error

pub trait LabEquipment: Bounds<Vector3, BoundingBox = BoundingBox> + std::fmt::Debug {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodTableVariable {
    NoVariable,
    Protons,
    Mass,
    ElectroNegativity,
}

#[derive(Debug)]
pub struct PeriodicTable {
    pub position: LabVector3,
    pub variable: PeriodTableVariable,
}

impl PeriodicTable {
    pub fn draw(
        &self,
        d: &mut dyn DynRaylibDraw3D,
        _thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
        origin: &PlayerVector3,
    ) {
        let mesh = &resources.periodic_table_mesh;
        let Vector3 { x, y, z } = self.position.to_player_relative(&player.position, origin);
        let translation = Matrix::translate(x, y, z);
        for (element, (matrix, material)) in Element::list()
            .iter()
            .zip(resources.periodic_table_mats.iter())
        {
            let protons = element.protons().get();
            let y_scale = match self.variable {
                PeriodTableVariable::NoVariable => 1.0,
                PeriodTableVariable::Protons => f32::from(protons) / 50.0,
                PeriodTableVariable::Mass => todo!(),
                PeriodTableVariable::ElectroNegativity => todo!(),
            };
            // SAFETY: TBD
            let material = unsafe { WeakMaterial::from_raw(**material) };
            d.draw_mesh(
                **mesh,
                *material,
                Matrix::scale(1.0, y_scale, 1.0)
                    * Matrix::translate(0.0, y_scale * 0.125, 0.0)
                    * translation
                    * *matrix,
            );
        }
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
    pub periodic_tables: Vec<PeriodicTable>,
}

impl PlayerOverlap for Laboratory {
    fn is_overlapping(&self, player: &Player) -> bool {
        self.bounds.contains(&player.eye_pos().to_lab(&self.origin))
    }

    fn local_floor(&self, _player: &Player) -> Option<PlayerCoord> {
        None // TODO
    }
}

impl Region for Laboratory {
    fn draw(
        &self,
        d: &mut dyn DynRaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
    ) {
        for periodic_table in &self.periodic_tables {
            periodic_table.draw(d, thread, resources, player, &self.origin);
        }

        let bbox = self.bounds;
        let bbox = BoundingBox {
            min: bbox.min.to_player_relative(&player.position, &self.origin),
            max: bbox.max.to_player_relative(&player.position, &self.origin),
        };
        d.draw_bounding_box(bbox, Color::BLUEVIOLET);
    }
}
