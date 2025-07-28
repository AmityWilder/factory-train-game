pub const AVOGADROS_NUMBER: f64 = 6.022_140_76e+26;
/// Atomic mass units (AMU) per kilogram (kg)
pub const AMU_PER_KG: f64 = AVOGADROS_NUMBER;
/// Kilograms (kg) per atomic mass unit (AMU)
pub const KG_PER_AMU: f64 = AMU_PER_KG.recip();

/// Picometers (pm) per meter (m)
pub const PM_PER_M: f64 = 1e+12;
/// Meters (m) per picometer (pm)
pub const M_PER_PM: f64 = PM_PER_M.recip();

/// Mass of a single proton in AMU
pub const PROTON_MASS: f64 = 1.007_276_466_879_91;
/// Mass of a single neutron in AMU
pub const NEUTRON_MASS: f64 = 1.008_106;
/// Mass of a single electron in AMU
pub const ELECTRON_MASS: f64 = 5.485_799_090_701_6e-4;
