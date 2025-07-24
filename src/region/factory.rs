use crate::{
    math::{
        bounds::{Bounds, FactoryBounds},
        coords::{FactoryVector3, PlayerCoord, PlayerVector3, RailVector3, VectorConstants},
    },
    ordinals::{Cardinal2D, Ordinal2D, Ordinal3D},
    player::Player,
    region::factory::grid_vis::GridVisualizer,
    resource::Resources,
};
use arrayvec::ArrayVec;
use raylib::prelude::*;
use std::num::NonZeroU8;

pub mod grid_vis;

/// Get collision info between ray and box
#[inline]
pub fn get_ray_collision_box(ray: Ray, box_: BoundingBox) -> RayCollision {
    // SAFETY: GetRayCollisionBox has no preconditions and is a pure math function
    unsafe { ffi::GetRayCollisionBox(ray.into(), box_.into()) }.into()
}

fn get_ray_collision_plane(ray: Ray, point: Vector3, normal: Vector3) -> RayCollision {
    let mut collision = RayCollision {
        hit: false,
        distance: 0.0,
        point: Vector3::ZERO,
        normal: Vector3::ZERO,
    };

    let distance = (point - ray.position).dot(normal) / ray.direction.dot(normal);

    if distance >= 0.0 {
        collision.hit = true;
        collision.distance = distance;
        collision.point = ray.position + ray.direction * distance;
        collision.normal = normal;
    }

    collision
}

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
        self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self
            .0
            .position
            .to_player_relative(player_pos, factory_origin);
        d.draw_cube(player_rel_pos, 1.0, 1.0, 1.0, Color::ORANGE);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BeltOutputNode(pub BeltNode);

