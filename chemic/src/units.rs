use paste::paste;

// macro_rules! product {
//     ([$($arr:tt)*] ; $($_ys:tt)*) => {
//         $($arr)*
//     };

//     ([$($arr:tt)*] $x:tt $(, $xs:tt)*; $($ys:tt),*) => {
//         product!([$($arr)* $($x $ys)*] $($xs),*; $($ys),*)
//     };
// }

// product!([] a, b, c, d; 1, 2, 3, 4);

#[doc(hidden)]
#[track_caller]
pub fn assert_near_eq_failed_inner(
    left: &dyn std::fmt::Debug,
    right: &dyn std::fmt::Debug,
    epsilon: &dyn std::fmt::Debug,
    args: Option<std::fmt::Arguments<'_>>,
) -> ! {
    match args {
        Some(args) => panic!(
            r#"assertion `left == right within epsilon` failed: {args}
    left: {left:?}
   right: {right:?}
 epsilon: {epsilon:?}"#
        ),
        None => panic!(
            r#"assertion `left == right within epsilon` failed
    left: {left:?}
   right: {right:?}
 epsilon: {epsilon:?}"#
        ),
    }
}

#[macro_export]
macro_rules! assert_near_eq {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon) => {
                if ((*left_val - *right_val).abs() > *epsilon) {
                    $crate::units::assert_near_eq_failed_inner(&*left_val, &*right_val, &*epsilon, None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $epsilon:expr, $($arg:tt)+) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon) => {
                if ((*left_val - *right_val).abs() > *epsilon) {
                    $crate::units::assert_near_eq_failed_inner(&*left_val, &*right_val, &*epsilon, Some(format_args!($($arg)+)));
                }
            }
        }
    };
}

macro_rules! unit_ops {
    // symmetric
    ($A:ident + Self => $B:ident) => {
        unit_ops!($A + $A -> $B);
        unit_ops!($B - $A -> $A);
    };
    ($A:ident - Self => $B:ident) => {
        unit_ops!($A - $A -> $B);
        unit_ops!($B + $A -> $A);
    };
    ($A:ident * Self => $B:ident) => {
        unit_ops!($A * $A -> $B);
        unit_ops!($B / $A -> $A);
    };
    ($A:ident / Self => $B:ident) => {
        unit_ops!($A / $A -> $B);
        unit_ops!($B * $A -> $A);
    };
    ($A:ident + $B:ident => $C:ident) => {
        unit_ops!($A + $B -> $C);
        unit_ops!($B + $A -> $C);
        unit_ops!($C - $A -> $B);
        unit_ops!($C - $B -> $A);
    };
    ($A:ident - $B:ident => $C:ident) => {
        unit_ops!($A - $B -> $C);
        unit_ops!($B - $A -> $C);
        unit_ops!($C + $A -> $B);
        unit_ops!($C + $B -> $A);
    };
    ($A:ident * $B:ident => $C:ident) => {
        unit_ops!($A * $B -> $C);
        unit_ops!($B * $A -> $C);
        unit_ops!($C / $A -> $B);
        unit_ops!($C / $B -> $A);
    };
    ($A:ident / $B:ident => $C:ident) => {
        unit_ops!($A / $B -> $C);
        unit_ops!($C * $B -> $A);
    };

    ($A:ident * $B:ident -> $C:ident) => {
        impl std::ops::Mul<$B> for $A {
            type Output = $C;

            #[inline]
            fn mul(self, rhs: $B) -> Self::Output {
                $C(self.0 * rhs.0)
            }
        }
    };

    ($A:ident / $B:ident -> $C:ident) => {
        impl std::ops::Div<$B> for $A {
            type Output = $C;

            #[inline]
            fn div(self, rhs: $B) -> Self::Output {
                $C(self.0 / rhs.0)
            }
        }
    };

    ($A:ident + $B:ident -> $C:ident) => {
        impl std::ops::Add<$B> for $A {
            type Output = $C;

            #[inline]
            fn add(self, rhs: $B) -> Self::Output {
                $C(self.0 + rhs.0)
            }
        }
    };

    ($A:ident - $B:ident -> $C:ident) => {
        impl std::ops::Sub<$B> for $A {
            type Output = $C;

            #[inline]
            fn sub(self, rhs: $B) -> Self::Output {
                $C(self.0 - rhs.0)
            }
        }
    };
}

