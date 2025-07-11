#![allow(dead_code)]

use std::num::NonZeroU32;
use raylib::prelude::*;

pub mod coords;
use crate::coords::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Cardinal2D {
    #[default]
    E = 0,
    N = 2,
    W = 4,
    S = 6,
}

impl Cardinal2D {
    pub const fn as_orientation(self) -> Ordinal2D {
        match self {
            Self::E => Ordinal2D::E,
            Self::N => Ordinal2D::N,
            Self::W => Ordinal2D::W,
            Self::S => Ordinal2D::S,
        }
    }

    /// Subtract rhs from self
    pub const fn minus(self, rhs: Self) -> Self {
        let n = (self as u8).wrapping_sub(rhs as u8) & 7;
        debug_assert!(matches!(n, 0|2|4|6));
        unsafe { std::mem::transmute::<u8, Self>(n) }
    }
}

impl std::ops::Sub for Cardinal2D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Ordinal2D {
    #[default]
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}

impl Ordinal2D {
    /// Converts orientation to an angle in radians
    pub const fn radians(self) -> f32 {
        self as u8 as f32 * std::f32::consts::FRAC_PI_8
    }

    /// Calculates the sine, cosine, and tangent of the orientation simultaneously
    pub const fn cos_sin_tan(self) -> (f32, f32, f32) {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            Self::E => (1.0, 0.0, 0.0),
            Self::NE => (FRAC_1_SQRT_2, FRAC_1_SQRT_2, 1.0),
            Self::N => (0.0, 1.0, f32::INFINITY),
            Self::NW => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2, -1.0),
            Self::W => (-1.0, 0.0, -0.0),
            Self::SW => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 1.0),
            Self::S => (0.0, -1.0, f32::NEG_INFINITY),
            Self::SE => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2, -1.0),
        }
    }
}

/// The direction items are transfered through a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Flow {
    Give = 1,
    Take = 2,
    #[default]
    Both = 3,
}

pub struct BeltNode {
    pub position: Point3D,
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
pub struct Belt {
    /// Each level doubles speed
    pub level: BeltLevel,
    pub src_position: Point3D,
    pub src_rotation: Cardinal2D,
    pub dst_position: Point3D,
    pub dst_rotation: Cardinal2D,
}

impl Belt {
    /// Cubic meters per sec
    pub const fn speed(&self) -> usize {
        self.level as usize
    }
}

/// Reacts two solutions to produce a pair of results
pub struct Reactor {
    pub position: FactoryVector3,
    pub rotation: Cardinal2D,
}

impl Reactor {
    pub const fn clearance(&self) -> FactorySize3 {
        FactorySize3 {
            width: 2,
            height: 2,
            length: 3,
        }
    }

    pub const fn inputs(&self) -> [FactoryVector3; 2] {
        todo!()
    }

    pub const fn outputs(&self) -> [FactoryVector3; 2] {
        todo!()
    }

    // TODO: batch draws of same machine type
    pub fn draw(&self, d: &mut impl RaylibDraw3D, _thread: &RaylibThread, player_pos: &PlayerVector3, factory_origin: &RailVector3) {
        let size = self.clearance();
        let position = self.position.to_rail(*factory_origin);
        d.draw_cube(
            (position.to_player() - *player_pos).to_vec3(),
            size.width as f32,
            size.height as f32,
            size.length as f32,
            Color::BLUE,
        );
    }
}

pub enum Machine {
    Reactor(Reactor),
    // todo: more machines
}

impl Machine {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    pub const fn clearance(&self) -> [NonZeroU32; 3] {
        match self {
            Self::Reactor(_) => [
                unsafe { NonZeroU32::new_unchecked(2) },
                unsafe { NonZeroU32::new_unchecked(1) },
                unsafe { NonZeroU32::new_unchecked(1) },
            ],
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw3D, thread: &RaylibThread, player_pos: &PlayerVector3, factory_origin: &RailVector3) {
        match self {
            Machine::Reactor(reactor) => reactor.draw(d, thread, player_pos, factory_origin),
        }
    }
}

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

pub struct Player {
    pub position: PlayerVector3,
}

fn main() {
    use KeyboardKey::*;
    // use MouseButton::*;

    let (mut rl, thread) = init()
        .title("factory game")
        .resizable()
        .build();

    rl.set_target_fps(60);
    rl.maximize_window();

    let font = rl.load_font_from_memory(&thread, ".ttf", include_bytes!("./FiraCode-Regular.ttf"), 20, None).unwrap();

    let mut player = Player {
        position: PlayerVector3::new(0, 0, 0),
    };

    let factory: Factory = Factory {
        origin: RailVector3 { x: 0, y: 0, z: -10 },
        machines: vec![
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: 5, y: 0, z: -4 },
                rotation: Cardinal2D::N,
            }),
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: -5, y: 2, z: -4 },
                rotation: Cardinal2D::N,
            }),
        ],
    };

    while !rl.window_should_close() {
        // player movement
        {
            let movement_forward = (rl.is_key_down(KEY_S) as i8 - rl.is_key_down(KEY_W) as i8) as f32;
            let movement_right = (rl.is_key_down(KEY_D) as i8 - rl.is_key_down(KEY_A) as i8) as f32;
            player.position += PlayerVector3::from_vec3(Vector3::new(movement_right, 0.0, movement_forward) * 5.0 *  rl.get_frame_time());
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(Camera3D::perspective(Vector3::ZERO, Vector3::NEG_Z, Vector3::Y, 45.0));
            factory.draw(&mut d, &thread, &player.position);
        }

        d.draw_text_ex(
            &font,
            &format!("player position: {:X}\n                 {}", player.position, player.position.to_vec3()),
            Vector2::new(500.0, 500.0),
            20.0,
            0.0,
            Color::MAGENTA,
        );

        d.draw_fps(0, 0);
    }
}
