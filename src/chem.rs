use std::num::NonZeroU8;
use arrayvec::ArrayVec;

pub const PROTON_MASS: f64 = 1.6726219259552e-27;
pub const NEUTRON_MASS: f64 = 1.6749275005685e-27;
pub const ELECTRON_MASS: f64 = 9.109383713928e-31;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Element {
    #[default]
    H = 1,                                                                                                                      He,
    Li, Be,                                                                                                 B,  C,  N,  O,  F,  Ne,
    Na, Mg,                                                                                                 Al, Si, P,  S,  Cl, Ar,
    K,  Ca, Sc,                                                         Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y,                                                          Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I,  Xe,
    Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra, Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
}
use Element::*;

impl std::fmt::Display for Element {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.symbol().fmt(f)
    }
}

impl Element {
    pub const fn symbol(self) -> &'static str {
        match self {
            H  => "H",
            He => "He",
            Li => "Li",
            Be => "Be",
            B  => "B",
            C  => "C",
            N  => "N",
            O  => "O",
            F  => "F",
            Ne => "Ne",
            Na => "Na",
            Mg => "Mg",
            Al => "Al",
            Si => "Si",
            P  => "P",
            S  => "S",
            Cl => "Cl",
            Ar => "Ar",
            K  => "K",
            Ca => "Ca",
            Sc => "Sc",
            Ti => "Ti",
            V  => "V",
            Cr => "Cr",
            Mn => "Mn",
            Fe => "Fe",
            Co => "Co",
            Ni => "Ni",
            Cu => "Cu",
            Zn => "Zn",
            Ga => "Ga",
            Ge => "Ge",
            As => "As",
            Se => "Se",
            Br => "Br",
            Kr => "Kr",
            Rb => "Rb",
            Sr => "Sr",
            Y  => "Y",
            Zr => "Zr",
            Nb => "Nb",
            Mo => "Mo",
            Tc => "Tc",
            Ru => "Ru",
            Rh => "Rh",
            Pd => "Pd",
            Ag => "Ag",
            Cd => "Cd",
            In => "In",
            Sn => "Sn",
            Sb => "Sb",
            Te => "Te",
            I  => "I",
            Xe => "Xe",
            Cs => "Cs",
            Ba => "Ba",
            La => "La",
            Ce => "Ce",
            Pr => "Pr",
            Nd => "Nd",
            Pm => "Pm",
            Sm => "Sm",
            Eu => "Eu",
            Gd => "Gd",
            Tb => "Tb",
            Dy => "Dy",
            Ho => "Ho",
            Er => "Er",
            Tm => "Tm",
            Yb => "Yb",
            Lu => "Lu",
            Hf => "Hf",
            Ta => "Ta",
            W  => "W",
            Re => "Re",
            Os => "Os",
            Ir => "Ir",
            Pt => "Pt",
            Au => "Au",
            Hg => "Hg",
            Tl => "Tl",
            Pb => "Pb",
            Bi => "Bi",
            Po => "Po",
            At => "At",
            Rn => "Rn",
            Fr => "Fr",
            Ra => "Ra",
            Ac => "Ac",
            Th => "Th",
            Pa => "Pa",
            U  => "U",
            Np => "Np",
            Pu => "Pu",
            Am => "Am",
            Cm => "Cm",
            Bk => "Bk",
            Cf => "Cf",
            Es => "Es",
            Fm => "Fm",
            Md => "Md",
            No => "No",
            Lr => "Lr",
            Rf => "Rf",
            Db => "Db",
            Sg => "Sg",
            Bh => "Bh",
            Hs => "Hs",
            Mt => "Mt",
            Ds => "Ds",
            Rg => "Rg",
            Cn => "Cn",
            Nh => "Nh",
            Fl => "Fl",
            Mc => "Mc",
            Lv => "Lv",
            Ts => "Ts",
            Og => "Og",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            H  => "Hydrogen",
            He => "Helium",
            Li => "Lithium",
            Be => "Beryllium",
            B  => "Boron",
            C  => "Carbon",
            N  => "Nitrogen",
            O  => "Oxygen",
            F  => "Fluorine",
            Ne => "Neon",
            Na => "Sodium",
            Mg => "Magnesium",
            Al => "Aluminium",
            Si => "Silicon",
            P  => "Phosphorus",
            S  => "Sulfur",
            Cl => "Chlorine",
            Ar => "Argon",
            K  => "Potassium",
            Ca => "Calcium",
            Sc => "Scandium",
            Ti => "Titanium",
            V  => "Vanadium",
            Cr => "Chromium",
            Mn => "Manganese",
            Fe => "Iron",
            Co => "Cobalt",
            Ni => "Nickel",
            Cu => "Copper",
            Zn => "Zinc",
            Ga => "Gallium",
            Ge => "Germanium",
            As => "Arsenic",
            Se => "Selenium",
            Br => "Bromine",
            Kr => "Krypton",
            Rb => "Rubidium",
            Sr => "Strontium",
            Y  => "Yttrium",
            Zr => "Zirconium",
            Nb => "Niobium",
            Mo => "Molybdenum",
            Tc => "Technetium",
            Ru => "Ruthenium",
            Rh => "Rhodium",
            Pd => "Palladium",
            Ag => "Silver",
            Cd => "Cadmium",
            In => "Indium",
            Sn => "Tin",
            Sb => "Antimony",
            Te => "Tellurium",
            I  => "Iodine",
            Xe => "Xenon",
            Cs => "Caesium",
            Ba => "Barium",
            La => "Lanthanum",
            Ce => "Cerium",
            Pr => "Praseodymium",
            Nd => "Neodymium",
            Pm => "Promethium",
            Sm => "Samarium",
            Eu => "Europium",
            Gd => "Gadolinium",
            Tb => "Terbium",
            Dy => "Dysprosium",
            Ho => "Holmium",
            Er => "Erbium",
            Tm => "Thulium",
            Yb => "Ytterbium",
            Lu => "Lutetium",
            Hf => "Hafnium",
            Ta => "Tantalum",
            W  => "Tungsten",
            Re => "Rhenium",
            Os => "Osmium",
            Ir => "Iridium",
            Pt => "Platinum",
            Au => "Gold",
            Hg => "Mercury",
            Tl => "Thallium",
            Pb => "Lead",
            Bi => "Bismuth",
            Po => "Polonium",
            At => "Astatine",
            Rn => "Radon",
            Fr => "Francium",
            Ra => "Radium",
            Ac => "Actinium",
            Th => "Thorium",
            Pa => "Protactinium",
            U  => "Uranium",
            Np => "Neptunium",
            Pu => "Plutonium",
            Am => "Americium",
            Cm => "Curium",
            Bk => "Berkelium",
            Cf => "Californium",
            Es => "Einsteinium",
            Fm => "Fermium",
            Md => "Mendelevium",
            No => "Nobelium",
            Lr => "Lawrencium",
            Rf => "Rutherfordium",
            Db => "Dubnium",
            Sg => "Seaborgium",
            Bh => "Bohrium",
            Hs => "Hassium",
            Mt => "Meitnerium",
            Ds => "Darmstadtium",
            Rg => "Roentgenium",
            Cn => "Copernicium",
            Nh => "Nihonium",
            Fl => "Flerovium",
            Mc => "Moscovium",
            Lv => "Livermorium",
            Ts => "Tennessine",
            Og => "Oganesson",
        }
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
        matches!(self,
            | H

            | N | O | F
            |         Cl
            |         Br
            |         I
        )
    }

    /// Elements in the leftmost pair of columns on the periodic table
    pub const fn is_main_group(self) -> bool {
        matches!(self,
            | H
            | Li | Be
            | Na | Mg
            | K  | Ca
            | Rb | Sr
            | Cs | Ba
            | Fr | Ra
        )
    }

    /// Elements that form cations
    pub const fn is_metal(self) -> bool {
        matches!(self,
            | Li | Be
            | Na | Mg                                                                                                                         | Al
            | K  | Ca | Sc                                                                       | Ti | V  | Cr | Mn | Fe | Co | Ni | Cu | Zn | Ga
            | Rb | Sr | Y                                                                        | Zr | Nb | Mo | Tc | Ru | Rh | Pd | Ag | Cd | In | Sn
            | Cs | Ba | La | Ce | Pr | Nd | Pm | Sm | Eu | Gd | Tb | Dy | Ho | Er | Tm | Yb | Lu | Hf | Ta | W  | Re | Os | Ir | Pt | Au | Hg | Tl | Pb | Bi | Po
            | Fr | Ra | Ac | Th | Pa | U  | Np | Pu | Am | Cm | Bk | Cf | Es | Fm | Md | No | Lr | Rf | Db | Sg | Bh | Hs | Mt | Ds | Rg | Cn | Nh | Fl | Mc | Lv
        )
    }

    /// Elements capable of multiple charged states
    pub const fn is_transition_metal(self) -> bool {
        matches!(self,
            | Ti | V  | Cr | Mn | Fe | Co | Ni | Cu | Zn
            | Zr | Nb | Mo | Tc | Ru | Rh | Pd | Ag | Cd
            | Hf | Ta | W  | Re | Os | Ir | Pt | Au | Hg
            | Rf | Db | Sg | Bh | Hs | Mt | Ds | Rg | Cn
        )
    }

    /// Elements that don't want to form compounds
    pub const fn is_noble_gas(self) -> bool {
        matches!(self,
            | He
            | Ne
            | Ar
            | Kr
            | Xe
            | Rn
            | Og
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
    /// Mass in kg of one atom
    pub const fn mass(self) -> f64 {
        self.element.protons().get() as f64 * PROTON_MASS +
        self.neutrons as f64 * NEUTRON_MASS +
        self.electrons as f64 * ELECTRON_MASS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masses() {
        const EPSILON: f64 = 0.01e-27;
        let expect = 1.67e-27;
        let actual = Atom { element: H, neutrons: 0, electrons: 1 }.mass();
        assert!((actual - expect).abs() <= EPSILON, "mass of hydrogen\n expect: {expect}kg\n actual: {actual}kg");
        let expect = 3.344476425e-27;
        let actual = Atom { element: H, neutrons: 1, electrons: 1 }.mass();
        assert!((actual - expect).abs() <= EPSILON, "mass of deuterium\n expect: {expect}kg\n actual: {actual}kg");
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
