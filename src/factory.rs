use std::num::NonZeroU8;
use arrayvec::ArrayVec;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BeltNode {
    pub position: FactoryVector3,
    pub rotation: Ordinal2D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BeltInputNode(pub BeltNode);

impl BeltInputNode {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self.0.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(
            player_rel_pos,
            1.0,
            1.0,
            1.0,
            Color::ORANGE,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BeltOutputNode(pub BeltNode);

impl BeltOutputNode {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self.0.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(
            player_rel_pos,
            1.0,
            1.0,
            1.0,
            Color::GREEN,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PipeNode {
    pub position: FactoryVector3,
    pub rotation: Ordinal3D,
}

impl PipeNode {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(
            player_rel_pos,
            1.0,
            1.0,
            1.0,
            Color::BLUE,
        );
    }
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
    pub src: BeltOutputNode,
    pub dst: BeltInputNode,
}

impl Belt {
    /// Cubic meters per sec
    pub const fn speed(&self) -> usize {
        self.level as usize
    }
}

#[derive(Debug)]
pub struct Pipe {
    pub a: PipeNode,
    pub b: PipeNode,
}

pub trait Machine {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    #[must_use]
    fn clearance(&self) -> [NonZeroU8; 3];

    /// Render the machine
    // TODO: batch draws of same machine type
    fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    );

    #[inline]
    #[must_use]
    fn belt_inputs(&self) -> ArrayVec<BeltInputNode, 8> {
        ArrayVec::new()
    }

    #[inline]
    #[must_use]
    fn belt_outputs(&self) -> ArrayVec<BeltOutputNode, 8> {
        ArrayVec::new()
    }

    #[inline]
    #[must_use]
    fn pipe_nodes(&self) -> ArrayVec<PipeNode, 8> {
        ArrayVec::new()
    }
}

/// Reacts two solutions to produce a pair of results
#[derive(Debug)]
pub struct Reactor {
    pub position: FactoryVector3,
    pub rotation: Cardinal2D,
}

impl Machine for Reactor {
    #[inline]
    fn clearance(&self) -> [NonZeroU8; 3] {
        unsafe {
            [
                NonZeroU8::new_unchecked(2),
                NonZeroU8::new_unchecked(2),
                NonZeroU8::new_unchecked(3),
            ]
        }
    }

    fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let [width, height, length] = self.clearance().map(|x| x.get() as f32);
        let player_rel_pos = self.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(
            player_rel_pos,
            width,
            height,
            length,
            Color::GRAY,
        );
    }

    fn belt_inputs(&self) -> ArrayVec<BeltInputNode, 8> {
        let mut arr = ArrayVec::new();
        let [_width, _height, _length] = self.clearance();
        arr.push(BeltInputNode(BeltNode {
            position: self.position + FactoryVector3 {
                x: 0,
                y: 0,
                z: 0,
            },
            rotation: self.rotation.as_ordinal(),
        }));
        arr
    }

    fn belt_outputs(&self) -> ArrayVec<BeltOutputNode, 8> {
        let mut arr = ArrayVec::new();
        let [_width, _height, length] = self.clearance();
        arr.push(BeltOutputNode(BeltNode {
            position: self.position + FactoryVector3 {
                x: 0,
                y: 0,
                z: length.get().into(),
            },
            rotation: self.rotation.as_ordinal(),
        }));
        arr
    }

    fn pipe_nodes(&self) -> ArrayVec<PipeNode, 8> {
        let mut arr = ArrayVec::new();
        let [width, _height, length] = self.clearance();
        arr.push(PipeNode {
            position: self.position + FactoryVector3 {
                x: width.get().into(),
                y: 0,
                z: 0,
            },
            rotation: self.rotation.as_ordinal().as_3D(),
        });
        arr.push(PipeNode {
            position: self.position + FactoryVector3 {
                x: width.get().into(),
                y: 0,
                z: length.get().into(),
            },
            rotation: self.rotation.as_ordinal().as_3D(),
        });
        arr
    }
}

#[derive(Debug)]
pub struct Resources {
    pub reactor_mesh: Mesh,
    pub reactor_material: WeakMaterial,
    /// NOT kept up to date--ONLY for reusing the allocation.
    reactor_transforms: Vec<Matrix>,
}

impl Resources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            reactor_mesh: Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0),
            reactor_material: rl.load_material_default(thread),
            reactor_transforms: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Factory {
    pub origin: RailVector3,
    pub reactors: Vec<Reactor>,
}

impl Factory {
    pub fn draw(&self, d: &mut impl RaylibDraw3D, thread: &RaylibThread, resources: &mut Resources, player_pos: &PlayerVector3) {
        let origin = &self.origin;

        resources.reactor_transforms.clear();
        resources.reactor_transforms.extend(
            self.reactors.iter()
                .map(|reactor| {
                    let Vector3 { x, y, z } = reactor.position.to_player_relative(player_pos, origin);
                    let (cos, sin, _) = reactor.rotation.as_ordinal().cos_sin_tan();
                    Matrix {
                        m0:  cos, m4: 0.0,  m8: sin, m12:   x,
                        m1:  0.0, m5: 1.0,  m9: 0.0, m13:   y,
                        m2: -sin, m6: 0.0, m10: cos, m14:   z,
                        m3:  0.0, m7: 0.0, m11: 0.0, m15: 1.0,
                    }
                })
        );
        d.draw_mesh_instanced(&resources.reactor_mesh, resources.reactor_material.clone(), &resources.reactor_transforms);

        // todo: other machines

        for belt_input in
            self.reactors.iter().flat_map(|reactor| reactor.belt_inputs())
            // todo: other machines
        {
            belt_input.draw(d, thread, player_pos, origin);
        }

        for belt_output in
            self.reactors.iter().flat_map(|reactor| reactor.belt_outputs())
            // todo: other machines
        {
            belt_output.draw(d, thread, player_pos, origin);
        }

        for pipe_node in
            self.reactors.iter().flat_map(|reactor| reactor.pipe_nodes())
            // todo: other machines
        {
            pipe_node.draw(d, thread, player_pos, origin);
        }
    }
}
