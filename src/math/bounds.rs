use raylib::prelude::*;

use crate::math::coords::{PlayerCoord, PlayerVector3, lab::LabCoord};

use super::coords::{FactoryVector3, LabVector3};

#[const_trait]
pub trait SpacialBounds {
    /// The vector space type of this bounding box
    type Vector;

    /// The minimum coordinate in `self`
    #[must_use]
    fn min(&self) -> Self::Vector;

    /// The maximum coordinate in `self`
    #[must_use]
    fn max(&self) -> Self::Vector;

    /// The center position of `self`
    #[must_use]
    fn mid(&self) -> Self::Vector;

    /// The componentwise size of `self`
    #[must_use]
    fn size(&self) -> Self::Vector;

    /// Check if `point` is in `self`
    #[must_use]
    fn contains(&self, point: &Self::Vector) -> bool;

    /// Check if `self` and `other` are colliding
    #[must_use]
    fn overlaps(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FactoryBounds {
    pub min: FactoryVector3,
    pub max: FactoryVector3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LabBounds {
    pub min: LabVector3,
    pub max: LabVector3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PlayerBounds {
    pub min: PlayerVector3,
    pub max: PlayerVector3,
}

impl const SpacialBounds for BoundingBox {
    type Vector = Vector3;
    #[inline]
    fn min(&self) -> Self::Vector {
        self.min
    }
    #[inline]
    fn max(&self) -> Self::Vector {
        self.max
    }
    #[inline]
    fn mid(&self) -> Self::Vector {
        Vector3 {
            x: 0.5 * self.max.x + self.min.x,
            y: 0.5 * self.max.y + self.min.y,
            z: 0.5 * self.max.z + self.min.z,
        }
    }
    #[inline]
    fn size(&self) -> Self::Vector {
        Vector3 {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
            z: self.max.z - self.min.z,
        }
    }
    fn contains(&self, point: &Self::Vector) -> bool {
        ((self.min.x <= point.x) && (point.x <= self.max.x))
            && ((self.min.y <= point.y) && (point.y <= self.max.y))
            && ((self.min.z <= point.z) && (point.z <= self.max.z))
    }
    fn overlaps(&self, other: &Self) -> bool {
        ((self.max.x >= other.min.x) && (self.min.x <= other.max.x))
            && ((self.max.y >= other.min.y) && (self.min.y <= other.max.y))
            && ((self.max.z >= other.min.z) && (self.min.z <= other.max.z))
    }
}

impl const SpacialBounds for FactoryBounds {
    type Vector = FactoryVector3;
    #[inline]
    fn min(&self) -> Self::Vector {
        self.min
    }
    #[inline]
    fn max(&self) -> Self::Vector {
        self.max
    }
    #[inline]
    fn mid(&self) -> Self::Vector {
        FactoryVector3 {
            x: self.max.x / 2 + self.min.x,
            y: self.max.y / 2 + self.min.y,
            z: self.max.z / 2 + self.min.z,
        }
    }
    #[inline]
    fn size(&self) -> Self::Vector {
        FactoryVector3 {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
            z: self.max.z - self.min.z,
        }
    }
    fn contains(&self, point: &Self::Vector) -> bool {
        ((self.min.x <= point.x) && (point.x <= self.max.x))
            && ((self.min.y <= point.y) && (point.y <= self.max.y))
            && ((self.min.z <= point.z) && (point.z <= self.max.z))
    }
    fn overlaps(&self, other: &Self) -> bool {
        ((self.max.x >= other.min.x) && (self.min.x <= other.max.x))
            && ((self.max.y >= other.min.y) && (self.min.y <= other.max.y))
            && ((self.max.z >= other.min.z) && (self.min.z <= other.max.z))
    }
}

impl const SpacialBounds for LabBounds {
    type Vector = LabVector3;
    #[inline]
    fn min(&self) -> Self::Vector {
        self.min
    }
    #[inline]
    fn max(&self) -> Self::Vector {
        self.max
    }
    #[inline]
    fn mid(&self) -> Self::Vector {
        LabVector3 {
            x: self.max.x.multiply(LabCoord::from_f32(0.5)) + self.min.x,
            y: self.max.y.multiply(LabCoord::from_f32(0.5)) + self.min.y,
            z: self.max.z.multiply(LabCoord::from_f32(0.5)) + self.min.z,
        }
    }
    #[inline]
    fn size(&self) -> Self::Vector {
        LabVector3 {
            x: self.max.x.minus(self.min.x),
            y: self.max.y.minus(self.min.y),
            z: self.max.z.minus(self.min.z),
        }
    }
    fn contains(&self, point: &Self::Vector) -> bool {
        ((self.min.x.compare(point.x).is_le()) && (point.x.compare(self.max.x).is_le()))
            && ((self.min.y.compare(point.y).is_le()) && (point.y.compare(self.max.y).is_le()))
            && ((self.min.z.compare(point.z).is_le()) && (point.z.compare(self.max.z).is_le()))
    }
    fn overlaps(&self, other: &Self) -> bool {
        ((self.max.x.compare(other.min.x).is_ge()) && (self.min.x.compare(other.max.x).is_le()))
            && ((self.max.y.compare(other.min.y).is_ge())
                && (self.min.y.compare(other.max.y).is_le()))
            && ((self.max.z.compare(other.min.z).is_ge())
                && (self.min.z.compare(other.max.z).is_le()))
    }
}

impl const SpacialBounds for PlayerBounds {
    type Vector = PlayerVector3;
    #[inline]
    fn min(&self) -> Self::Vector {
        self.min
    }
    #[inline]
    fn max(&self) -> Self::Vector {
        self.max
    }
    #[inline]
    fn mid(&self) -> Self::Vector {
        self.max.minus(self.min)
    }
    #[inline]
    fn size(&self) -> Self::Vector {
        self.min.plus(self.max.scale(PlayerCoord::from_f32(0.5)))
    }
    fn contains(&self, point: &Self::Vector) -> bool {
        ((self.min.x.compare(point.x).is_le()) && (point.x.compare(self.max.x).is_le()))
            && ((self.min.y.compare(point.y).is_le()) && (point.y.compare(self.max.y).is_le()))
            && ((self.min.z.compare(point.z).is_le()) && (point.z.compare(self.max.z).is_le()))
    }
    fn overlaps(&self, other: &Self) -> bool {
        ((self.max.x.compare(other.min.x).is_ge()) && (self.min.x.compare(other.max.x).is_le()))
            && ((self.max.y.compare(other.min.y).is_ge())
                && (self.min.y.compare(other.max.y).is_le()))
            && ((self.max.z.compare(other.min.z).is_ge())
                && (self.min.z.compare(other.max.z).is_le()))
    }
}

/// Object that takes up space that has a definitive minimum and maximum,
/// which can be used for identifying general proximity
#[const_trait]
pub trait Bounds<V> {
    type BoundingBox: SpacialBounds<Vector = V>;

    /// The bounding box of the object in the coordinate system of `V`
    fn bounds(&self) -> Self::BoundingBox;
}
