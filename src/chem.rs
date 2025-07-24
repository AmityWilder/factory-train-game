use raylib::prelude::*;
use std::num::NonZeroU8;

pub const AVOGADROS_NUMBER: f64 = 6.022_140_76e+26;
/// Atomic mass units (AMU) per kilogram (kg)
pub const AMU_PER_KG: f64 = AVOGADROS_NUMBER;
/// Kilograms (kg) per atomic mass unit (AMU)
pub const KG_PER_AMU: f64 = AMU_PER_KG.recip();

/// Picometers (pm) per meter (m)
pub const PM_PER_M: f64 = 1e+12;
/// Meters (m) per picometer (pm)
pub const M_PER_PM: f64 = PM_PER_M.recip();

/// Mass of a single proton in AMU
pub const PROTON_MASS: f64 = 1.007_276_466_879_91;
/// Mass of a single neutron in AMU
pub const NEUTRON_MASS: f64 = 1.008_106;
/// Mass of a single electron in AMU
pub const ELECTRON_MASS: f64 = 5.485_799_090_701_6e-4;

// S: Spherical
// P: Dumbell
// D: Clover
// F: 8 knotted balloons

// Steric number describes molecular geometry

// valance electrons are always the same in a single column of the periodic table

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Element {
// |   S   |                           F                           |                   D                   |           P           |
// |-------|-------------------------------------------------------|---------------------------------------|-----------------------|
    H = 1,                                                                                                                      He, // n=1
    Li, Be,                                                                                                 B,  C,  N,  O,  F,  Ne, // n=2
    Na, Mg,                                                                                                 Al, Si, P,  S,  Cl, Ar, // n=3
    K,  Ca,                                                         Sc, Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, // n=4
    Rb, Sr,                                                         Y,  Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I,  Xe, // n=5
    Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, // n=6
    Fr, Ra, Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og, // n=7
}
#[allow(
    clippy::enum_glob_use,
    reason = "I am importing all of them and don't want to repeat all 118 names. They don't shadow anything else here."
)]
use Element::*;

use crate::{resource::Resources, rl_helpers::DynRaylibDraw3D};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NobleGas {
    He = He as isize,
    Ne = Ne as isize,
    Ar = Ar as isize,
    Kr = Kr as isize,
    Xe = Xe as isize,
    Rn = Rn as isize,
    Og = Og as isize,
}

impl From<NobleGas> for Element {
    #[inline]
    fn from(value: NobleGas) -> Self {
        value.as_element()
    }
}

impl NobleGas {
    #[inline]
    pub const fn try_from_element(element: Element) -> Option<Self> {
        if matches!(element, He | Ne | Ar | Kr | Xe | Rn | Og) {
            // SAFETY: Checked and element is a noble gas
            Some(unsafe { std::mem::transmute::<Element, Self>(element) })
        } else {
            None
        }
    }

    #[inline]
    pub const fn as_element(self) -> Element {
        // SAFETY: NobleGas is a subset of Element
        unsafe { std::mem::transmute::<Self, Element>(self) }
    }
}

impl std::fmt::Display for Element {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.symbol().fmt(f)
    }
}

static ELEMENT_LIST: [Element; 118] = [
    H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca, Sc, Ti, V, Cr, Mn, Fe,
    Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn,
    Sb, Te, I, Xe, Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,
    Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf,
    Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
];

