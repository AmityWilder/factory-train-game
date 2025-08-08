use paste::paste;

macro_rules! unit_ops {
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
        unit_ops!($B / $A -> $C);
        unit_ops!($C * $A -> $B);
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

macro_rules! define_measurements {
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
        $(
        $vis:vis $Measurement:ident($base_unit:ident ($base_symbol:ident) $($base_si:ident)?) {
            $(
            $(* $scale:literal)? $(+ $offset:literal)? = $alt_unit:ident ($alt_symbol:ident) $($alt_si:ident)?
            ),* $(,)?
        }
        )+
    ) => {
        paste!{
            $(
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

            impl std::ops::Div<$Measurement> for f64 {
                type Output = $Measurement;

                #[inline]
                fn div(self, rhs: $Measurement) -> Self::Output {
                    $Measurement(self / rhs.0)
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
                    define_measurements!(
                        @alternate
                        $alt_unit ($alt_symbol) = $base_unit ($base_symbol) $(* $scale)? $(+ $offset)?
                    );
                )*
            }

            $($base_si!($Measurement::{ $base_unit ($base_symbol) });)?
            $($($alt_si!($Measurement::{ $alt_unit ($alt_symbol) });)?)*
            )+
        }
    };
}

macro_rules! unit_scales {
    ($($Measurement:ident::{ $($base_unit:ident ($base_symbol:ident)),* $(,)? }),* $(,)?) => {
        paste!{
            $(
            impl $Measurement {
                $(
                define_measurements!(@alternate [<tera  $base_unit>] ([<T  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e12);
                define_measurements!(@alternate [<giga  $base_unit>] ([<G  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e9);
                define_measurements!(@alternate [<mega  $base_unit>] ([<M  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e6);
                define_measurements!(@alternate [<kilo  $base_unit>] ([<k  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e3);
                define_measurements!(@alternate [<hecto $base_unit>] ([<h  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e2);
                define_measurements!(@alternate [<deca  $base_unit>] ([<da $base_symbol>]) = $base_unit ($base_symbol) * 1.0e1);
                define_measurements!(@alternate [<deci  $base_unit>] ([<d  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-1);
                define_measurements!(@alternate [<centi $base_unit>] ([<c  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-2);
                define_measurements!(@alternate [<milli $base_unit>] ([<m  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-3);
                define_measurements!(@alternate [<micro $base_unit>] ([<μ  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-6);
                define_measurements!(@alternate [<nano  $base_unit>] ([<n  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-9);
                define_measurements!(@alternate [<pico  $base_unit>] ([<p  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-12);
                define_measurements!(@alternate [<femto $base_unit>] ([<f  $base_symbol>]) = $base_unit ($base_symbol) * 1.0e-15);
                )*
            }
            )*
        }
    };
}

define_measurements! {
    pub Length(meters (m) unit_scales) {
        * 1609.0 = miles (mi),
        * 0.3048 = feet (ft),
        * 0.0254 = inches (in_),
        * 0.9144 = yards (yd),
    }

    pub Mass(grams (g) unit_scales) {
        * 453.6 = pounds (lb),
        * 28.35 = ounces (oz),
    }

    pub Time(seconds (s) unit_scales) {
        * 60.0 = minutes (min),
        * 3600.0 = hours (hr),
        * 86_400.0 = days (d),
    }

    pub Temperature(kelvin (K) unit_scales) {
        + 273.15 = degrees_celsius (degC),
        * 0.555_555_555_555_555_6 + 459.67
            = degrees_fahrenheit (degF),
    }

    pub Amount(moles (mol) unit_scales) {}

    pub Energy(joules (J) unit_scales) {}

    pub Area(meters_sqr (m2) unit_scales) {}

    pub Volume(liters (L) unit_scales) {
        = meters_cubed (m3) unit_scales,
    }

    pub Density(grams_per_cubic_meter (g_m3) unit_scales) {}
}

unit_ops!(Length * Self => Area);
unit_ops!(Area * Length => Volume);
unit_ops!(Mass / Volume => Density);
