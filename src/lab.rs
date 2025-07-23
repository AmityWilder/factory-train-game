use raylib::prelude::*;

use crate::math::bounds::SpacialBounds;

// Lab is not grid aligned, and small enough that I don't care about floating point error

pub trait LabEquipment: SpacialBounds<Vector3, BoundingBox = BoundingBox> {}

pub struct PeriodicTable {
    position: Vector3,
}

impl SpacialBounds<Vector3> for PeriodicTable {
    type BoundingBox = BoundingBox;

    fn bounds(&self) -> Self::BoundingBox {
        todo!()
    }
}

impl LabEquipment for PeriodicTable {}

pub struct Laboratory {
    equipment: Vec<Box<dyn LabEquipment>>,
}