#[rustfmt::skip]
static ELEMENT_INFO: [(&str, &str); 118] = [
    ("H",  "Hydrogen"     ),
    ("He", "Helium"       ),
    ("Li", "Lithium"      ),
    ("Be", "Beryllium"    ),
    ("B",  "Boron"        ),
    ("C",  "Carbon"       ),
    ("N",  "Nitrogen"     ),
    ("O",  "Oxygen"       ),
    ("F",  "Fluorine"     ),
    ("Ne", "Neon"         ),
    ("Na", "Sodium"       ),
    ("Mg", "Magnesium"    ),
    ("Al", "Aluminium"    ),
    ("Si", "Silicon"      ),
    ("P",  "Phosphorus"   ),
    ("S",  "Sulfur"       ),
    ("Cl", "Chlorine"     ),
    ("Ar", "Argon"        ),
    ("K",  "Potassium"    ),
    ("Ca", "Calcium"      ),
    ("Sc", "Scandium"     ),
    ("Ti", "Titanium"     ),
    ("V",  "Vanadium"     ),
    ("Cr", "Chromium"     ),
    ("Mn", "Manganese"    ),
    ("Fe", "Iron"         ),
    ("Co", "Cobalt"       ),
    ("Ni", "Nickel"       ),
    ("Cu", "Copper"       ),
    ("Zn", "Zinc"         ),
    ("Ga", "Gallium"      ),
    ("Ge", "Germanium"    ),
    ("As", "Arsenic"      ),
    ("Se", "Selenium"     ),
    ("Br", "Bromine"      ),
    ("Kr", "Krypton"      ),
    ("Rb", "Rubidium"     ),
    ("Sr", "Strontium"    ),
    ("Y",  "Yttrium"      ),
    ("Zr", "Zirconium"    ),
    ("Nb", "Niobium"      ),
    ("Mo", "Molybdenum"   ),
    ("Tc", "Technetium"   ),
    ("Ru", "Ruthenium"    ),
    ("Rh", "Rhodium"      ),
    ("Pd", "Palladium"    ),
    ("Ag", "Silver"       ),
    ("Cd", "Cadmium"      ),
    ("In", "Indium"       ),
    ("Sn", "Tin"          ),
    ("Sb", "Antimony"     ),
    ("Te", "Tellurium"    ),
    ("I",  "Iodine"       ),
    ("Xe", "Xenon"        ),
    ("Cs", "Caesium"      ),
    ("Ba", "Barium"       ),
    ("La", "Lanthanum"    ),
    ("Ce", "Cerium"       ),
    ("Pr", "Praseodymium" ),
    ("Nd", "Neodymium"    ),
    ("Pm", "Promethium"   ),
    ("Sm", "Samarium"     ),
    ("Eu", "Europium"     ),
    ("Gd", "Gadolinium"   ),
    ("Tb", "Terbium"      ),
    ("Dy", "Dysprosium"   ),
    ("Ho", "Holmium"      ),
    ("Er", "Erbium"       ),
    ("Tm", "Thulium"      ),
    ("Yb", "Ytterbium"    ),
    ("Lu", "Lutetium"     ),
    ("Hf", "Hafnium"      ),
    ("Ta", "Tantalum"     ),
    ("W",  "Tungsten"     ),
    ("Re", "Rhenium"      ),
    ("Os", "Osmium"       ),
    ("Ir", "Iridium"      ),
    ("Pt", "Platinum"     ),
    ("Au", "Gold"         ),
    ("Hg", "Mercury"      ),
    ("Tl", "Thallium"     ),
    ("Pb", "Lead"         ),
    ("Bi", "Bismuth"      ),
    ("Po", "Polonium"     ),
    ("At", "Astatine"     ),
    ("Rn", "Radon"        ),
    ("Fr", "Francium"     ),
    ("Ra", "Radium"       ),
    ("Ac", "Actinium"     ),
    ("Th", "Thorium"      ),
    ("Pa", "Protactinium" ),
    ("U",  "Uranium"      ),
    ("Np", "Neptunium"    ),
    ("Pu", "Plutonium"    ),
    ("Am", "Americium"    ),
    ("Cm", "Curium"       ),
    ("Bk", "Berkelium"    ),
    ("Cf", "Californium"  ),
    ("Es", "Einsteinium"  ),
    ("Fm", "Fermium"      ),
    ("Md", "Mendelevium"  ),
    ("No", "Nobelium"     ),
    ("Lr", "Lawrencium"   ),
    ("Rf", "Rutherfordium"),
    ("Db", "Dubnium"      ),
    ("Sg", "Seaborgium"   ),
    ("Bh", "Bohrium"      ),
    ("Hs", "Hassium"      ),
    ("Mt", "Meitnerium"   ),
    ("Ds", "Darmstadtium" ),
    ("Rg", "Roentgenium"  ),
    ("Cn", "Copernicium"  ),
    ("Nh", "Nihonium"     ),
    ("Fl", "Flerovium"    ),
    ("Mc", "Moscovium"    ),
    ("Lv", "Livermorium"  ),
    ("Ts", "Tennessine"   ),
    ("Og", "Oganesson"    ),
];