macro_rules! define_measurement {
    (
        @alternate
        $alt_unit:ident ($alt_symbol:ident) = $base_unit:ident ($base_symbol:ident) $(* $scale:literal)? $(+ $offset:literal)?
    ) => {
        paste!{
            pub const [<$alt_unit:upper _PER_ $base_unit:upper>]: f64 = 1.0 $(/ $scale)?;
            pub const [<$base_unit:upper _PER_ $alt_unit:upper>]: f64 = 1.0 $(* $scale)?;
            pub const [<$alt_unit:upper _AT_0_ $base_unit:upper>]: f64 = 0.0 $(- $offset)?;
            pub const [<$base_unit:upper _AT_0_ $alt_unit:upper>]: f64 = 0.0 $(+ $offset)?;

            #[inline]
            #[must_use]
            pub const fn [<from_ $alt_unit>]($alt_symbol: f64) -> Self {
                Self::[<from_ $base_unit>](($alt_symbol + Self::[<$base_unit:upper _AT_0_ $alt_unit:upper>]) * Self::[<$base_unit:upper _PER_ $alt_unit:upper>])
            }

            #[inline]
            #[must_use]
            pub const fn [<to_ $alt_unit>](self) -> f64 {
                self.[<to_ $base_unit>]() * Self::[<$alt_unit:upper _PER_ $base_unit:upper>] + Self::[<$alt_unit:upper _AT_0_ $base_unit:upper>]
            }
        }
    };

    (
        $(#[$m:meta])*
        $vis:vis $Measurement:ident($base_unit:ident ($base_symbol:ident) $($base_si:ident)?) {
            $(
            $(* $scale:literal)? $(+ $offset:literal)? = $alt_unit:ident ($alt_symbol:ident) $($alt_si:ident)?
            ),* $(,)?
        }
    ) => {
        paste!{
            $(#[$m])*
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
            $vis struct $Measurement(f64);

            unit_ops!($Measurement + Self => $Measurement);

            impl std::ops::Mul<f64> for $Measurement {
                type Output = $Measurement;

                #[inline]
                fn mul(self, rhs: f64) -> Self::Output {
                    $Measurement(self.0 * rhs)
                }
            }

            impl std::ops::Mul<$Measurement> for f64 {
                type Output = $Measurement;

                #[inline]
                fn mul(self, rhs: $Measurement) -> Self::Output {
                    $Measurement(self * rhs.0)
                }
            }

            impl std::ops::Div<f64> for $Measurement {
                type Output = $Measurement;

                #[inline]
                fn div(self, rhs: f64) -> Self::Output {
                    $Measurement(self.0 / rhs)
                }
            }

            impl std::ops::Div<$Measurement> for $Measurement {
                type Output = f64;

                #[inline]
                fn div(self, rhs: $Measurement) -> Self::Output {
                    self.0 / rhs.0
                }
            }

            impl $Measurement {
                #[inline]
                #[must_use]
                pub const fn [<from_ $base_unit>]($base_symbol: f64) -> Self {
                    Self($base_symbol)
                }

                #[inline]
                #[must_use]
                pub const fn [<to_ $base_unit>](self) -> f64 {
                    self.0
                }

                $(
                    define_measurement!(
                        @alternate
                        $alt_unit ($alt_symbol) = $base_unit ($base_symbol) $(* $scale)? $(+ $offset)?
                    );
                )*
            }

            $($base_si!($Measurement::{ $base_unit ($base_symbol) });)?
            $($($alt_si!($Measurement::{ $alt_unit ($alt_symbol) });)?)*
        }
    };
}

macro_rules! unit_scales {
    ($($Measurement:ident::{ $($base_unit:ident ($base_symbol:ident)),* $(,)? }),* $(,)?) => {
        paste!{
            $(
            impl $Measurement {
                $(
                define_measurement!(@alternate [<tera  $base_unit>] ([<T  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e12);
                define_measurement!(@alternate [<giga  $base_unit>] ([<G  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e9);
                define_measurement!(@alternate [<mega  $base_unit>] ([<M  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e6);
                define_measurement!(@alternate [<kilo  $base_unit>] ([<k  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e3);
                define_measurement!(@alternate [<hecto $base_unit>] ([<h  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e2);
                define_measurement!(@alternate [<deca  $base_unit>] ([<da $base_symbol>]) = $base_unit ($base_symbol) * 1.0e1);
                define_measurement!(@alternate [<deci  $base_unit>] ([<d  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-1);
                define_measurement!(@alternate [<centi $base_unit>] ([<c  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-2);
                define_measurement!(@alternate [<milli $base_unit>] ([<m  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-3);
                define_measurement!(@alternate [<micro $base_unit>] ([<μ  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-6);
                define_measurement!(@alternate [<nano  $base_unit>] ([<n  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-9);
                define_measurement!(@alternate [<pico  $base_unit>] ([<p  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-12);
                define_measurement!(@alternate [<femto $base_unit>] ([<f  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-15);
                )*
            }
            )*
        }
    };
}

define_measurement! {
    pub Length(meters (m) unit_scales) {
        * 1609.0 = miles (mi),
        * 0.3048 = feet (ft),
        * 0.0254 = inches (in_),
        * 0.9144 = yards (yd),
    }
}
impl Length {
    #[inline]
    #[must_use]
    pub const fn squared(self) -> Area {
        Area(self.0 * self.0)
    }

    #[inline]
    #[must_use]
    pub const fn cubed(self) -> Volume {
        Volume(self.0 * self.0 * self.0)
    }
}
define_measurement! {
    pub Mass(grams (g) unit_scales) {
        * 453.6 = pounds (lb),
        * 28.35 = ounces (oz),
    }
}
define_measurement! {
    pub Time(seconds (s) unit_scales) {
        * 60.0 = minutes (min),
        * 3600.0 = hours (hr),
        * 86_400.0 = days (d),
    }
}
define_measurement! {
    pub Temperature(kelvin (K) unit_scales) {
        + 273.15 = degrees_celsius (degC),
        * 0.555_555_555_555_555_6 + 459.67
            = degrees_fahrenheit (degF),
    }
}
define_measurement! {
    pub Amount(moles (mol) unit_scales) {}
}
define_measurement! {
    pub Energy(joules (J) unit_scales) {}
}
define_measurement! {
    /// [`Length`] &times; [`Width`]
    pub Area(meters_sqr (m2) unit_scales) {}
}
impl Area {
    #[inline]
    #[must_use]
    pub const fn new(l: Length, w: Width) -> Self {
        Self(l.0 * w.0)
    }

    #[inline]
    #[must_use]
    pub const fn split(self) -> (Length, Width) {
        (Length(self.0), Width(1.0))
    }
}
define_measurement! {
    /// [`Length`] &times; [`Width`] &times; [`Height`]
    pub Volume(liters (L) unit_scales) {
        = meters_cubed (m3),
    }
}
impl Volume {
    #[inline]
    #[must_use]
    pub const fn new(l: Length, w: Width, h: Height) -> Self {
        Self(l.0 * w.0 * h.0)
    }
}
define_measurement! {
    /// [`Mass`] &div; [`Volume`]
    pub Density(grams_per_liter (g_L)) {
        = grams_per_meter_cubed (g_m3),
    }
}
impl Density {
    #[inline]
    #[must_use]
    pub const fn new(m: Mass, V: Volume) -> Self {
        Self(m.0 / V.0)
    }
}
define_measurement! {
    /// [`Distance`] over [`Time`]
    pub Speed(meters_per_sec (m_s)) {}
}
impl Speed {
    #[inline]
    #[must_use]
    pub const fn new(m: Distance, V: Time) -> Self {
        Self(m.0 / V.0)
    }
}

pub use Length as Width;
pub use Length as Height;
pub use Length as Depth;
pub use Length as Distance;

unit_ops!(Length * Self => Area);
unit_ops!(Area * Length => Volume);
unit_ops!(Mass / Volume => Density);
unit_ops!(Density / Mass => Volume);
unit_ops!(Mass / Density => Volume);
unit_ops!(Distance / Time => Speed);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume() {
        assert_near_eq!(Volume::from_milliliters(1000.0).to_liters(), 1.0, 0.001);
        assert_near_eq!(
            (Length::from_centimeters(100.0)
                * Width::from_centimeters(100.0)
                * Height::from_centimeters(100.0))
            .to_meters_cubed(),
            0.0001,
            0.0001
        );
    }

    #[test]
    fn problem_1() {
        let m: Mass = Mass::from_grams(4.55); // 3 s.f.
        let V_f: Volume = Volume::from_milliliters(23.1); // 3 s.f.
        let V_i: Volume = Volume::from_milliliters(19.4); // 3 s.f.
        let V: Volume = V_f - V_i; // 2 s.f.
        assert_near_eq!(V.to_milliliters(), 3.7, 0.1);
        let D: Density = m / V; // 2 s.f.
        assert_near_eq!(D.to_grams_per_liter() * 1000.0, 1.2, 0.1);
    }

    #[test]
    fn problem_2() {
        let m = Mass::from_kilograms(0.250); // 3 s.f.

        let D_water: Density = Mass::from_kilograms(1000.0) / Volume::from_liters(1.0); // 4 s.f.
        let V_water: Volume = m / D_water; // 3 s.f.
        assert_near_eq!(V_water.to_liters(), 2.50e-4, 0.01e-4);

        let D_ice: Density = Mass::from_kilograms(920.0) / Volume::from_liters(1.0); // 3 s.f.
        let V_ice: Volume = m / D_ice; // 3 s.f.
        assert_near_eq!(V_ice.to_liters(), 2.71e-4, 0.01e-4);
    }

    #[test]
    fn problem_3() {
        let D_gold: Density = Mass::from_grams(19.3) / Length::from_centimeters(1.0).cubed(); // 3 s.f.
        let l = Length::from_inches(6.00); // 3 s.f.
        let w = Width::from_inches(4.00); // 3 s.f.
        let h = Height::from_inches(2.00); // 3 s.f.
        let V = l * w * h; // 3 s.f.
        assert_near_eq!(V / Length::from_inches(1.0).cubed(), 48.0, 0.1);
        let m = D_gold * V;
    }
}
