#![doc = include_str!("matter-and-measurements.md")]
#![allow(non_snake_case)]
#![feature(const_trait_impl, auto_traits)]

use std::num::NonZeroUsize;

pub mod units;
use units::{Mass, Volume};

/// Matter is anything that has mass and takes up space.
#[derive(Debug)]
pub struct Matter {
    pub mass: Mass,
    pub space: Volume,
}

/// Substances of matter can be classified as pure substances or mixtures.
#[derive(Debug)]
pub enum Substance {
    Pure(),
    Mixture(),
}

/// [`Compound`]s - contain two or more different elements, and a wide variety of subscripts on the
/// elements in the compound are possible.
#[derive(Debug)]
pub struct Compound(pub Vec<(Element, NonZeroUsize)>);

/// [`Element`]s - pertain to those you see listed on the periodic table in their naturally occurring state.
///
/// Some elements are monoatomic--they exist as single atoms.
///
/// | Name    | Carbon | Sodium |
/// |---------|--------|--------|
/// | Formula |    C   |   Na   |
///
/// Others may exist as molecules--one or more atoms of the same kind bonded together.
///
/// | Name    | Nitrogen | Oxygen | Chlorine |
/// |---------|----------|--------|----------|
/// | Formula |    N₂    |   O₃   |    Cl₂   |
#[derive(Debug)]
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

/// [`Molecule`] - two or more atoms joined together by strong bonds.
///
/// Ex) **Di**atomics: H₂, N₂, F₂, O₂, I₂, Cl₂, Br₂
/// Name: Name of the element.
pub struct Molecule(pub Vec<Atom>);

pub struct Atom; // TODO

pub enum Mixture {
    /// Homogeneous mixtures - uniform characteristics throughout a sample of substance
    ///
    /// Ex) Air
    Homogeneous,

    /// Heterogeneous - substances which have many parts that are not the same. A sample from one
    /// part may be different from a sample taken from another part.
    ///
    /// Ex) Sand
    Heterogeneous,
}

// Measurements are quantitative data abtained directly from an analytical instrument (for example
// a thermometer, analytical balance, graduated cylinder, or ruler).

// UNIT(S) must follow all measurements. Some common units in the lab are: °C, g, mL, and cm

// Precision is the amount of information conveyed in a number related to its digits. For example.
// 2.3 cm is less precise than 2.34 cm as the "3" in the tenths place is a less precise increment
// than the "4" in the hundredths place.

// The precision of a number is determined by the instrument itself. Each instrument has its own
// levelof precision, and every reading obtained from a single instrument will have the same level
// of precision. Keep in mind that the number of significant figures may vary despite numbers
// having the same levelof precision. For example, consider this digital thermometer. Because the
// same instrument is used, all of the readings are ALL precise to the tenths place. However 98.6
// °F contains three significant figures while 100.0 F and 101.8 °F both contain four
// figures.

/// *Analytical Balances*
///
/// This instrument is adigital instrumnent used to measure mass. The unit of the measured number
/// is typically grams (g). When using adigital tool you always write down every digit shown in the
/// display - including all the zeros! Check out the link for more information on how to use an
/// analytical balance in the lab.
pub struct AnalyticalBalance;

/// *Rulers*
///
/// Rulers measure length. In the chemistry lab we use metric rulers, most often with centinmeters
/// (cm) as the unit. When using a hatched marking tool always write down the known digits plus
/// one estimated digit. The precisionof hatch marked tools, such as a ruler, depends upon the
/// increments.
///
/// 2.55 cm is more precise than 2.5cm
pub struct Ruler;

/// *Graduated Cylinders*
///
/// This is a common lab piece of liquid volume measuring equipment. The unit of measurement is
/// typically milliliters (mL). Graduated cylinders come in varying sizes. Graduated cylinders are a
/// type of hatch marking, and have varying levels of precision. Obtaining an accurate reading
/// requires proper technique and training. Check out the link for more information on how to use
/// and read a graduated cylinder.
///
/// <table>
/// <tr><td><img/></td><td><img/></td><td><img/></td></tr>
/// <tr>
/// <td><p>Various sizes.</p><p>Precision level will vary<br/>depending on the increments<br/>between graduated markings.</p></td>
/// <td><p>View at eye level</p><p>Read at the bottom of the curved<br/>meniscus</p></td>
/// <td><p>The markings are at the ones<br/>place, therefore we estimate to<br/>the tenths place.</p><p>Record volume as 30.0 mL</p></td>
/// </tr>
/// </table>
pub struct GraduatedCylinder;

// **Observing Matter**
//
// One of the most important skills of any scientist is their ability to observe, record, and
// communicate recorded observations and data. Observations fall into two basic categories
// qualitative and quantitative.
//
// *Qualitative observations*
//
// are observations that describe "qualities" of something. These would include characteristics
// such as texture, color shape, and do not involve countingor measuring. Qualitative data
// includes words - NOT numbers.
//
// *Quantitative observations/data*
//
// involve counting or measuring "quantities" using standard increments. Thiswould include such
// things as mass, volume, time, temperature, frequency of occurrence, etc. Quantitative data
// includes numbers and units.
//
// ***Qualitative vs Quantitative Video Overview***: https://youtu.be/sRKO49652ic

// **Classifying Properties and Changes in Matter**
// *Properties: Chemical vs Physical*
