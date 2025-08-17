use crate::{resource::Resources, rl_helpers::DynRaylibDraw3D};
use raylib::prelude::*;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orbital {
    S,
    P,
    D,
    F,
}

impl Orbital {
    pub fn draw(
        self,
        d: &mut dyn DynRaylibDraw3D,
        _thread: &RaylibThread,
        resources: &Resources,
        matrix: Matrix,
        energy_level: u8,
    ) {
        let scale = energy_level.into();
        let model = match self {
            Self::S => &resources.orbital_s,
            Self::P => &resources.orbital_p,
            Self::D => &resources.orbital_d,
            Self::F => &resources.orbital_f,
        };
        d.draw_mesh(
            *model.meshes()[0],
            *model.materials()[0],
            Matrix::scale(scale, scale, scale) * matrix * (*model.transform()),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubLevel {
    S,
    P,
    D,
    F,
    G,
    H,
    I,
}

impl SubLevel {
    pub const fn orbitals(self) -> NonZeroU8 {
        // SAFETY: +1 guarantees nonzero
        unsafe { NonZeroU8::new_unchecked(2 * (self as u8) + 1) }
    }

    pub const fn capacity(self) -> NonZeroU8 {
        // SAFETY: No sublevel has enough electrons to overflow u8
        unsafe { NonZeroU8::new_unchecked(2 * self.orbitals().get()) }
    }

    /// The number of [`SubLevel`]s at energy level `n`
    pub const fn sublevels_at_energy(n: u8) -> u8 {
        if n > 0 { n / 2 + 1 } else { 0 }
    }

    /// The total electrons in an energy level with `self` as its highest sublevel
    pub const fn level_capacity_thru(self) -> u8 {
        let n = self as u8 + 1;
        2 * n * n
    }

    /// The total electron capacity of energy level `n`
    pub const fn level_capacity(n: u8) -> u8 {
        let sublevels = Self::sublevels_at_energy(n);
        2 * sublevels * sublevels
    }
}

const _: () = {
    assert!(SubLevel::sublevels_at_energy(0) == 0);
    assert!(SubLevel::sublevels_at_energy(1) == 1);
    assert!(SubLevel::sublevels_at_energy(2) == 2);
    assert!(SubLevel::sublevels_at_energy(3) == 2);
    assert!(SubLevel::sublevels_at_energy(4) == 3);
    assert!(SubLevel::sublevels_at_energy(5) == 3);
    assert!(SubLevel::sublevels_at_energy(6) == 4);
    assert!(SubLevel::sublevels_at_energy(7) == 4);
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElectronConfig(u8);

impl ElectronConfig {
    pub const fn new(electrons: u8) -> Self {
        Self(electrons)
    }

    pub fn available(self) -> u8 {
        todo!()
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[rustfmt::skip]
// pub enum SubLevel {
//     _1S = 1 << 2,
//     _2S = 2 << 2, _2P = (2 << 2) | 1,
//     _3S = 3 << 2, _3P = (3 << 2) | 1, _3D = (3 << 2) | 2,
//     _4S = 4 << 2, _4P = (4 << 2) | 1, _4D = (4 << 2) | 2, _4F = (4 << 2) | 3,
//     _5S = 5 << 2, _5P = (5 << 2) | 1, _5D = (5 << 2) | 2, _5F = (5 << 2) | 3,
//     _6S = 6 << 2, _6P = (6 << 2) | 1, _6D = (6 << 2) | 2,
//     _7S = 7 << 2, _7P = (7 << 2) | 1,
// }
// #[allow(
//     clippy::enum_glob_use,
//     reason = "I'm using all of them and don't want to repeat them"
// )]
// use SubLevel::*;

// impl std::fmt::Display for SubLevel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}{}", self.energy_level(), self.symbol())
//     }
// }

// #[rustfmt::skip]
// static ORBITALS: [SubLevel; 19] = [
//     _1S,
//     _2S,           _2P,
//     _3S,           _3P,
//     _4S,      _3D, _4P,
//     _5S,      _4D, _5P,
//     _6S, _4F, _5D, _6P,
//     _7S, _5F, _6D, _7P,
// ];

// impl SubLevel {
//     const fn index(self) -> u8 {
//         self as u8 & 3
//     }

//     pub const fn energy_level(self) -> u8 {
//         self as u8 >> 2
//     }

//     pub const fn symbol(self) -> char {
//         b"spdf"[self.index() as usize] as char
//     }

//     pub const fn orbitals(self) -> NonZeroU8 {
//         let n = self.index();
//         // SAFETY: Highest index is 6, which can be shl'd to 12 without overflowing.
//         let n = unsafe { n.unchecked_shl(1) };
//         // SAFETY: Highest valid is 12, which can be incremented to 13 without overflowing.
//         let n = unsafe { n.unchecked_add(1) };
//         // SAFETY: Adding 1 guarantees non-zero.
//         unsafe { NonZeroU8::new_unchecked(n) }
//     }

//     pub const fn capacity(self) -> NonZeroU8 {
//         // SAFETY: The highest orbital is `I` with 13.
//         // 13 << 1 = 26, which does not overflow.
//         let n = unsafe { self.orbitals().get().unchecked_shl(1) };
//         // SAFETY: nonzero multiplied by nonzero is nonzero, given no overflow
//         unsafe { NonZeroU8::new_unchecked(n) }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct ElectronConfig {
//     levels: u8,
//     /// Electrons in the outermost sublevel
//     outermost: u8,
// }

// impl ElectronConfig {
//     pub const fn new(mut electrons: u8) -> ElectronConfig {
//         let mut i = 0;
//         loop {
//             let cap = ORBITALS[i as usize].capacity().get();
//             if electrons > cap {
//                 electrons -= cap;
//                 i += 1;
//                 assert!((i as usize) < ORBITALS.len(), "too many electrons");
//             } else {
//                 break ElectronConfig {
//                     levels: i + (electrons > 0) as u8,
//                     outermost: electrons,
//                 };
//             }
//         }
//     }

//     pub const fn sublevels(self) -> &'static [SubLevel] {
//         ORBITALS.split_at(self.levels as usize).0
//     }

//     /// Total electrons at highest occupied energy level
//     pub const fn valance_electrons(self) -> u8 {
//         self.outermost
//     }

//     /// Number of electrons available for forming bonds
//     pub const fn available(self) -> u8 {
//         let capacity = match self.valance_capacity() {
//             Some(n) => n.get(),
//             None => 0,
//         };
//         let electrons = self.valance_electrons();
//         assert!(
//             electrons <= capacity,
//             "number of electrons in a given shell cannot exceed that shell's capacity"
//         );
//         capacity - electrons
//     }
// }

// impl std::fmt::Display for ElectronConfig {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         fn superscript(buf: &mut (String, String), n: u8) -> std::fmt::Result {
//             use std::fmt::Write;
//             buf.0.clear();
//             write!(buf.0, "{n}")?;
//             buf.1.clear();
//             buf.1
//                 .extend(buf.0.chars().map(|ch| ch.to_superscript().unwrap()));
//             Ok(())
//         }
//         let mut sublevels = self
//             .sublevels()
//             .iter()
//             .map(|o| (o, o.capacity().get()))
//             .collect::<Vec<_>>();
//         if let Some((_, n)) = sublevels.last_mut() {
//             *n = self.outermost;
//         }
//         sublevels.sort_by_key(|lv| lv.0.energy_level());
//         let total = sublevels.len();
//         let mut buf0 = String::new();
//         let mut buf1 = String::new();
//         for (n, (orbital, electrons)) in sublevels.into_iter().enumerate() {
//             use std::fmt::Write;
//             buf0.clear();
//             write!(buf0, "{electrons}")?;
//             buf1.clear();
//             buf1.extend(buf0.chars().map(|ch| ch.to_superscript().unwrap()));
//             write!(f, "{orbital}{buf1}")?;
//             if n < total - 1 {
//                 write!(f, " ")?;
//             }
//         }
//         Ok(())
//     }
// }
