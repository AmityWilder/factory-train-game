use arrayvec::ArrayString;

pub enum MathSymbol {
    UpperAlpha,
    LowerAlpha,
    UpperBeta,
    LowerBeta,
    UpperGamma,
    LowerGamma,
    UpperDelta,
    LowerDelta,
    UpperEpsilon,
    LowerEpsilon,
    UpperZeta,
    LowerZeta,
    UpperEta,
    LowerEta,
    UpperTheta,
    LowerTheta,
    UpperIota,
    LowerIota,
    UpperKappa,
    LowerKappa,
    UpperLambda,
    LowerLambda,
    UpperMu,
    LowerMu,
    UpperNu,
    LowerNu,
    UpperXi,
    LowerXi,
    UpperOmicron,
    LowerOmicron,
    UpperPi,
    LowerPi,
    UpperRho,
    LowerRho,
    UpperSigma,
    LowerSigma,
    FinalSigma,
    UpperTau,
    LowerTau,
    UpperUpsilon,
    LowerUpsilon,
    UpperPhi,
    LowerPhi,
    UpperChi,
    LowerChi,
    UpperPsi,
    LowerPsi,
    UpperOmega,
    LowerOmega,
}

impl std::fmt::Display for MathSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(
            &match self {
                Self::UpperAlpha => 'Α',
                Self::LowerAlpha => 'α',
                Self::UpperBeta => 'Β',
                Self::LowerBeta => 'β',
                Self::UpperGamma => 'Γ',
                Self::LowerGamma => 'γ',
                Self::UpperDelta => 'Δ',
                Self::LowerDelta => 'δ',
                Self::UpperEpsilon => 'Ε',
                Self::LowerEpsilon => 'ε',
                Self::UpperZeta => 'Ζ',
                Self::LowerZeta => 'ζ',
                Self::UpperEta => 'Η',
                Self::LowerEta => 'η',
                Self::UpperTheta => 'Θ',
                Self::LowerTheta => 'θ',
                Self::UpperIota => 'Ι',
                Self::LowerIota => 'ι',
                Self::UpperKappa => 'Κ',
                Self::LowerKappa => 'κ',
                Self::UpperLambda => 'Λ',
                Self::LowerLambda => 'λ',
                Self::UpperMu => 'Μ',
                Self::LowerMu => 'μ',
                Self::UpperNu => 'Ν',
                Self::LowerNu => 'ν',
                Self::UpperXi => 'Ξ',
                Self::LowerXi => 'ξ',
                Self::UpperOmicron => 'Ο',
                Self::LowerOmicron => 'ο',
                Self::UpperPi => 'Π',
                Self::LowerPi => 'π',
                Self::UpperRho => 'Ρ',
                Self::LowerRho => 'ρ',
                Self::UpperSigma => 'Σ',
                Self::LowerSigma => 'σ',
                Self::FinalSigma => 'ς',
                Self::UpperTau => 'Τ',
                Self::LowerTau => 'τ',
                Self::UpperUpsilon => 'Υ',
                Self::LowerUpsilon => 'υ',
                Self::UpperPhi => 'Φ',
                Self::LowerPhi => 'φ',
                Self::UpperChi => 'Χ',
                Self::LowerChi => 'χ',
                Self::UpperPsi => 'Ψ',
                Self::LowerPsi => 'ψ',
                Self::UpperOmega => 'Ω',
                Self::LowerOmega => 'ω',
            },
            f,
        )
    }
}

pub const SUP_0: char = '⁰';
pub const SUP_1: char = '¹';
pub const SUP_2: char = '²';
pub const SUP_3: char = '³';
pub const SUP_4: char = '⁴';
pub const SUP_5: char = '⁵';
pub const SUP_6: char = '⁶';
pub const SUP_7: char = '⁷';
pub const SUP_8: char = '⁸';
pub const SUP_9: char = '⁹';
pub const SUP_PLUS: char = '⁺';
pub const SUP_HYPHEN: char = '⁻';
pub const SUP_EQUAL: char = '⁼';
pub const SUP_LPAREN: char = '⁽';
pub const SUP_RPAREN: char = '⁾';

