#[allow(
    clippy::enum_glob_use,
    reason = "I am importing all of them and don't want to repeat all 118 names. They don't shadow anything else here."
)]
use Element::*;
use std::num::{NonZeroI8, NonZeroU8};

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

    /// The number of protons the element has
    ///
    /// A typical atom will also have this many neutrons and electrons
    #[inline]
    pub const fn protons_signed(self) -> NonZeroI8 {
        debug_assert!((self as u8) <= i8::MAX as u8);
        // SAFETY: No element has 0 protons.
        unsafe { NonZeroI8::new_unchecked(self as i8) }
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
