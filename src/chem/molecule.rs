use super::atom::Atom;
use std::{collections::BTreeMap, num::NonZeroU8};

// Dashed line = London Dispersion Force (LDF)

/// Keeping elements deep-sorted enables equality testing
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compound {
    Atom(Atom),
    Tree(BTreeMap<Compound, NonZeroU8>),
}

impl std::fmt::Display for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => std::fmt::Display::fmt(atom, f),
            Self::Tree(btree_map) => todo!(),
        }
    }
}
