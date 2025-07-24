use crate::{
    math::{bounds::SpacialBounds, coords::PlayerVector3},
    player::Player,
    region::{factory::grid_vis::GridVisualizer, rail::World},
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
    ) -> Region<'a> {
        match self {
            Self::Rail => Region::Rail(world),
            Self::Factory(idx) => Region::Factory(&factories[idx]),
            Self::Lab => Region::Lab(lab),
        }
    }

    pub const fn to_mut_region<'a>(
        self,
        factories: &'a mut [Factory],
        lab: &'a mut Laboratory,
        world: &'a mut World,
    ) -> RegionMut<'a> {
        match self {
            Self::Rail => RegionMut::Rail(world),
            Self::Factory(idx) => RegionMut::Factory(&mut factories[idx]),
            Self::Lab => RegionMut::Lab(lab),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Region<'a> {
    Rail(&'a World),
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
        grid: Option<&GridVisualizer>,
    ) {
        match self {
            Self::Rail(world) => world.draw(d, thread, resources, player),
            Self::Factory(factory) => factory.draw(
                d,
                thread,
                resources,
                player,
                grid.expect("entering factory region should create a grid"),
            ),
            Self::Lab(lab) => lab.draw(d, thread, resources, player),
        }
    }
}

#[derive(Debug)]
pub enum RegionMut<'a> {
    Rail(&'a mut World),
    Factory(&'a mut Factory),
    Lab(&'a mut Laboratory),
}

impl RegionMut<'_> {
    pub const fn as_region(&self) -> Region<'_> {
        match self {
            RegionMut::Rail(world) => Region::Rail(world),
            RegionMut::Factory(factory) => Region::Factory(factory),
            RegionMut::Lab(lab) => Region::Lab(lab),
        }
    }
}