impl Element {
    const fn info(self) -> &'static (&'static str, &'static str) {
        // SAFETY: positive NonZero guaranteed not to underflow
        &ELEMENT_INFO[unsafe { self.protons().get().unchecked_sub(1) } as usize]
    }

    pub const fn list() -> &'static [Element; 118] {
        &ELEMENT_LIST
    }

    /// The symbol used to represent this element
    pub const fn symbol(self) -> &'static str {
        self.info().0
    }

    /// The common name of this element
    pub const fn name(self) -> &'static str {
        self.info().1
    }

    /// The number of protons the element has
    ///
    /// A typical atom will also have this many neutrons and electrons
    #[inline]
    pub const fn protons(self) -> NonZeroU8 {
        // SAFETY: No element has 0 protons.
        unsafe { NonZeroU8::new_unchecked(self as u8) }
    }

    /// Atoms that always form pairs with themselves when given the chance
    pub const fn is_diatomic(self) -> bool {
        matches!(self, H | N | O | F | Cl | Br | I)
    }

    /// Elements that don't want to form compounds
    pub const fn is_noble_gas(self) -> bool {
        matches!(self, He | Ne | Ar | Kr | Xe | Rn | Og)
    }

    /// Elements that tend to form cations instead of anions
    #[rustfmt::skip]
    pub const fn is_metal(self) -> bool {
        matches!(self,
            |Li|Be
            |Na|Mg                                                                        |Al
            |K |Ca                                          |Sc|Ti|V |Cr|Mn|Fe|Co|Ni|Cu|Zn|Ga
            |Rb|Sr                                          |Y |Zr|Nb|Mo|Tc|Ru|Rh|Pd|Ag|Cd|In|Sn
            |Cs|Ba|La|Ce|Pr|Nd|Pm|Sm|Eu|Gd|Tb|Dy|Ho|Er|Tm|Yb|Lu|Hf|Ta|W |Re|Os|Ir|Pt|Au|Hg|Tl|Pb|Bi
            |Fr|Ra|Ac|Th|Pa|U |Np|Pu|Am|Cm|Bk|Cf|Es|Fm|Md|No|Lr|Rf|Db|Sg|Bh|Hs|Mt|Ds|Rg|Cn|Nh|Fl|Mc|Lv
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub element: Element,
    pub neutrons: u8,
    pub electrons: u8,
}

impl Default for Atom {
    fn default() -> Self {
        Self {
            element: H,
            neutrons: 0,
            electrons: 1,
        }
    }
}

impl Atom {
    pub const fn neutral(element: Element) -> Self {
        Self {
            element,
            neutrons: element.protons().get(),
            electrons: element.protons().get(),
        }
    }

    /// The name of the isotope
    ///
    /// Returns [`None`] if the isotope has no meaningful name.
    ///
    /// See also: [`Self::systematic_name`].
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Self {
                element: H,
                neutrons: 1,
                electrons: _,
            } => Some("Deuterium"),
            _ => None,
        }
    }

    /// Generate the name of an atom using numbers.
    pub fn systematic_name(self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "{}-{}", self.element.name(), self.neutrons)
    }

    /// Mass of one atom in AMU
    pub const fn mass(self) -> f64 {
        self.element.protons().get() as f64 * PROTON_MASS
            + self.neutrons as f64 * NEUTRON_MASS
            + self.electrons as f64 * ELECTRON_MASS
    }
}

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
        d: &mut DynRaylibDraw3D,
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
            &model.meshes()[0],
            model.materials()[0].clone(),
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

#[const_trait]
pub trait SubSupScript: Sized {
    type Output: Sized;

    /// Convert a unicode character to its superscript equivalent
    ///
    /// Returns [`None`] if the character has no superscript version
    #[must_use]
    fn to_superscript(self) -> Option<Self::Output>;

    /// Convert a unicode character to its subscript equivalent
    ///
    /// Returns [`None`] if the character has no subscript version
    #[must_use]
    fn to_subscript(self) -> Option<Self::Output>;
}

/// Only works for `0`-`9`, `+`, `-`, `=`, `(`, and `)`
impl const SubSupScript for char {
    type Output = char;

    #[inline]
    fn to_superscript(self) -> Option<Self::Output> {
        match self {
            '0' => Some('⁰'),
            '1' => Some('¹'),
            '2' => Some('²'),
            '3' => Some('³'),
            '4' => Some('⁴'),
            '5' => Some('⁵'),
            '6' => Some('⁶'),
            '7' => Some('⁷'),
            '8' => Some('⁸'),
            '9' => Some('⁹'),
            '+' => Some('⁺'),
            '-' => Some('⁻'),
            '=' => Some('⁼'),
            '(' => Some('⁽'),
            ')' => Some('⁾'),
            _ => None,
        }
    }

