#![doc = include_str!("../README.md")]

// The unit types
pub use length::Length;
pub use time::Time;
pub use data::Data;
pub use data_rate::DataRate;

// The unit name enums
pub mod unit {
    //! The unit names are all namespaced under `unit` to avoid conflicts with other names.
    //! Unit names come from their respective modules and are re-exported from lib.rs.
    //! Within a given dimension module, unit enums are defined in the form DimensionUnit.
    //! For example, the length module has LengthUnit, the time module has TimeUnit, etc.
    pub use crate::length::LengthUnit::*;
    pub use crate::time::TimeUnit::*;
    pub use crate::data::DataUnit::*;
    pub use crate::data_rate::DataRateUnit::*;
}

// Prelude
pub mod prelude;

// Module declarations
pub mod length;
pub mod time;
pub mod data;
pub mod data_rate;

// Module declarations - internal modules
mod unit_traits;
mod utils;


