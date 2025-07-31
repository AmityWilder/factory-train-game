#![allow(missing_debug_implementations)]

#[allow(clippy::wildcard_imports)]
use super::*;
use std::ptr::NonNull;

#[derive(Copy, Clone)]
pub struct Placeholder {
    pub position: usize,
    pub translation: Vector2,
    pub rotation: f32,
    pub scale: Vector2,
    pub tint: Color,
}

#[derive(Copy, Clone)]
enum ArgumentType<'a> {
    Placeholder {
        // INVARIANT: `renderer` has type `fn(&T, _) -> _` for some `T`, and `value`
        // was derived from a `&'a T`.
        value: NonNull<()>,
        renderer: unsafe fn(NonNull<()>, &mut Renderer<'_>) -> Result,
        _lifetime: PhantomData<&'a ()>,
    },
}

/// This struct represents a generic "argument" which is taken by [`render_args!()`].
///
/// This can be either a placeholder argument or a count argument.
///
/// - A placeholder argument contains a function to render the given value. At
///   compile time it is ensured that the function and the value have the correct
///   types, and then this struct is used to canonicalize arguments to one type.
///   Placeholder arguments are essentially an optimized partially applied rendering
///   function, equivalent to `exists T.(&T, fn(&T, &mut Renderer<'_>) -> Result`.
///
/// - A count argument contains a count for dynamic rendering parameters like
///   precision and width.
#[derive(Copy, Clone)]
pub struct Argument<'a> {
    ty: ArgumentType<'a>,
}

macro_rules! argument_new {
    ($T:ty, $x:expr, $d:expr) => {
        Argument {
            // INVARIANT: this creates an `ArgumentType<'a>` from a `&'a T` and
            // a `fn(&T, ...)`, so the invariant is maintained.
            ty: ArgumentType::Placeholder {
                value: NonNull::<$T>::from_ref($x).cast(),
                renderer: {
                    let d: fn(&$T, &mut Renderer<'_>) -> Result = $d;
                    // SAFETY: This is only called with `value`, which has the right type.
                    unsafe {
                        std::mem::transmute::<
                            fn(&T, &mut Renderer<'_>) -> Result,
                            unsafe fn(std::ptr::NonNull<()>, &mut Renderer<'_>) -> Result,
                        >(d)
                    }
                },
                _lifetime: PhantomData,
            },
        }
    };
}

impl Argument<'_> {
    #[inline]
    pub const fn new_draw<T: Draw>(x: &T) -> Argument<'_> {
        argument_new!(T, x, <T as Draw>::draw)
    }
    #[inline]
    pub const fn new_debug_vis<T: DebugVis>(x: &T) -> Argument<'_> {
        argument_new!(T, x, <T as DebugVis>::draw)
    }
    #[inline]
    pub const fn new_debug_noop<T: DebugVis>(x: &T) -> Argument<'_> {
        argument_new!(T, x, |_: &T, _| Ok(()))
    }
    /// Format this placeholder argument.
    ///
    /// # Safety
    ///
    /// This argument must actually be a placeholder argument.
    #[inline]
    pub(super) unsafe fn fmt(&self, f: &mut Renderer<'_>) -> Result {
        match self.ty {
            ArgumentType::Placeholder {
                renderer, value, ..
            } =>
            // SAFETY:
            // Because of the invariant that if `renderer` had the type
            // `fn(&T, _) -> _` then `value` has type `&'b T` where `'b` is
            // the lifetime of the `ArgumentType`, and because references
            // and `NonNull` are ABI-compatible, this is completely equivalent
            // to calling the original function passed to `new` with the
            // original reference, which is sound
            unsafe { renderer(value, f) },
        }
    }

    /// Used by `render_args` when all arguments are gone after inlining,
    /// when using `&[]` would incorrectly allow for a bigger lifetime.
    ///
    /// This fails without format argument inlining, and that shouldn't be different
    /// when the argument is inlined:
    #[inline]
    pub const fn none() -> [Self; 0] {
        []
    }
}

/// Used by the [`render_args!()`] macro to create a fmt::Arguments object.
#[doc(hidden)]
impl<'a> Arguments<'a> {
    /// When using the [`render_args!()`] macro, this function is used to generate the
    /// Arguments structure.
    #[inline]
    #[must_use]
    #[allow(
        clippy::missing_const_for_fn,
        reason = "This function should *not* be const, to make sure we don't accept \
        [`render_args!()`] and panic!() with arguments in const, even when not evaluated"
    )]
    pub fn new_v1<const N: usize>(args: &'a [rt::Argument<'a>; N]) -> Arguments<'a> {
        Arguments { fmt: None, args }
    }
}
