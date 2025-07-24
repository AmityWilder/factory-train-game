use raylib::prelude::*;

use crate::{
    chem::Element,
    math::{
        bounds::{Bounds, LabBounds},
        coords::{LabVector3, PlayerVector3},
    },
    player::Player,
    resource::Resources,
};

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
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        resources: &Resources,
        player_pos: &PlayerVector3,
        origin: &PlayerVector3,
    ) {
        let mesh = &resources.periodic_table_mesh;
        let Vector3 { x, y, z } = self.position.to_player_relative(player_pos, origin);
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
                mesh,
                material,
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
            min: bbox.min.to_player_relative(player_pos, origin),
            max: bbox.max.to_player_relative(player_pos, origin),
        };
        d.draw_bounding_box(bbox, Color::BLUEVIOLET);
    }
}
