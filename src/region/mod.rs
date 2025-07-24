use crate::{
    math::{
        bounds::SpacialBounds,
        coords::{PlayerCoord, PlayerVector3},
    },
    player::Player,
    region::rail::World,
    resource::Resources,
    rl_helpers::DynRaylibDraw3D,
};
use factory::Factory;
use lab::Laboratory;
use raylib::prelude::*;

pub mod factory;
pub mod lab;
pub mod rail;

pub trait PlayerOverlap {
    #[must_use]
    fn is_overlapping(&self, player: &Player) -> bool;

    #[must_use]
    fn local_floor(&self, player: &Player) -> Option<PlayerCoord>;
}

pub trait Region: PlayerOverlap {
    fn draw(
        &self,
        d: &mut DynRaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RegionId {
    #[default]
    Rail,
    Factory(usize),
    Lab,
}

// Priority: Lab > Factory > Outside

impl RegionId {
    /// Get the ID of the region containing `pos`
    pub fn containing(
        pos: &PlayerVector3,
        factories: &[Factory],
        lab: &Laboratory,
        _world: &World,
    ) -> Self {
        lab.bounds
            .contains(&pos.to_lab(&lab.origin))
            .then_some(RegionId::Lab)
            .or_else(|| {
                factories
                    .iter()
                    .position(|factory| {
                        pos.to_factory(&factory.origin)
                            .is_ok_and(|pos| factory.bounds.contains(&pos))
                    })
                    .map(RegionId::Factory)
            })
            .unwrap_or_default()
    }

    /// Get the ID of the region containing `pos`, returning `true` if the region has changed
    pub fn update(
        &mut self,
        pos: &PlayerVector3,
        factories: &[Factory],
        lab: &Laboratory,
        world: &World,
    ) -> bool {
        let new_value = Self::containing(pos, factories, lab, world);
        let is_changed = self != &new_value;
        *self = new_value;
        is_changed
    }

    pub const fn to_region<'a>(
        self,
        factories: &'a [Factory],
        lab: &'a Laboratory,
        world: &'a World,
    ) -> &'a (dyn Region + 'a) {
        match self {
            Self::Rail => world,
            Self::Factory(idx) => &factories[idx],
            Self::Lab => lab,
        }
    }

    pub const fn to_mut_region<'a>(
        self,
        factories: &'a mut [Factory],
        lab: &'a mut Laboratory,
        world: &'a mut World,
    ) -> &'a mut (dyn Region + 'a) {
        match self {
            Self::Rail => world,
            Self::Factory(idx) => &mut factories[idx],
            Self::Lab => lab,
        }
    }
}
