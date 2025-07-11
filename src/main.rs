use std::num::NonZeroU32;
use raylib::prelude::*;

pub mod units;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point3 {
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
    pub const fn as_orientation(self) -> Orientation2D {
        match self {
            Self::E => Orientation2D::E,
            Self::N => Orientation2D::N,
            Self::W => Orientation2D::W,
            Self::S => Orientation2D::S,
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

    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Orientation2D {
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

impl Orientation2D {
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

/// Each level doubles speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BeltLevel {
    Mk1 = 1,
    Mk2 = Self::Mk1 as u8 * 2,
    Mk3 = Self::Mk2 as u8 * 2,
    Mk4 = Self::Mk3 as u8 * 2,
    Mk5 = Self::Mk4 as u8 * 2,
    Mk6 = Self::Mk5 as u8 * 2,
    Mk7 = Self::Mk6 as u8 * 2,
    Mk8 = Self::Mk7 as u8 * 2,
}

/// Belts are 1 meter wide, minimum 1 meter long, and have 1 meter vertical clearance.
pub struct Belt {
    /// Each level doubles speed
    pub level: BeltLevel,
    pub src_position: Point3,
    pub src_rotation: Cardinal2D,
    pub dst_position: Point3,
    pub dst_rotation: Cardinal2D,
}

impl Belt {
    /// Cubic meters per sec
    pub const fn speed(&self) -> usize {
        self.level as usize
    }

    /// Length in floored meters (determines belt capacity and cost)
    pub const fn calc_length(src_pos: Point3, src_rot: Cardinal2D, dst_pos: Point3, dst_rot: Cardinal2D) -> usize {
        let rot_diff = dst_rot.minus(src_rot);

        todo!()
    }

    /// Length in meters (determines belt capacity and cost)
    pub const fn length(&self) -> usize {
        Self::calc_length(self.src_position, self.src_rotation, self.dst_position, self.dst_rotation)
    }
}

pub struct Constructor {

}

pub struct Assembler {

}

pub enum Machine {
    Ctor(Constructor),
    Asm(Assembler),
}

impl Machine {
    /// The dimensions of the machine in meters.
    /// `[length, width, height]`
    pub const fn clearance(&self) -> [NonZeroU32; 3] {
        match self {
            Self::Ctor(_) => [
                unsafe { NonZeroU32::new_unchecked(2) },
                unsafe { NonZeroU32::new_unchecked(1) },
                unsafe { NonZeroU32::new_unchecked(1) },
            ],
            Self::Asm(_) => [
                unsafe { NonZeroU32::new_unchecked(3) },
                unsafe { NonZeroU32::new_unchecked(2) },
                unsafe { NonZeroU32::new_unchecked(2) },
            ],
        }
    }
}

fn main() {
    let (mut rl, thread) = init()
        .title("factory game")
        .resizable()
        .build();

    rl.set_target_fps(60);
    rl.maximize_window();

    let machines: Vec<Machine> = Vec::new();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

    }
}
