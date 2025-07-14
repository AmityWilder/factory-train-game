use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Element {
    #[default]
    H = 1,                                                                                                                      He,
    Li, Be,                                                                                                 B,  C,  N,  O,  F,  Ne,
    Na, Mg,                                                                                                 Al, Si, P,  S,  Cl, Ar,
    K,  Ca, Sc,                                                         Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y,                                                          Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I,  Xe,
    Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yd, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra, Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, T,  Og,
}

impl Element {
    #[inline]
    pub const fn protons(self) -> NonZeroU8 {
        // SAFETY: No element has 0 protons. That would be a beta particle.
        unsafe { NonZeroU8::new_unchecked(self as u8) }
    }

    pub const fn is_diatomic(self) -> bool {
        use Element::*;
        matches!(self, H | N | O | F | Cl | Br | I)
    }
}

pub struct PolyAtomic {

}

pub struct Ion {

}

pub struct Compound {

}
