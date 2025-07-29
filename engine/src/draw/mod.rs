//! Drawing inspired by [`std::fmt`].

pub mod draw2d;
pub mod draw3d;

/// The error type which is returned from rendering to a buffer.
#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "draw failed unexpectedly")
    }
}

impl std::error::Error for Error {}

/// The type returned by renderer methods.
pub type Result = std::result::Result<(), Error>;
