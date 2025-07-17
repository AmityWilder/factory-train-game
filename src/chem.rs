use arrayvec::ArrayVec;
use std::num::NonZeroU8;

pub const PROTON_MASS: f64 = 1.672_621_925_955_2e-27;
pub const NEUTRON_MASS: f64 = 1.674_927_500_568_5e-27;
pub const ELECTRON_MASS: f64 = 9.109_383_713_928e-31;

// S: Spherical
// P: Dumbell
// D: Clover
// F: 8 knotted balloons

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Element {
    H = 1,                                                                                                                      He,
    Li, Be,                                                                                                 B,  C,  N,  O,  F,  Ne,
    Na, Mg,                                                                                                 Al, Si, P,  S,  Cl, Ar,
    K,  Ca, Sc,                                                         Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y,                                                          Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I,  Xe,
    Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra, Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
}
#[allow(
    clippy::enum_glob_use,
    reason = "I am importing all of them and don't want to repeat all 118 names. They don't shadow anything else here."
)]
use Element::*;

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
        let expect = 3.344476425e-27;
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
