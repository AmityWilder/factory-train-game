use crate::{
    coords::{FactoryVector3, PlayerVector3, RailVector3},
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

#[const_trait]
pub trait Clearance {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    #[must_use]
    fn clearance(&self) -> [NonZeroU8; 3];
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
}

/// Reacts two solutions
#[derive(Debug)]
pub struct Reactor {
    pub position: FactoryVector3,
    pub rotation: Cardinal2D,
}

impl const Clearance for Reactor {
    #[inline]
    fn clearance(&self) -> [NonZeroU8; 3] {
        [
            // SAFETY: 2 is not zero
            unsafe { NonZeroU8::new_unchecked(2) },
            // SAFETY: 2 is not zero
            unsafe { NonZeroU8::new_unchecked(2) },
            // SAFETY: 3 is not zero
            unsafe { NonZeroU8::new_unchecked(3) },
        ]
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
        let [width, height, length] = self.clearance().map(|x| x.get().into());
        let player_rel_pos = self.position.to_player_relative(player_pos, factory_origin);
        d.draw_cube(player_rel_pos, width, height, length, Color::GRAY);
    }

    fn belt_inputs(&self) -> ArrayVec<BeltInputNode, 8> {
        let mut arr = ArrayVec::new();
        let [_width, _height, _length] = self.clearance();
        arr.push(BeltInputNode(BeltNode {
            position: self.position + FactoryVector3 { x: 0, y: 0, z: 0 },
            rotation: self.rotation.as_ordinal(),
        }));
        arr
    }

    fn belt_outputs(&self) -> ArrayVec<BeltOutputNode, 8> {
        let mut arr = ArrayVec::new();
        let [_width, _height, length] = self.clearance();
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
        let [width, _height, length] = self.clearance();
        arr.push(PipeNode {
            position: self.position
                + FactoryVector3 {
                    x: width.get().into(),
                    y: 0,
                    z: 0,
                },
            rotation: self.rotation.as_ordinal().as_3D(),
        });
        arr.push(PipeNode {
            position: self.position
                + FactoryVector3 {
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
#[allow(
    clippy::struct_field_names,
    reason = "more resources will be added in the future"
)]
pub struct Resources {
    pub reactor_mesh: Mesh,
    pub reactor_material: Material,
    /// NOT kept up to date--ONLY for reusing the allocation.
    reactor_transforms: Vec<Matrix>,
}

impl Resources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut reactor_shader = rl.load_shader_from_memory(
            thread,
            Some(include_str!("../assets/lighting_instancing.vs")),
            Some(include_str!("../assets/lighting.fs")),
        );
        assert!(reactor_shader.is_shader_valid());
        reactor_shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_MATRIX_MVP as usize] =
            reactor_shader.get_shader_location("mvp");
        reactor_shader.locs_mut()[ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
            reactor_shader.get_shader_location("viewPos");
        reactor_shader.set_shader_value(
            reactor_shader.locs()[ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize],
            Vector3::ZERO,
        );
        reactor_shader.set_shader_value(
            reactor_shader.get_shader_location("ambient"),
            Vector4::new(0.2, 0.2, 0.2, 1.0),
        );
        Light::new(
            LightType::Directional,
            Vector3::new(0.0, 50.0, 0.0),
            Vector3::ZERO,
            Color::WHITE,
            &mut reactor_shader,
        )
        .unwrap();
        let image = Image::gen_image_gradient_linear(64, 64, 0, Color::GRAY, Color::LIGHTGRAY);
        let reactor_texture = rl.load_texture_from_image(thread, &image).unwrap();
        // SAFETY: Material loaded is unique to Resources and will not be double-freed
        let mut reactor_material = unsafe { Material::from_raw(*rl.load_material_default(thread)) };
        // SAFETY: Material unloads non-default shader on its own
        *reactor_material.shader_mut() = unsafe { reactor_shader.make_weak() };
        // SAFETY: Material unloads non-default textures on its own
        reactor_material.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, unsafe {
            reactor_texture.make_weak()
        });
        *reactor_material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
            Color::GRAY;
        assert!(reactor_material.is_material_valid());
        Self {
            reactor_mesh: Mesh::gen_mesh_cube(thread, 2.0, 2.0, 4.0),
            reactor_material,
            reactor_transforms: Vec::new(),
        }
    }

    pub fn reactor_material_weak(&self) -> WeakMaterial {
        // SAFETY: Material and WeakMaterial are both transparent wrappers of ffi::Material
        unsafe { std::mem::transmute_copy::<Material, WeakMaterial>(&self.reactor_material) }
    }
}

#[derive(Debug)]
pub struct Factory {
    pub origin: RailVector3,
    pub reactors: Vec<Reactor>,
}

impl Factory {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &mut Resources,
        player_pos: &PlayerVector3,
    ) {
        let origin = &self.origin;

        resources.reactor_transforms.clear();
        resources
            .reactor_transforms
            .extend(self.reactors.iter().map(|reactor| {
                let Vector3 { x, y, z } = reactor.position.to_player_relative(player_pos, origin);
                let (cos, sin, _) = reactor.rotation.as_ordinal().cos_sin_tan();
                #[rustfmt::skip]
                Matrix {
                    m0:  cos, m4: 0.0, m8:  sin, m12:   x,
                    m1:  0.0, m5: 1.0, m9:  0.0, m13:   y,
                    m2: -sin, m6: 0.0, m10: cos, m14:   z,
                    m3:  0.0, m7: 0.0, m11: 0.0, m15: 1.0,
                }
            }));
        if true {
            for transform in &resources.reactor_transforms {
                d.draw_mesh(
                    &resources.reactor_mesh,
                    resources.reactor_material_weak(),
                    transform,
                );
            }
        } else {
            d.draw_mesh_instanced(
                &resources.reactor_mesh,
                resources.reactor_material_weak(),
                &resources.reactor_transforms,
            );
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
