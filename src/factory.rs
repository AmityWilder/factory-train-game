use std::num::NonZeroU8;
use raylib::prelude::*;
use crate::coords::*;

/// A 2D cardinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Cardinal2D {
    #[default]
    East  = 0,
    North = 2,
    West  = 4,
    South = 6,
}

impl Cardinal2D {
    #[inline]
    pub const fn as_ordinal(self) -> Ordinal2D {
        match self {
            Self::East  => Ordinal2D::East,
            Self::North => Ordinal2D::North,
            Self::West  => Ordinal2D::West,
            Self::South => Ordinal2D::South,
        }
    }

    /// Subtract rhs from self
    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        let value = self.as_ordinal().minus(rhs.as_ordinal());

        // SAFETY: Cardinal add/sub guaranteed to result in a cardinal
        unsafe { value.as_cardinal_unchecked() }
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
    East = 0,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

impl Ordinal2D {
    #[inline]
    pub const fn try_as_cardinal(self) -> Option<Cardinal2D> {
        match self {
            Self::East  => Some(Cardinal2D::East ),
            Self::North => Some(Cardinal2D::North),
            Self::West  => Some(Cardinal2D::West ),
            Self::South => Some(Cardinal2D::South),
            _ => None,
        }
    }

    /// # Safety
    /// Must be `East`, `North`, `West`, or `South`
    #[inline]
    pub const unsafe fn as_cardinal_unchecked(self) -> Cardinal2D {
        debug_assert!(matches!(self, Self::East|Self::North|Self::West|Self::South));
        // SAFETY: Caller must uphold safety contract
        unsafe { std::mem::transmute::<Self, Cardinal2D>(self) }
    }

    /// Converts orientation to an angle in radians
    #[inline]
    pub const fn radians(self) -> f32 {
        self as u8 as f32 * std::f32::consts::FRAC_PI_8
    }

    /// Calculates the sine, cosine, and tangent of the orientation simultaneously
    #[inline]
    pub const fn cos_sin_tan(self) -> (f32, f32, f32) {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            Self::East      => (           1.0,            0.0,                0.0),
            Self::Northeast => ( FRAC_1_SQRT_2,  FRAC_1_SQRT_2,                1.0),
            Self::North     => (           0.0,            1.0,  f32::    INFINITY),
            Self::Northwest => (-FRAC_1_SQRT_2,  FRAC_1_SQRT_2, -              1.0),
            Self::West      => (-          1.0,            0.0, -              0.0),
            Self::Southwest => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2,                1.0),
            Self::South     => (           0.0, -          1.0,  f32::NEG_INFINITY),
            Self::Southeast => ( FRAC_1_SQRT_2, -FRAC_1_SQRT_2, -              1.0),
        }
    }

    /// Add rhs to self
    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        // NOTE: only works so easily because the ordinals are modulo 8
        let n = (self as u8 + rhs as u8) & 7;
        // SAFETY: `n` is masked to within enum discriminant range
        unsafe { std::mem::transmute::<u8, Self>(n) }
    }

    /// Subtract rhs from self
    #[inline]
    pub const fn minus(self, rhs: Self) -> Self {
        // NOTE: only works so easily because the ordinals are modulo 8
        let n = (self as u8).wrapping_sub(rhs as u8) & 7;
        // SAFETY: `n` is masked to within enum discriminant range
        unsafe { std::mem::transmute::<u8, Self>(n) }
    }
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

pub struct BeltNode {
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

/// Reacts two solutions to produce a pair of results
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

    pub const fn inputs(&self) -> [FactoryVector3; 2] {
        todo!()
    }

    pub const fn outputs(&self) -> [FactoryVector3; 2] {
        todo!()
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
