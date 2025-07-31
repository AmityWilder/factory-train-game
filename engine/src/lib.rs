//! Custom game engine using Raylib.

#![feature(min_specialization, array_windows, array_chunks, box_vec_non_null)]
#![warn(
    clippy::pedantic,
    clippy::all,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_safety_comment,
    clippy::must_use_candidate,
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    // missing_docs,
    // missing_debug_implementations
)]
#![deny(clippy::perf, clippy::multiple_unsafe_ops_per_block)]
#![allow(clippy::missing_errors_doc)]

pub mod draw;
pub use draw::{draw2d, draw3d};

pub mod prelude {
    pub use crate::{render, render_args};
}