impl BeltOutputNode {
    pub fn draw(
        self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self
            .0
            .position
            .to_player_relative(player_pos, factory_origin);
        d.draw_cube(player_rel_pos, 1.0, 1.0, 1.0, Color::GREEN);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PipeNode {
    pub position: FactoryVector3,
    pub rotation: Ordinal3D,
}

impl PipeNode {
    pub fn draw(
        self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let player_rel_pos = self.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(player_rel_pos, 1.0, 1.0, 1.0, Color::BLUE);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachineSize {
    pub width: NonZeroU8,
    pub height: NonZeroU8,
    pub length: NonZeroU8,
}

impl MachineSize {
    /// # Safety
    ///
    /// All parameters must be non-zero
    #[inline]
    pub const unsafe fn new_unchecked(width: u8, height: u8, length: u8) -> Self {
        Self {
            // SAFETY: Caller must uphold safety contract
            width: unsafe { NonZeroU8::new_unchecked(width) },
            // SAFETY: Caller must uphold safety contract
            height: unsafe { NonZeroU8::new_unchecked(height) },
            // SAFETY: Caller must uphold safety contract
            length: unsafe { NonZeroU8::new_unchecked(length) },
        }
    }
}

#[const_trait]
pub trait Clearance {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    #[must_use]
    fn clearance(&self) -> MachineSize;
}

pub trait Machine: Clearance + Bounds<FactoryVector3, BoundingBox = FactoryBounds> {
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

pub trait DrawMachine: Machine {
    /// Render the machine
    // TODO: batch draws of same machine type
    fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    );
}

/// Reacts two solutions
#[derive(Debug)]
pub struct Reactor {
    pub position: FactoryVector3,
    pub rotation: Cardinal2D,
}

impl const Clearance for Reactor {
    #[inline]
    fn clearance(&self) -> MachineSize {
        // SAFETY: 2 and 3 are not zero
        unsafe { MachineSize::new_unchecked(2, 2, 3) }
    }
}

impl Bounds<FactoryVector3> for Reactor {
    type BoundingBox = FactoryBounds;

    fn bounds(&self) -> Self::BoundingBox {
        let FactoryVector3 { x, y, z } = self.position;
        let MachineSize {
            width,
            height,
            length,
        } = self.clearance();
        let width: i16 = width.get().into();
        let height: i16 = height.get().into();
        let length: i16 = length.get().into();
        let (cos, sin, _) = self.rotation.cos_sin_tan();
        #[allow(
            clippy::cast_possible_truncation,
            reason = "cos and sin of Cardinal2D are guaranteed to be -1, 0, or 1"
        )]
        let (cos, sin) = (cos as i16, sin as i16);
        let width = cos * width + sin * length;
        let length = sin * width + cos * length;
        let (mut xs, mut zs) = ([x, x + width], [z, z + length]);
        for a in [&mut xs, &mut zs] {
            if !a.is_sorted() {
                a.reverse();
            }
        }
        let ([xmin, xmax], [zmin, zmax]) = (xs, zs);
        // height is never negative (at the time of writing)
        FactoryBounds {
            min: FactoryVector3 {
                x: xmin,
                y,
                z: zmin,
            },
            max: FactoryVector3 {
                x: xmax,
                y: y + height,
                z: zmax,
            },
        }
    }
}

impl Machine for Reactor {
    fn belt_inputs(&self) -> ArrayVec<BeltInputNode, 8> {
        let mut arr = ArrayVec::new();
        arr.push(BeltInputNode(BeltNode {
            position: self.position + FactoryVector3 { x: 0, y: 0, z: 0 },
            rotation: self.rotation.as_ordinal(),
        }));
        arr
    }

    fn belt_outputs(&self) -> ArrayVec<BeltOutputNode, 8> {
        let mut arr = ArrayVec::new();
        let MachineSize { length, .. } = self.clearance();
        arr.push(BeltOutputNode(BeltNode {
            position: self.position
                + FactoryVector3 {
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
        let MachineSize { width, length, .. } = self.clearance();
        arr.push(PipeNode {
            position: self.position
                + FactoryVector3 {
                    x: width.get().into(),
                    y: 0,
                    z: 0,
                },
            rotation: self.rotation.as_ordinal().as_3d(),
        });
        arr.push(PipeNode {
            position: self.position
                + FactoryVector3 {
                    x: width.get().into(),
                    y: 0,
                    z: length.get().into(),
                },
            rotation: self.rotation.as_ordinal().as_3d(),
        });
        arr
    }
}

impl DrawMachine for Reactor {
    fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        player_pos: &PlayerVector3,
        factory_origin: &RailVector3,
    ) {
        let size = self.clearance();
        let player_rel_pos = self.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(
            player_rel_pos,
            size.width.get().into(),
            size.height.get().into(),
            size.length.get().into(),
            Color::GRAY,
        );
    }
}

pub const fn machine_matrix(
    player_pos: &PlayerVector3,
    position: FactoryVector3,
    origin: &RailVector3,
    rotation: Cardinal2D,
) -> Matrix {
    let Vector3 { x, y, z } = position.to_player_relative(player_pos, origin);
    let (cos, sin, _) = rotation.cos_sin_tan();
    #[rustfmt::skip]
    Matrix {
        m0:  cos, m4: 0.0, m8:  sin, m12:   x,
        m1:  0.0, m5: 1.0, m9:  0.0, m13:   y,
        m2: -sin, m6: 0.0, m10: cos, m14:   z,
        m3:  0.0, m7: 0.0, m11: 0.0, m15: 1.0,
    }
}

/// Note: vectors are in Factory coordinates
pub struct FactoryCollision<'a> {
    pub target: Option<&'a dyn Machine>,
    pub distance: f32,
    pub normal: Vector3,
    pub point: Vector3,
}

#[derive(Debug)]
pub struct Factory {
    pub origin: RailVector3,
    pub bounds: FactoryBounds,
    pub reactors: Vec<Reactor>,
}

impl Factory {
    /// Cast a ray and see what it hits
    pub fn get_ray_collision(&self, ray: Ray) -> Option<FactoryCollision<'_>> {
        std::iter::once_with(|| {
            let RayCollision {
                hit,
                distance,
                point,
                normal,
            } = get_ray_collision_plane(ray, Vector3::ZERO, Vector3::UP);

            if hit {
                Some(FactoryCollision {
                    target: None,
                    distance,
                    normal,
                    point,
                })
            } else {
                None
            }
        })
        .chain(self.reactors.iter().map(|reactor| {
            let bbox = reactor.bounds();
            let bbox = BoundingBox {
                min: Vector3 {
                    x: bbox.min.x.into(),
                    y: bbox.min.y.into(),
                    z: bbox.min.z.into(),
                },
                max: Vector3 {
                    x: bbox.max.x.into(),
                    y: bbox.max.y.into(),
                    z: bbox.max.z.into(),
                },
            };
            let RayCollision {
                hit,
                distance,
                point,
                normal,
            } = get_ray_collision_box(ray, bbox);

            hit.then_some(FactoryCollision {
                target: Some(reactor),
                distance,
                normal,
                point,
            })
        }))
        .flatten()
        .min_by_key(|collision| PlayerCoord::from_f32(collision.distance))
    }

    fn draw_machines(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player_pos: &PlayerVector3,
        origin: &RailVector3,
    ) {
        let reactor_model_transform = *resources.reactor.transform();
        for reactor in &self.reactors {
            let matrix = machine_matrix(player_pos, reactor.position, origin, reactor.rotation)
                * reactor_model_transform;
            d.draw_mesh(
                &resources.reactor.meshes()[0],
                resources.reactor.materials()[0].clone(),
                matrix,
            );
            let bounds = reactor.bounds();
            let bbox = BoundingBox {
                min: bounds.min.to_player_relative(player_pos, origin),
                max: bounds.max.to_player_relative(player_pos, origin),
            };
            d.draw_bounding_box(bbox, Color::MAGENTA);
        }

        // todo: other machines

        for belt_input in self.reactors.iter().flat_map(Machine::belt_inputs)
        // todo: chain other machines
        {
            belt_input.draw(d, thread, player_pos, origin);
        }

        for belt_output in self.reactors.iter().flat_map(Machine::belt_outputs)
        // todo: chain other machines
        {
            belt_output.draw(d, thread, player_pos, origin);
        }

        for pipe_node in self.reactors.iter().flat_map(Machine::pipe_nodes)
        // todo: chain other machines
        {
            pipe_node.draw(d, thread, player_pos, origin);
        }
    }

    fn draw_highlight(
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        _resources: &Resources,
        player_pos: &PlayerVector3,
        origin: &RailVector3,
        player_lookat: &FactoryCollision<'_>,
    ) {
        if let Some(target) = player_lookat.target {
            const EXPAND: Vector3 = Vector3::splat(0.025);
            let bbox = target.bounds();
            let mut bbox = BoundingBox {
                min: bbox.min.to_player_relative(player_pos, origin),
                max: bbox.max.to_player_relative(player_pos, origin),
            };
            bbox.min -= EXPAND;
            bbox.max += EXPAND;
            d.draw_bounding_box(bbox, Color::YELLOW);
        } else {
            #[allow(clippy::cast_possible_truncation, reason = "this is intentional")]
            let position_in_factory = FactoryVector3 {
                x: player_lookat.point.x as i16,
                y: player_lookat.point.y as i16,
                z: player_lookat.point.z as i16,
            };
            let point = position_in_factory.to_player_relative(player_pos, origin)
                + Vector3::new(0.5, 0.5, 0.5);
            d.draw_line3D(
                point + Vector3::BACKWARD,
                point + Vector3::FORWARD,
                Color::BLUE,
            );
            d.draw_line3D(point + Vector3::LEFT, point + Vector3::RIGHT, Color::RED);
            d.draw_line3D(point + Vector3::DOWN, point + Vector3::UP, Color::GREEN);
            d.draw_cube_wires_v(point, Vector3::new(1.0, 1.0, 1.0), Color::WHITE);
        }
    }

    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
        grid: &GridVisualizer,
    ) {
        let origin = &self.origin;
        let player_pos = &player.position;
        let player_vision_ray = player.vision_ray();
        let player_lookat = self.get_ray_collision(player_vision_ray);

        grid.draw(d, thread, resources, player_pos, self);
        if let Some(player_lookat) = &player_lookat {
            Self::draw_highlight(d, thread, resources, player_pos, origin, player_lookat);
        }
        self.draw_machines(d, thread, resources, player_pos, origin);
    }
}
