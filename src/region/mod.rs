use crate::{
    math::{bounds::SpacialBounds, coords::PlayerVector3},
    player::Player,
    region::rail::World,
    resource::Resources,
};
use factory::Factory;
use lab::Laboratory;
use raylib::prelude::*;

pub mod factory;
pub mod lab;
pub mod rail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RegionId {
    #[default]
    Rail,
    Factory(usize),
    Lab,
}

impl RegionId {
    pub fn containing(pos: &PlayerVector3, factories: &[Factory], lab: &Laboratory) -> Self {
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

    pub const fn to_region<'a>(self, factories: &'a [Factory], lab: &'a Laboratory) -> Region<'a> {
        match self {
            Self::Rail => Region::Rail,
            Self::Factory(idx) => Region::Factory(&factories[idx]),
            Self::Lab => Region::Lab(lab),
        }
    }

    pub const fn to_mut_region<'a>(
        self,
        factories: &'a mut [Factory],
        lab: &'a mut Laboratory,
    ) -> RegionMut<'a> {
        match self {
            Self::Rail => RegionMut::Rail,
            Self::Factory(idx) => RegionMut::Factory(&mut factories[idx]),
            Self::Lab => RegionMut::Lab(lab),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Region<'a> {
    #[default]
    Rail,
    Factory(&'a Factory),
    Lab(&'a Laboratory),
}

impl Region<'_> {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
    ) {
        match self {
            Self::Rail => World.draw(d, thread, resources, player), // TODO
            Self::Factory(factory) => factory.draw(d, thread, resources, player),
            Self::Lab(lab) => lab.draw(d, thread, resources, player),
        }
    }
}

#[derive(Debug, Default)]
pub enum RegionMut<'a> {
    #[default]
    Rail,
    Factory(&'a mut Factory),
    Lab(&'a mut Laboratory),
}

impl RegionMut<'_> {
    pub const fn as_region(&self) -> Region<'_> {
        match self {
            RegionMut::Rail => Region::Rail,
            RegionMut::Factory(factory) => Region::Factory(factory),
            RegionMut::Lab(lab) => Region::Lab(lab),
        }
    }
}