    #[inline]
    fn to_subscript(self) -> Option<Self::Output> {
        match self {
            '0' => Some('₀'),
            '1' => Some('₁'),
            '2' => Some('₂'),
            '3' => Some('₃'),
            '4' => Some('₄'),
            '5' => Some('₅'),
            '6' => Some('₆'),
            '7' => Some('₇'),
            '8' => Some('₈'),
            '9' => Some('₉'),
            '+' => Some('₊'),
            '-' => Some('₋'),
            '=' => Some('₌'),
            '(' => Some('₍'),
            ')' => Some('₎'),
            _ => None,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<[u8; 2]>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_masses() {
//         const EPSILON: f64 = 0.01e-27;
//         const TESTS: [(Atom, f64); 3] = [
//             (
//                 Atom {
//                     element: H,
//                     neutrons: 0,
//                     electrons: 1,
//                 },
//                 1.008,
//             ),
//             (
//                 Atom {
//                     element: H,
//                     neutrons: 1,
//                     electrons: 1,
//                 },
//                 2.014,
//             ),
//             (
//                 Atom {
//                     element: C,
//                     neutrons: 6,
//                     electrons: 6,
//                 },
//                 12.0107,
//             ),
//         ];
//         for (atom, expect) in TESTS {
//             let actual = atom.mass();
//             assert!(
//                 (actual - expect).abs() <= EPSILON,
//                 "mass of {atom:?}\n expect: {expect}kg\n actual: {actual}kg",
//             );
//         }
//     }

//     #[test]
//     fn test_configuration_fmt() {
//         #[rustfmt::skip]
//         const TESTS: [(Element, &str); 4] = [
//             (He, "1s²"),
//             (B,  "1s² 2s² 2p¹"),
//             (Ne, "1s² 2s² 2p⁶"),
//             (Al, "1s² 2s² 2p⁶ 3s² 3p¹"),
//             // TODO: apparently elections can be "fascinated"??
//             // (U,  "1s² 2s² 2p⁶ 3s² 3p⁶ 4s² 3d¹⁰ 4p⁶ 5s² 4d¹⁰ 5p⁶ 6s² 4f¹⁴ 5d¹⁰ 6p⁶ 7s² 5f³ 6d¹"),
//         ];
//         for (element, expect) in TESTS {
//             let electrons = element.protons().get();
//             let config = ElectronConfig::new(electrons);
//             let actual = config.to_string();
//             assert_eq!(&actual, expect);
//         }
//     }

//     #[test]
//     fn test_configurations() {
//         #[rustfmt::skip]
//         const TESTS: [(u8, ElectronConfig); 13] = [
//             ( 0, ElectronConfig { levels:  0, outermost: 0 }),
//             ( 1, ElectronConfig { levels:  1, outermost: 1 }),
//             ( 2, ElectronConfig { levels:  1, outermost: 2 }),
//             ( 3, ElectronConfig { levels:  2, outermost: 1 }),
//             ( 4, ElectronConfig { levels:  2, outermost: 2 }),
//             ( 5, ElectronConfig { levels:  3, outermost: 1 }),
//             ( 6, ElectronConfig { levels:  3, outermost: 2 }),
//             ( 7, ElectronConfig { levels:  3, outermost: 3 }),
//             ( 8, ElectronConfig { levels:  3, outermost: 4 }),
//             ( 9, ElectronConfig { levels:  3, outermost: 5 }),
//             (10, ElectronConfig { levels:  3, outermost: 6 }),
//             (11, ElectronConfig { levels:  4, outermost: 1 }),
//             (53, ElectronConfig { levels: 11, outermost: 5 }),
//         ];
//         for (electrons, expect) in TESTS {
//             let actual = ElectronConfig::new(electrons);
//             let available = actual.available();
//             println!("{electrons:>2} electrons: {actual} -- {available} available");
//             assert_eq!(actual, expect);
//         }
//     }

//     #[test]
//     fn test_availability() {
//         const TESTS: [(Element, u8); 8] = [
//             (H, 1),
//             (He, 0),
//             (O, 2),
//             (N, 3),
//             (C, 4),
//             (F, 1),
//             (Fe, 4),
//             (U, 10),
//         ];
//         for (element, expect) in TESTS {
//             let electrons = element.protons().get();
//             let config = ElectronConfig::new(electrons);
//             let actual = config.available();
//             println!(
//                 "neutral {element} ({electrons} electrons -- {config}) can form {actual} bonds"
//             );
//             assert_eq!(actual, expect);
//         }
//     }
// }
