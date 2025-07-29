#![allow(missing_debug_implementations)]

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
pub enum Count {
    /// Specified with a literal number, stores the value
    Is(u16),
    /// Specified using `$` and `*` syntaxes, stores the index into `args`
    Param(usize),
    /// Not specified
    Implied,
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
    Count(u16),
}
/// This struct represents a generic "argument" which is taken by format_args!().
///
/// This can be either a placeholder argument or a count argument.
/// * A placeholder argument contains a function to format the given value. At
///   compile time it is ensured that the function and the value have the correct
///   types, and then this struct is used to canonicalize arguments to one type.
///   Placeholder arguments are essentially an optimized partially applied formatting
///   function, equivalent to `exists T.(&T, fn(&T, &mut Renderer<'_>) -> Result`.
/// * A count argument contains a count for dynamic formatting parameters like
///   precision and width.
#[derive(Copy, Clone)]
pub struct Argument<'a> {
    ty: ArgumentType<'a>,
}

macro_rules! argument_new {
    ($t:ty, $x:expr, $f:expr) => {
        Argument {
            // INVARIANT: this creates an `ArgumentType<'a>` from a `&'a T` and
            // a `fn(&T, ...)`, so the invariant is maintained.
            ty: ArgumentType::Placeholder {
                value: NonNull::<$t>::from_ref($x).cast(),
                renderer: {
                    let f: fn(&$t, &mut Renderer<'_>) -> Result = $f;
                    // SAFETY: This is only called with `value`, which has the right type.
                    unsafe { std::mem::transmute(f) }
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
    pub const fn new_debugvis<T: DebugVis>(x: &T) -> Argument<'_> {
        argument_new!(T, x, <T as DebugVis>::draw)
    }
    #[inline]
    pub const fn new_debug_noop<T: DebugVis>(x: &T) -> Argument<'_> {
        argument_new!(T, x, |_: &T, _| Ok(()))
    }
    #[inline]
    #[track_caller]
    pub const fn from_usize(x: &usize) -> Argument<'_> {
        if *x > u16::MAX as usize {
            panic!("Formatting argument out of range");
        }
        Argument {
            ty: ArgumentType::Count(*x as u16),
        }
    }
    /// Format this placeholder argument.
    ///
    /// # Safety
    ///
    /// This argument must actually be a placeholder argument.
    #[inline]
    pub(super) unsafe fn fmt(&self, f: &mut Renderer<'_>) -> Result {
        match self.ty {
            // SAFETY:
            // Because of the invariant that if `renderer` had the type
            // `fn(&T, _) -> _` then `value` has type `&'b T` where `'b` is
            // the lifetime of the `ArgumentType`, and because references
            // and `NonNull` are ABI-compatible, this is completely equivalent
            // to calling the original function passed to `new` with the
            // original reference, which is sound.
            ArgumentType::Placeholder {
                renderer, value, ..
            } => unsafe { renderer(value, f) },
            // SAFETY: the caller promised this.
            ArgumentType::Count(_) => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline]
    pub(super) const fn as_u16(&self) -> Option<u16> {
        match self.ty {
            ArgumentType::Count(count) => Some(count),
            ArgumentType::Placeholder { .. } => None,
        }
    }

    /// Used by `format_args` when all arguments are gone after inlining,
    /// when using `&[]` would incorrectly allow for a bigger lifetime.
    ///
    /// This fails without format argument inlining, and that shouldn't be different
    /// when the argument is inlined:
    ///
    /// ```compile_fail,E0716
    /// let f = format_args!("{}", "a");
    /// println!("{f}");
    /// ```
    #[inline]
    pub const fn none() -> [Self; 0] {
        []
    }
}

/// This struct represents the unsafety of constructing an `Arguments`.
/// It exists, rather than an unsafe function, in order to simplify the expansion
/// of `format_args!(..)` and reduce the scope of the `unsafe` block.
pub struct UnsafeArg {
    _private: (),
}

impl UnsafeArg {
    /// See documentation where `UnsafeArg` is required to know when it is safe to
    /// create and use `UnsafeArg`.
    #[inline]
    pub const unsafe fn new() -> Self {
        Self { _private: () }
    }
}

/// Used by the format_args!() macro to create a fmt::Arguments object.
#[doc(hidden)]
impl<'a> Arguments<'a> {
    #[inline]
    pub const fn new_const<const N: usize>(pieces: &'a [&'static str; N]) -> Self {
        const { assert!(N <= 1) };
        Arguments {
            fmt: None,
            args: &[],
        }
    }

    /// When using the format_args!() macro, this function is used to generate the
    /// Arguments structure.
    ///
    /// This function should _not_ be const, to make sure we don't accept
    /// format_args!() and panic!() with arguments in const, even when not evaluated:
    #[inline]
    pub fn new_v1<const P: usize, const A: usize>(
        args: &'a [rt::Argument<'a>; A],
    ) -> Arguments<'a> {
        Arguments { fmt: None, args }
    }

    /// Specifies nonstandard formatting parameters.
    ///
    /// An `rt::UnsafeArg` is required because the following invariants must be held
    /// in order for this function to be safe:
    /// 1. The `pieces` slice must be at least as long as `fmt`.
    /// 2. Every `rt::Placeholder::position` value within `fmt` must be a valid index of `args`.
    /// 3. Every `rt::Count::Param` within `fmt` must contain a valid index of `args`.
    ///
    /// This function should _not_ be const, to make sure we don't accept
    /// format_args!() and panic!() with arguments in const, even when not evaluated:
    #[inline]
    pub fn new_v1_formatted(
        pieces: &'a [&'static str],
        args: &'a [rt::Argument<'a>],
        fmt: &'a [rt::Placeholder],
        _unsafe_arg: rt::UnsafeArg,
    ) -> Arguments<'a> {
        Arguments {
            fmt: Some(fmt),
            args,
        }
    }
}
