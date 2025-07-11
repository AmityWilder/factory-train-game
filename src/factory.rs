use std::num::NonZeroU8;
use raylib::prelude::*;
use crate::{coords::*, ordinals::*};

/// The direction items are transfered through a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Flow {
    Give = 1,
    Take = 2,
    #[default]
    Both = 3,
}

#[derive(Debug)]
pub struct ConnectorNode {
    pub position: FactoryVector3,
    pub rotation: Ordinal2D,
}

/// Each level doubles speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BeltLevel {
    Mk1 = 1 << 0,
    Mk2 = 1 << 1,
    Mk3 = 1 << 2,
    Mk4 = 1 << 3,
    Mk5 = 1 << 4,
    Mk6 = 1 << 5,
    Mk7 = 1 << 6,
    Mk8 = 1 << 7,
}

/// Belts are 1 meter wide, minimum 1 meter long, and have 1 meter vertical clearance.
#[derive(Debug)]
pub struct Belt {
    /// Each level doubles speed
    pub level: BeltLevel,
    pub src_position: FactoryVector3,
    pub src_rotation: Cardinal2D,
    pub dst_position: FactoryVector3,
    pub dst_rotation: Cardinal2D,
}

impl Belt {
    /// Cubic meters per sec
    pub const fn speed(&self) -> usize {
        self.level as usize
    }
}

#[derive(Debug)]
pub enum Connector {

}

/// Reacts two solutions to produce a pair of results
#[derive(Debug)]
pub struct Reactor {
    pub position: FactoryVector3,
    pub rotation: Cardinal2D,
}

impl Reactor {
    pub const fn clearance() -> [NonZeroU8; 3] {
        unsafe {
            [
                NonZeroU8::new_unchecked(2),
                NonZeroU8::new_unchecked(2),
                NonZeroU8::new_unchecked(3),
            ]
        }
    }

    pub const fn inputs(&self) -> [ConnectorNode; 2] {
        [
            ConnectorNode {
                position: self.position.plus(FactoryVector3 { x: 0, y: 0, z: 0 }),
                rotation: self.rotation.as_ordinal(),
            },
            ConnectorNode {
                position: self.position.plus(FactoryVector3 { x: 2, y: 0, z: 0 }),
                rotation: self.rotation.as_ordinal(),
            },
        ]
    }

    pub const fn outputs(&self) -> [ConnectorNode; 2] {
        [
            ConnectorNode {
                position: self.position.plus(FactoryVector3 { x: 0, y: 0, z: 3 }),
                rotation: self.rotation.as_ordinal(),
            },
            ConnectorNode {
                position: self.position.plus(FactoryVector3 { x: 2, y: 0, z: 3 }),
                rotation: self.rotation.as_ordinal(),
            },
        ]
    }

    // TODO: batch draws of same machine type
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let [width, height, length] = Self::clearance().map(|x| x.get() as f32);
        let position = self.position.to_rail(*factory_origin);
        d.draw_cube(
            (position.to_player() - *player_pos).to_vec3() + Vector3::new(width, height, length)*0.5,
            width,
            height,
            length,
            Color::GRAY,
        );
    }
}

#[derive(Debug)]
pub enum Machine {
    Reactor(Reactor),
    // todo: more machines
}

impl Machine {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    pub const fn clearance(&self) -> [NonZeroU8; 3] {
        match self {
            Self::Reactor(_) => Reactor::clearance(),
        }
    }

    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        match self {
            Machine::Reactor(reactor) => reactor.draw(d, thread, player_pos, factory_origin),
        }
    }
}

#[derive(Debug)]
pub struct Factory {
    pub origin: RailVector3,
    pub machines: Vec<Machine>,
}

impl Factory {
    pub fn draw(&self, d: &mut impl RaylibDraw3D, thread: &RaylibThread, player_pos: &PlayerVector3) {
        for machine in &self.machines {
            machine.draw(d, thread, player_pos, &self.origin);
        }
    }
}
