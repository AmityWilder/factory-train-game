use raylib::prelude::*;

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

    /// Check if `point` is in `self`
    #[must_use]
    fn contains(&self, point: &Self::Vector) -> bool;

    /// Check if `self` and `other` are colliding
    #[must_use]
    fn overlaps(&self, other: &Self) -> bool;
}

macro_rules! define_bbox {
    (
        $Type:ident: $Vector:ty
    ) => {
        impl $crate::math::bounds::SpacialBounds for $Type {
            type Vector = $Vector;

            #[inline]
            fn min(&self) -> Self::Vector {
                self.min
            }

            #[inline]
            fn max(&self) -> Self::Vector {
                self.max
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
    };
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

define_bbox!(BoundingBox: Vector3);
define_bbox!(FactoryBounds: FactoryVector3);
define_bbox!(LabBounds: LabVector3);

/// Object that takes up space that has a definitive minimum and maximum,
/// which can be used for identifying general proximity
#[const_trait]
pub trait Bounds<V> {
    type BoundingBox: SpacialBounds<Vector = V>;

    /// The bounding box of the object in the coordinate system of `V`
    fn bounds(&self) -> Self::BoundingBox;
}
