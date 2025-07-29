//! Custom game engine using Raylib.

#![warn(
    clippy::pedantic,
    clippy::all,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_safety_comment,
    clippy::must_use_candidate,
    // missing_docs,
    // missing_debug_implementations
)]
#![deny(clippy::perf, clippy::multiple_unsafe_ops_per_block)]
#![forbid(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![allow(clippy::missing_errors_doc)]

pub mod draw;
pub use draw::{draw2d, draw3d};