pub const SUB_0: char = '₀';
pub const SUB_1: char = '₁';
pub const SUB_2: char = '₂';
pub const SUB_3: char = '₃';
pub const SUB_4: char = '₄';
pub const SUB_5: char = '₅';
pub const SUB_6: char = '₆';
pub const SUB_7: char = '₇';
pub const SUB_8: char = '₈';
pub const SUB_9: char = '₉';
pub const SUB_PLUS: char = '₊';
pub const SUB_HYPHEN: char = '₋';
pub const SUB_EQUAL: char = '₌';
pub const SUB_LPAREN: char = '₍';
pub const SUB_RPAREN: char = '₎';

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
            '0' => Some(SUP_0),
            '1' => Some(SUP_1),
            '2' => Some(SUP_2),
            '3' => Some(SUP_3),
            '4' => Some(SUP_4),
            '5' => Some(SUP_5),
            '6' => Some(SUP_6),
            '7' => Some(SUP_7),
            '8' => Some(SUP_8),
            '9' => Some(SUP_9),
            '+' => Some(SUP_PLUS),
            '-' => Some(SUP_HYPHEN),
            '=' => Some(SUP_EQUAL),
            '(' => Some(SUP_LPAREN),
            ')' => Some(SUP_RPAREN),
            _ => None,
        }
    }

    #[inline]
    fn to_subscript(self) -> Option<Self::Output> {
        match self {
            '0' => Some(SUB_0),
            '1' => Some(SUB_1),
            '2' => Some(SUB_2),
            '3' => Some(SUB_3),
            '4' => Some(SUB_4),
            '5' => Some(SUB_5),
            '6' => Some(SUB_6),
            '7' => Some(SUB_7),
            '8' => Some(SUB_8),
            '9' => Some(SUB_9),
            '+' => Some(SUB_PLUS),
            '-' => Some(SUB_HYPHEN),
            '=' => Some(SUB_EQUAL),
            '(' => Some(SUB_LPAREN),
            ')' => Some(SUB_RPAREN),
            _ => None,
        }
    }
}

pub trait DisplaySuperscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl DisplaySuperscript for char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_superscript()
            .ok_or(std::fmt::Error)
            .and_then(|ch| std::fmt::Write::write_char(f, ch))
    }
}

impl DisplaySuperscript for str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.chars() {
            DisplaySuperscript::fmt(&ch, f)?;
        }
        Ok(())
    }
}

pub struct Superscript<T: ?Sized + DisplaySuperscript>(pub T);

impl<T: ?Sized + DisplaySuperscript> std::fmt::Display for Superscript<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplaySuperscript::fmt(&self.0, f)
    }
}

impl<T: ?Sized + DisplaySuperscript> DisplaySuperscript for &T {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplaySuperscript::fmt(&**self, f)
    }
}

pub trait DisplaySubscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl DisplaySubscript for char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_subscript()
            .ok_or(std::fmt::Error)
            .and_then(|ch| std::fmt::Write::write_char(f, ch))
    }
}

impl DisplaySubscript for str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.chars() {
            DisplaySubscript::fmt(&ch, f)?;
        }
        Ok(())
    }
}

impl<T: ?Sized + DisplaySubscript> DisplaySubscript for &T {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplaySubscript::fmt(&**self, f)
    }
}

pub struct Subscript<T: ?Sized + DisplaySubscript>(pub T);

impl<T: ?Sized + DisplaySubscript> std::fmt::Display for Subscript<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplaySubscript::fmt(&self.0, f)
    }
}

macro_rules! impl_num_scripts {
    ($($T:ty),* $(,)?) => {$(
        impl DisplaySuperscript for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut buf = ArrayString::<64>::new();
                std::fmt::Write::write_fmt(&mut buf, format_args!("{}", self))?;
                DisplaySuperscript::fmt(buf.as_str(), f)
            }
        }
        impl DisplaySubscript for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut buf = ArrayString::<64>::new();
                std::fmt::Write::write_fmt(&mut buf, format_args!("{}", self))?;
                DisplaySubscript::fmt(buf.as_str(), f)
            }
        }
    )*};
}

impl_num_scripts! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superscript() {
        assert_eq!(&Superscript('+').to_string(), "⁺");
        assert_eq!(&Superscript("2(140+73)=426").to_string(), "²⁽¹⁴⁰⁺⁷³⁾⁼⁴²⁶");
        assert_eq!(&Superscript(-65i8).to_string(), "⁻⁶⁵");
    }

    #[test]
    fn test_subscript() {
        assert_eq!(&Subscript('+').to_string(), "₊");
        assert_eq!(&Subscript("2(140+73)=426").to_string(), "₂₍₁₄₀₊₇₃₎₌₄₂₆");
        assert_eq!(&Subscript(-65i8).to_string(), "₋₆₅");
    }
}
