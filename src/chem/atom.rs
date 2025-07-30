use super::element::Element;
use super::units::{ELECTRON_MASS, NEUTRON_MASS, PROTON_MASS};
use crate::chem::fmt::Superscript;
use crate::resource::Resources;
use arrayvec::ArrayVec;
use raylib::prelude::*;

macro_rules! isotopes {
    ($($element:ident $neutrons:literal),* $(,)?) => {
        [$(Atom { element: Element::$element, neutrons: $neutrons, electrons: const { Element::$element.protons().get() } }),*]
    };
}

pub static PRIMORDIAL_ISOTOPES: [Atom; 304] = isotopes![
    Sn 120,
    Sn 118,
    Sn 116,
    Sn 119,
    Sn 117,
    Sn 124,
    Sn 122,
    Sn 112,
    Sn 114,
    Sn 115,
    Xe 132,
    Xe 129,
    Xe 131,
    Xe 134,
    Xe 136, // unstable
    Xe 130,
    Xe 128,
    Xe 124, // unstable
    Xe 126,
    Cd 114,
    Cd 112,
    Cd 111,
    Cd 110,
    Cd 113, // unstable
    Cd 116, // unstable
    Cd 106,
    Cd 108,
    Te 130, // unstable
    Te 128, // unstable
    Te 126,
    Te 125,
    Te 124,
    Te 122,
    Te 123,
    Te 120,
    Ru 102,
    Ru 104,
    Ru 101,
    Ru 99,
    Ru 100,
    Ru 96,
    Ru 98,
    Dy 164,
    Dy 162,
    Dy 163,
    Dy 161,
    Dy 160,
    Dy 158,
    Dy 156,
    Yb 174,
    Yb 172,
    Yb 173,
    Yb 171,
    Yb 176,
    Yb 170,
    Yb 168,
    Hg 202,
    Hg 200,
    Hg 199,
    Hg 201,
    Hg 198,
    Hg 204,
    Hg 196,
    Mo 98,
    Mo 96,
    Mo 95,
    Mo 92,
    Mo 100, // unstable
    Mo 97,
    Mo 94,
    Ba 138,
    Ba 137,
    Ba 136,
    Ba 135,
    Ba 134,
    Ba 132,
    Ba 130, // unstable
    Gd 158,
    Gd 160,
    Gd 156,
    Gd 157,
    Gd 155,
    Gd 154,
    Gd 152, // unstable
    Nd 142,
    Nd 144, // unstable
    Nd 146,
    Nd 143,
    Nd 145,
    Nd 148,
    Nd 150, // unstable
    Sm 152,
    Sm 154,
    Sm 147, // unstable
    Sm 149,
    Sm 148, // unstable
    Sm 150,
    Sm 144,
    Os 192,
    Os 190,
    Os 189,
    Os 188,
    Os 187,
    Os 186, // unstable
    Os 184, // unstable
    Pd 106,
    Pd 108,
    Pd 105,
    Pd 110,
    Pd 104,
    Pd 102,
    Er 166,
    Er 168,
    Er 167,
    Er 170,
    Er 164,
    Er 162,
    Ca 40,
    Ca 44,
    Ca 42,
    Ca 43,
    Ca 46,
    Se 80,
    Se 78,
    Se 76,
    Se 82, // unstable
    Se 77,
    Se 74,
    Kr 84,
    Kr 86,
    Kr 82,
    Kr 83,
    Kr 80,
    Kr 78, // unstable
    Hf 180,
    Hf 178,
    Hf 177,
    Hf 179,
    Hf 176,
    Hf 174, // unstable
    Pt 195,
    Pt 194,
    Pt 196,
    Pt 198,
    Pt 192,
    Pt 190, // unstable
    Ti 48,
    Ti 46,
    Ti 47,
    Ti 49,
    Ti 50,
    Ni 58,
    Ni 60,
    Ni 61,
    Ni 64,
    Zn 64,
    Zn 66,
    Zn 68,
    Zn 67,
    Zn 70,
    Ge 74,
    Ge 72,
    Ge 70,
    Ge 73,
    Ge 76, // unstable
    Zr 90,
    Zr 94,
    Zr 92,
    Zr 91,
    Zr 96, // unstable
    W 184,
    W 186,
    W 182,
    W 183,
    W 180, // unstable
    S 32,
    S 34,
    S 33,
    S 36,
    Cr 52,
    Cr 53,
    Cr 50,
    Cr 54,
    Fe 54,
    Fe 57,
    Fe 58,
    Sr 88,
    Sr 86,
    Sr 87,
    Sr 84,
    Ce 140,
    Ce 142,
    Ce 138,
    Ce 136,
    Pb 208,
    Pb 206,
    Pb 207,
    Pb 204,
    O 16,
    O 17,
    Ne 20,
    Ne 22,
    Ne 21,
    Mg 24,
    Mg 26,
    Mg 25,
    Si 28,
    Si 29,
    Si 30,
    Ar 40,
    Ar 36,
    Ar 38,
    Be 9,
    K 39,
    K 41,
    Li 7,
    Li 6,
    B 11,
    B 10,
    N 14,
    N 15,
    Cl 35,
    Cu 63,
    Cu 65,
    Ga 69,
    Ga 71,
    Br 79,
    Br 81,
    Ag 107,
    Ag 109,
    Sb 121,
    Sb 123,
    Ta 181,
    Ir 193,
    Ir 191,
    Tl 205,
    Tl 203,
    V 51,
    V 50, // unstable
    Rb 85,
    Rb 87, // unstable
    In 115, // unstable
    In 113,
    La 139,
    La 138, // unstable
    Eu 153,
    Eu 151, // unstable
    Lu 175,
    Lu 176, // unstable
    Re 187, // unstable
    Re 185,
    F 19,
    Na 23,
    Al 27,
    P 31,
    Sc 45,
    Mn 55,
    Co 59,
    As 75,
    Y 89,
    Nb 93,
    Rh 103,
    I 127,
    Cs 133,
    Pr 141,
    Tb 159,
    Ho 165,
    Tm 169,
    Au 197,
    Pu 244,
    Cm 247,
    Tc 97,
    Np 237,
    Pa 231,
    Am 243,
    Ra 226,
    Bk 247,
    Cf 251,
    Po 209,
    Ac 227,
    Pm 145,
    Es 252,
    Fm 257,
    Md 258,
    Rn 222,
    Db 268,
    Lr 266,
    At 210,
    No 259,
    Rf 267,
    Fr 223,
    Sg 269,
    Bh 270,
    Rg 282,
    Cn 285,
    Hs 269,
    Ds 281,
    Nh 286,
    Mt 278,
    Fl 289,
    Mc 290,
    Lv 293,
    Ts 294,
    Og 294,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub element: Element,
    pub neutrons: u16,
    pub electrons: u8,
}

