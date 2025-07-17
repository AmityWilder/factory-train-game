use arrayvec::ArrayVec;
use std::num::NonZeroU8;

pub const PROTON_MASS: f64 = 1.672_621_925_955_2e-27;
pub const NEUTRON_MASS: f64 = 1.674_927_500_568_5e-27;
pub const ELECTRON_MASS: f64 = 9.109_383_713_928e-31;

// S: Spherical
// P: Dumbell
// D: Clover
// F: 8 knotted balloons

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
    pub const fn as_element(self) -> Element {
        // SAFETY: NobleGas is a subset of Element
        unsafe { std::mem::transmute(self) }
    }
}

impl std::fmt::Display for Element {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.symbol().fmt(f)
    }
}

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

    /// Mass in kg of one atom
    pub const fn mass(self) -> f64 {
        self.element.protons().get() as f64 * PROTON_MASS
            + self.neutrons as f64 * NEUTRON_MASS
            + self.electrons as f64 * ELECTRON_MASS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum SubLevel {
    _1S = 1 << 2,
    _2S = 2 << 2, _2P = (2 << 2) | 1,
    _3S = 3 << 2, _3P = (3 << 2) | 1, _3D = (3 << 2) | 2,
    _4S = 4 << 2, _4P = (4 << 2) | 1, _4D = (4 << 2) | 2, _4F = (4 << 2) | 3,
    _5S = 5 << 2, _5P = (5 << 2) | 1, _5D = (5 << 2) | 2, _5F = (5 << 2) | 3,
    _6S = 6 << 2, _6P = (6 << 2) | 1, _6D = (6 << 2) | 2,
    _7S = 7 << 2, _7P = (7 << 2) | 1,
}
#[allow(clippy::enum_glob_use)]
use SubLevel::*;

impl std::fmt::Display for SubLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.energy_level(), self.symbol())
    }
}

#[rustfmt::skip]
static ORBITALS: [SubLevel; 19] = [
    _1S,
    _2S,           _2P,
    _3S,           _3P,
    _4S,      _3D, _4P,
    _5S,      _4D, _5P,
    _6S, _4F, _5D, _6P,
    _7S, _5F, _6D, _7P,
];

impl SubLevel {
    const fn index(self) -> u8 {
        self as u8 & 3
    }

    pub const fn energy_level(self) -> u8 {
        self as u8 >> 2
    }

    pub const fn symbol(self) -> char {
        b"spdf"[self.index() as usize] as char
    }

    pub const fn orbitals(self) -> NonZeroU8 {
        let n = self.index();
        // SAFETY: Highest index is 6, which can be shl'd to 12 without overflowing.
        let n = unsafe { n.unchecked_shl(1) };
        // SAFETY: Highest valid is 12, which can be incremented to 13 without overflowing.
        let n = unsafe { n.unchecked_add(1) };
        // SAFETY: Adding 1 guarantees non-zero.
        unsafe { NonZeroU8::new_unchecked(n) }
    }

