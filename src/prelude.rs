//! The prelude brings in necessary traits for certain operations and all the unit names.
//! The unit names are all namespaced under `unit` to avoid conflicts with other names.

pub use crate::unit_traits::UnitConversion;
pub use crate::unit_traits::Unit;

// Bring in all the unit names, but namespace them under "unit"
pub mod unit {
    pub use crate::unit::*;
}