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

#[macro_export]
macro_rules! render_args {
    ({$arg:ident}) => {
        $crate::render_args!({}, $arg)
    };
    ({$arg:ident:?}) => {
        $crate::render_args!({:?}, $arg)
    };
    ({}, $arg:expr) => {
        [$crate::draw2d::rt::Argument::new_draw(&$arg)]
    };
    ({:?}, $arg:expr) => {
        [$crate::draw2d::rt::Argument::new_debug_vis(&$arg)]
    };
}

#[macro_export]
macro_rules! render {
    ($dst:expr, $($arg:tt)*) => {
        $crate::draw2d::Render::render($dst, $crate::draw2d::Arguments::new_v1(&$crate::render_args!($($arg)*)))
    };
}
