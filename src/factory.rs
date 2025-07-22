use crate::{
    coords::{
        BACKWARD, DOWN, FORWARD, FactoryVector3, LEFT, PlayerVector3, RIGHT, RailVector3, UP,
    },
    ordinals::{Cardinal2D, Ordinal2D, Ordinal3D},
    rlights::{Light, LightType},
};
use arrayvec::ArrayVec;
use raylib::prelude::*;
use std::num::NonZeroU8;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineBounds {
    pub x: std::ops::Range<i16>,
    pub y: std::ops::Range<i16>,
    pub z: std::ops::Range<i16>,
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

pub trait Machine: Clearance {
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

    #[must_use]
    fn bounding_box(&self) -> MachineBounds;
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

impl Machine for Reactor {
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

    fn bounding_box(&self) -> MachineBounds {
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
        MachineBounds {
            x: (xmin..xmax),
            y: (y..y + height), // height is never negative (at the time of writing)
            z: (zmin..zmax),
        }
    }
}

#[derive(Debug)]
#[allow(
    clippy::struct_field_names,
    reason = "more resources will be added in the future"
)]
pub struct Resources {
    pub skybox: Texture2D,
    pub reactor: Model,
}

impl Resources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            skybox: {
                let image = Image::gen_image_gradient_radial(
                    256,
                    256,
                    0.1,
                    Color::DODGERBLUE,
                    Color::CORAL,
                );
                rl.load_texture_from_image(thread, &image).unwrap()
            },
            reactor: {
                // Mesh
                let mesh = Mesh::gen_mesh_cube(thread, 2.0, 2.0, 3.0);

                let mut mat = rl.load_material_default(thread);

                // Shader
                let mut shader = rl.load_shader_from_memory(
                    thread,
                    Some(include_str!("../assets/lighting.vs")),
                    Some(include_str!("../assets/lighting.fs")),
                );
                assert!(shader.is_shader_valid());
                shader.set_shader_value(
                    shader.get_shader_location("ambient"),
                    Vector4::new(0.2, 0.2, 0.2, 1.0),
                );
                Light::new(
                    LightType::Directional,
                    Vector3::new(0.0, 50.0, 0.0),
                    Vector3::ZERO,
                    Color::WHITE,
                    &mut shader,
                )
                .unwrap();
                // SAFETY: Material unloads non-default shader on its own
                *mat.shader_mut() = unsafe { shader.make_weak() };

                // Color
                *mat.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::GRAY;

                // Texture
                let image =
                    Image::gen_image_gradient_linear(64, 64, 0, Color::GRAY, Color::LIGHTGRAY);
                let reactor_texture = rl.load_texture_from_image(thread, &image).unwrap();
                // SAFETY: Material unloads non-default textures on its own
                mat.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, unsafe {
                    reactor_texture.make_weak()
                });
                assert!(mat.is_material_valid());

                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = mat;
                model.transform = Matrix::translate(1.0, 1.0, 1.5).into();

                assert!(model.is_model_valid());
                model
            },
        }
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

#[derive(Debug)]
pub struct Factory {
    pub origin: RailVector3,
    pub reactors: Vec<Reactor>,
}

