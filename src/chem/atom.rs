use crate::chem::fmt::{SubSupScript, Superscript};

use super::element::Element;
use super::units::{ELECTRON_MASS, NEUTRON_MASS, PROTON_MASS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub element: Element,
    pub neutrons: u8,
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

    pub const fn charge(self) -> i16 {
        self.element.protons().get() as i16 - self.electrons as i16
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)?;
        let charge = self.charge();
        let mag = charge.unsigned_abs();
        if mag > 1 {
            Superscript::fmt(&mag, f)?;
        }
        if mag > 0 {
            let sign = if charge.is_negative() { '-' } else { '+' };
            Superscript::fmt(&sign, f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(
            &Atom {
                element: Element::He,
                neutrons: 2,
                electrons: 2,
            }
            .to_string(),
            "He"
        );
        assert_eq!(
            &Atom {
                element: Element::He,
                neutrons: 2,
                electrons: 3,
            }
            .to_string(),
            "He⁻"
        );
        assert_eq!(
            &Atom {
                element: Element::He,
                neutrons: 2,
                electrons: 4,
            }
            .to_string(),
            "He²⁻"
        );
        assert_eq!(
            &Atom {
                element: Element::He,
                neutrons: 2,
                electrons: 1,
            }
            .to_string(),
            "He⁺"
        );
        assert_eq!(
            &Atom {
                element: Element::He,
                neutrons: 2,
                electrons: 0,
            }
            .to_string(),
            "He²⁺"
        );
    }
}