    pub const fn capacity(self) -> NonZeroU8 {
        // SAFETY: The highest orbital is `I` with 13.
        // 13 << 1 = 26, which does not overflow.
        let n = unsafe { self.orbitals().get().unchecked_shl(1) };
        // SAFETY: nonzero multiplied by nonzero is nonzero, given no overflow
        unsafe { NonZeroU8::new_unchecked(n) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElectronConfig {
    levels: u8,
    outermost: u8,
}

impl ElectronConfig {
    pub const fn new(mut electrons: u8) -> ElectronConfig {
        let mut i = 0;
        loop {
            let cap = ORBITALS[i as usize].capacity().get();
            if electrons > cap {
                electrons -= cap;
                i += 1;
                assert!((i as usize) < ORBITALS.len(), "too many electrons");
            } else {
                break ElectronConfig {
                    levels: i + (electrons > 0) as u8,
                    outermost: electrons,
                };
            }
        }
    }

    pub const fn sublevels(self) -> &'static [SubLevel] {
        ORBITALS.split_at(self.levels as usize).0
    }

    /// Returns [`None`] if there are no shells
    pub const fn valance_capacity(self) -> Option<NonZeroU8> {
        if let Some(valance) = self.sublevels().last() {
            Some(valance.capacity())
        } else {
            None
        }
    }

    pub const fn valance_electrons(self) -> u8 {
        self.outermost
    }

    /// Valance shell capacity - valance electrons
    pub const fn available(self) -> u8 {
        let capacity = match self.valance_capacity() {
            Some(n) => n.get(),
            None => 0,
        };
        let electrons = self.valance_electrons();
        assert!(
            electrons <= capacity,
            "number of electrons in a given shell cannot exceed that shell's capacity"
        );
        capacity - electrons
    }
}

impl std::fmt::Display for ElectronConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn superscript(buf: &mut (String, String), n: u8) -> std::fmt::Result {
            use std::fmt::Write;
            buf.0.clear();
            buf.1.clear();
            write!(buf.0, "{n}")?;
            for ch in buf.0.chars().map(|ch| match ch {
                '0' => '⁰',
                '1' => '¹',
                '2' => '²',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                _ => unreachable!(),
            }) {
                buf.1.push(ch);
            }
            Ok(())
        }
        let mut sublevels = self.sublevels().to_vec();
        sublevels.sort_by_key(|lv| lv.energy_level());
        let mut it = sublevels.iter();
        let mut buf = (String::new(), String::new());
        for orbital in it.by_ref().take(sublevels.len().saturating_sub(1)) {
            superscript(&mut buf, orbital.capacity().get())?;
            write!(f, "{}{} ", orbital, buf.1)?;
        }
        if let Some(orbital) = it.next() {
            superscript(&mut buf, self.outermost)?;
            write!(f, "{}{}", orbital, buf.1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ion {
    pub element: Element,
    pub count: NonZeroU8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    Solid,
    Aqueous,
    Liquid,
    Gas,
}

pub struct Compound {
    cation: ArrayVec<Element, 8>,
    anion: ArrayVec<Element, 8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masses() {
        const EPSILON: f64 = 0.01e-27;
        let expect = 1.67e-27;
        let actual = Atom {
            element: H,
            neutrons: 0,
            electrons: 1,
        }
        .mass();
        assert!(
            (actual - expect).abs() <= EPSILON,
            "mass of hydrogen\n expect: {expect}kg\n actual: {actual}kg"
        );
        let expect = 3.344_476_425e-27;
        let actual = Atom {
            element: H,
            neutrons: 1,
            electrons: 1,
        }
        .mass();
        assert!(
            (actual - expect).abs() <= EPSILON,
            "mass of deuterium\n expect: {expect}kg\n actual: {actual}kg"
        );
    }

    #[test]
    fn test_configurations() {
        #[rustfmt::skip]
        const TESTS: [(u8, ElectronConfig); 13] = [
            ( 0, ElectronConfig { levels:  0, outermost: 0 }),
            ( 1, ElectronConfig { levels:  1, outermost: 1 }),
            ( 2, ElectronConfig { levels:  1, outermost: 2 }),
            ( 3, ElectronConfig { levels:  2, outermost: 1 }),
            ( 4, ElectronConfig { levels:  2, outermost: 2 }),
            ( 5, ElectronConfig { levels:  3, outermost: 1 }),
            ( 6, ElectronConfig { levels:  3, outermost: 2 }),
            ( 7, ElectronConfig { levels:  3, outermost: 3 }),
            ( 8, ElectronConfig { levels:  3, outermost: 4 }),
            ( 9, ElectronConfig { levels:  3, outermost: 5 }),
            (10, ElectronConfig { levels:  3, outermost: 6 }),
            (11, ElectronConfig { levels:  4, outermost: 1 }),
            (53, ElectronConfig { levels: 11, outermost: 5 }),
        ];
        for (electrons, expect) in TESTS {
            let actual = ElectronConfig::new(electrons);
            let available = actual.available();
            println!("{electrons:>2} electrons: {actual} -- {available} available");
            assert_eq!(actual, expect);
        }
    }
}