impl Factory {
    fn draw_world_grid(
        d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        _resources: &mut Resources,
        player_pos: &PlayerVector3,
        origin: &RailVector3,
    ) {
        const GRID_SIZE: i16 = 100;

        let position_in_factory = player_pos.to_factory(origin).unwrap();

        let player_snapped = position_in_factory.to_player_relative(player_pos, origin)
            + Vector3::new(0.5, 0.5, 0.5);

        d.draw_line3D(
            player_snapped + BACKWARD,
            player_snapped + FORWARD,
            Color::BLUE,
        );
        d.draw_line3D(player_snapped + LEFT, player_snapped + RIGHT, Color::RED);
        d.draw_line3D(player_snapped + DOWN, player_snapped + UP, Color::GREEN);
        d.draw_cube_wires_v(player_snapped, Vector3::new(1.0, 1.0, 1.0), Color::WHITE);

        for x in (-GRID_SIZE)..GRID_SIZE {
            d.draw_line3D(
                FactoryVector3 {
                    x: x + position_in_factory.x,
                    y: 0,
                    z: position_in_factory.z - GRID_SIZE,
                }
                .to_player_relative(player_pos, origin),
                FactoryVector3 {
                    x: x + position_in_factory.x,
                    y: 0,
                    z: position_in_factory.z + GRID_SIZE,
                }
                .to_player_relative(player_pos, origin),
                Color::RED,
            );
        }
        for z in (-GRID_SIZE)..GRID_SIZE {
            d.draw_line3D(
                FactoryVector3 {
                    x: position_in_factory.x - GRID_SIZE,
                    y: 0,
                    z: position_in_factory.z + z,
                }
                .to_player_relative(player_pos, origin),
                FactoryVector3 {
                    x: position_in_factory.x + GRID_SIZE,
                    y: 0,
                    z: position_in_factory.z + z,
                }
                .to_player_relative(player_pos, origin),
                Color::BLUE,
            );
        }
    }

    fn draw_skybox(
        _d: &mut impl RaylibDraw3D,
        _thread: &RaylibThread,
        resources: &mut Resources,
        _player_pos: &PlayerVector3,
        _origin: &RailVector3,
    ) {
        #[allow(
            clippy::cast_possible_wrap,
            reason = "RL_QUADS is an i32 in Raylib, but bindgen made it a u32"
        )]
        const RL_QUADS: i32 = ffi::RL_QUADS as i32;

        #[allow(
            clippy::multiple_unsafe_ops_per_block,
            reason = "safety comment is complicated and shared by all operations in this block"
        )]
        // SAFETY: RaylibDraw3D is exclusively borrowed, guaranteeing the window has been
        // initialized, 3D drawing processes are loaded, and rlgl statics are syncronous
        // for this function (assuming no soundness holes outside of this function).
        // RaylibThread (which does not implement Send/Sync) is borrowed, guaranteeing
        // this is the thread that initialized the window and graphics.
        unsafe {
            ffi::rlSetTexture(resources.skybox.id);
            ffi::rlBegin(RL_QUADS);
            {
                ffi::rlColor4ub(255, 255, 255, 255);

                ffi::rlTexCoord2f(0.0, 1.0);
                ffi::rlVertex3f(-1000.0, 50.0, -1000.0);

                ffi::rlTexCoord2f(1.0, 1.0);
                ffi::rlVertex3f(1000.0, 50.0, -1000.0);

                ffi::rlTexCoord2f(1.0, 0.0);
                ffi::rlVertex3f(1000.0, 50.0, 1000.0);

                ffi::rlTexCoord2f(0.0, 0.0);
                ffi::rlVertex3f(-1000.0, 50.0, 1000.0);
            }
            ffi::rlEnd();
            ffi::rlSetTexture(0);
        }
    }

    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &mut Resources,
        player_pos: &PlayerVector3,
    ) {
        let origin = &self.origin;

        Self::draw_world_grid(d, thread, resources, player_pos, origin);
        Self::draw_skybox(d, thread, resources, player_pos, origin);

        let reactor_model_transform = *resources.reactor.transform();
        for reactor in &self.reactors {
            let matrix = machine_matrix(player_pos, reactor.position, origin, reactor.rotation)
                * reactor_model_transform;
            d.draw_mesh(
                &resources.reactor.meshes()[0],
                resources.reactor.materials()[0].clone(),
                matrix,
            );
            let bounds = reactor.bounding_box();
            let bbox = BoundingBox {
                min: FactoryVector3 {
                    x: bounds.x.start,
                    y: bounds.y.start,
                    z: bounds.z.start,
                }
                .to_player_relative(player_pos, origin),
                max: FactoryVector3 {
                    x: bounds.x.end,
                    y: bounds.y.end,
                    z: bounds.z.end,
                }
                .to_player_relative(player_pos, origin),
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
}
