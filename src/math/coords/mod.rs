use raylib::prelude::Vector3;

#[const_trait]
pub trait VectorConstants: Sized + Copy + std::ops::Neg {
    /// Vector with all components set to 0
    const ZERO: Self;
    /// Vector with all components set to 1
    const ONE: Self;
    /// Vector with all components set to -1
    const NEG_ONE: Self;
    /// Vector with all components set to the minimum for their type
    const MIN: Self;
    /// Vector with all components set to the maximum for their type
    const MAX: Self;
    /// <1, 0, 0>
    const X: Self;
    /// <0, 1, 0>
    const Y: Self;
    /// <0, 0, 1>
    const Z: Self;
    /// <-1, 0, 0>
    const NEG_X: Self;
    /// <0, -1, 0>
    const NEG_Y: Self;
    /// <0, 0, -1>
    const NEG_Z: Self;

    /// The forward (length) vector
    const FORWARD: Self = Self::NEG_Z;
    /// The negative [`Self::FORWARD`] (length) vector
    const BACKWARD: Self = Self::Z;
    /// The right (width) vector
    const RIGHT: Self = Self::X;
    /// The negative [`Self::RIGHT`] (width) vector
    const LEFT: Self = Self::NEG_X;
    /// The up (height) vector
    const UP: Self = Self::Y;
    /// The negative [`Self::UP`] (height) vector
    const DOWN: Self = Self::NEG_Y;
}

impl VectorConstants for Vector3 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    const NEG_ONE: Self = Self::NEG_ONE;
    const X: Self = Self::X;
    const Y: Self = Self::Y;
    const Z: Self = Self::Z;
    const NEG_X: Self = Self::NEG_X;
    const NEG_Y: Self = Self::NEG_Y;
    const NEG_Z: Self = Self::NEG_Z;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

pub mod factory;
pub mod lab;
pub mod player;
pub mod rail;

pub use {
    factory::FactoryVector3,
    lab::LabVector3,
    player::{PlayerCoord, PlayerVector3},
    rail::RailVector3,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromFactoryVectorError(());

impl std::fmt::Display for TryFromFactoryVectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "i32 Rail coordinates were out of bounds of i16 Factory coordinates".fmt(f)
    }
}

impl std::error::Error for TryFromFactoryVectorError {}