impl Default for Atom {
    fn default() -> Self {
        Self {
            element: Element::H,
            neutrons: 0,
            electrons: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtomBuilder {
    atom: Atom,
}

impl AtomBuilder {
    pub const fn new(element: Element) -> Self {
        Self {
            atom: Atom {
                element,
                neutrons: 0,
                electrons: 0,
            },
        }
    }

    pub const fn neutral(&mut self) -> &mut Self {
        self.atom.electrons = self.atom.element.protons().get();
        self
    }

    pub const fn charge(&mut self, charge: i8) -> Result<&mut Self, &mut Self> {
        const U8_MIN: i16 = u8::MIN as i16;
        const U8_MAX: i16 = u8::MAX as i16;
        match self.atom.element.protons().get() as i16 - charge as i16 {
            electrons @ U8_MIN..=U8_MAX =>
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "cannot in this branch"
            )]
            {
                self.atom.electrons = electrons as u8;
                Ok(self)
            }
            _ => Err(self),
        }
    }

    pub const fn electrons(&mut self, electrons: u8) -> &mut Self {
        self.atom.electrons = electrons;
        self
    }

    pub const fn stable(&mut self) -> &mut Self {
        // TODO: This is not necessarily stable...
        self.atom.neutrons = self.atom.element.protons().get() as u16;
        self
    }

    pub const fn neutrons(&mut self, neutrons: u16) -> &mut Self {
        self.atom.neutrons = neutrons;
        self
    }

    pub const fn build(&mut self) -> Atom {
        self.atom
    }
}

impl Element {
    #[inline]
    pub const fn atom(self) -> AtomBuilder {
        AtomBuilder::new(self)
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
                element: Element::H,
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

    /// Charge of the atom
    /// - negative = more electrons
    /// - positive = fewer electrons
    pub const fn charge(self) -> i16 {
        self.element.protons().get() as i16 - self.electrons as i16
    }

    pub fn draw(self, d: &mut impl RaylibDraw3D, position: Vector3, scale: f32) {
        const GOLDEN_ANGLE: f32 = 2.0 * std::f32::consts::PI / std::f32::consts::PHI;

        let mut is_proton = false;
        let mut protons = u16::from(self.element.protons().get());
        let mut neutrons = self.neutrons;
        let nucleons = protons + neutrons;

        let rise = if nucleons > 1 {
            -2.0 / f32::from(nucleons - 1)
        } else {
            0.0
        };

        let mut points = ArrayVec::<(Vector3, Color), 255>::new();

        for i in 0..nucleons {
            is_proton = match (protons, neutrons) {
                (0, _) => false,
                (_, 0) => true,
                (_, _) => !is_proton,
            };

            let offset = if nucleons > 1 {
                let y = rise * f32::from(i) + 1.0; // y goes from 1 to -1
                let radius = (1.0 - y * y).sqrt(); // radius at y
                let theta = GOLDEN_ANGLE * f32::from(i); // golden angle increment
                let (z, x) = theta.sin_cos();
                Vector3::new(x * radius, y, z * radius)
            } else {
                Vector3::ZERO
            };

            points.push((offset, if is_proton { Color::RED } else { Color::GRAY }));

            *if is_proton {
                &mut protons
            } else {
                &mut neutrons
            } -= 1;
        }

        let mut min_distance = f32::MAX;
        for (p1, _) in &points {
            for (p2, _) in
        }

        for (offset, color) in points {
            d.draw_sphere(position + offset * scale * 2.0, scale, color);
        }

        // todo
        for i in 0..self.electrons {
            d.draw_sphere(
                position + Vector3::new(f32::from(i) * scale * 2.0, 0.0, scale * 4.0),
                scale * 0.5,
                Color::DODGERBLUE,
            );
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element = self.element;
        let charge = self.charge();
        let (sign, mag) = (
            Superscript(char::from(b"+-"[usize::from(charge.is_negative())])),
            Superscript(charge.unsigned_abs()),
        );
        match mag.0 {
            0 => write!(f, "{element}"),
            1 => write!(f, "{element}{sign}"),
            2.. => write!(f, "{element}{mag}{sign}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        use Element::He;

        assert_eq!(He.atom().charge(0).unwrap().build().to_string(), "He");
        assert_eq!(He.atom().charge(-1).unwrap().build().to_string(), "He⁻");
        assert_eq!(He.atom().charge(-2).unwrap().build().to_string(), "He²⁻");
        assert_eq!(He.atom().charge(1).unwrap().build().to_string(), "He⁺");
        assert_eq!(He.atom().charge(2).unwrap().build().to_string(), "He²⁺");
    }
}
