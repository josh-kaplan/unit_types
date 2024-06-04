//! This module contains the `Data` enum and its associated unit conversions.
//! The `Data` enum is used to represent data sizes in bytes, kilobytes, megabytes, etc.
//! The `DataUnit` enum is used to specify the unit of the data size.
//! The `Data` enum implements the `Unit` and `UnitConversion` traits, which offer the ability to 
//! convert between different units of data sizes via the `to_base` and `to` methods.

use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;
use crate::unit_traits::UnitConversion;
use crate::unit_traits::Unit;


#[derive(Debug)]
#[warn(non_camel_case_types)]
pub enum DataUnit {
    Bytes,
    Kilobytes,
    KB,
    Megabytes,
    MB,
    Gigabytes,
    GB,
    Terabytes,
    TB,
    Kibibytes,
    KiB,
    Mebibytes,
    MiB,
    Gibibytes,
    GiB,
    Tebibytes,
    TiB,
}


/// The `Data` enum represents data sizes in bytes, kilobytes, megabytes, etc.
#[derive(Clone,Copy,Debug)]
pub enum Data {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
    Kibibytes(f64),
    Mebibytes(f64),
    Gibibytes(f64),
    Tebibytes(f64),
}

impl Data {
    pub const BYTES_PER_KB: f64 = 1000.0;
    pub const BYTES_PER_MB: f64 = 1000.0 * 1000.0;
    pub const BYTES_PER_GB: f64 = 1000.0 * 1000.0 * 1000.0;
    pub const BYTES_PER_TB: f64 = 1000.0 * 1000.0 * 1000.0 * 1000.0;

    pub const BYTES_PER_KBI: f64 = 1024.0;
    pub const BYTES_PER_MBI: f64 = 1024.0 * 1024.0;
    pub const BYTES_PER_GBI: f64 = 1024.0 * 1024.0 * 1024.0;
    pub const BYTES_PER_TBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    /* ALIASES */

    #[allow(non_snake_case)]
    pub fn KB(x: f64) -> Data {
        Data::Kilobytes(x)
    }

    #[allow(non_snake_case)]
    pub fn KiB(x: f64) -> Data {
        Data::Kibibytes(x)
    }

    #[allow(non_snake_case)]
    pub fn MB(x: f64) -> Data {
        Data::Megabytes(x)
    }

    #[allow(non_snake_case)]
    pub fn MiB(x: f64) -> Data {
        Data::Mebibytes(x)
    }

    #[allow(non_snake_case)]
    pub fn GB(x: f64) -> Data {
        Data::Gigabytes(x)
    }

    #[allow(non_snake_case)]
    pub fn GiB(x: f64) -> Data {
        Data::Gibibytes(x)
    }
}

impl Unit for Data {
    /// Returns the inner f64 value of the Data enum.
    fn value(&self) -> f64 {
        match self {
            Data::Bytes(x) => *x,
            Data::Kilobytes(x) => *x,
            Data::Megabytes(x) => *x,
            Data::Gigabytes(x) => *x,
            Data::Terabytes(x) => *x,
            Data::Kibibytes(x) => *x,
            Data::Mebibytes(x) => *x,
            Data::Gibibytes(x) => *x,
            Data::Tebibytes(x) => *x,
        }
    }
}

impl UnitConversion for Data {
    //! The `UnitConversion` trait is implemented for the Data enum.
    //! This trait provides methods for converting between different units of the same dimension.
    
    type Units = DataUnit;
    
    /// Converts a given instance of a Data enum into the base variant, `Data::Bytes`.
    fn to_base(&self) -> Data {
        match self {
            Data::Bytes(x) => Data::Bytes(*x),
            Data::Kilobytes(x) => Data::Bytes(x * Data::BYTES_PER_KB),
            Data::Megabytes(x) => Data::Bytes(x * Data::BYTES_PER_MB),
            Data::Gigabytes(x) => Data::Bytes(x * Data::BYTES_PER_GB),
            Data::Terabytes(x) => Data::Bytes(x * Data::BYTES_PER_TB),
            Data::Kibibytes(x) => Data::Bytes(x * Data::BYTES_PER_KBI),
            Data::Mebibytes(x) => Data::Bytes(x * Data::BYTES_PER_MBI),
            Data::Gibibytes(x) => Data::Bytes(x * Data::BYTES_PER_GBI),
            Data::Tebibytes(x) => Data::Bytes(x * Data::BYTES_PER_TBI),
        }
    }

    /// Converts a given instance of a Data enum into the specified variant as indicated by
    /// the `DataUnit` provided.
    fn to(&self, variant: DataUnit) -> Data {
        let bytes = match self.to_base() {
            Data::Bytes(x) => x,
            _ => panic!("expected Data::Bytes variant"),
        };
        match variant {
            DataUnit::Bytes => Data::Bytes(bytes),
            DataUnit::Kilobytes | DataUnit::KB => Data::Kilobytes(bytes / Data::BYTES_PER_KB),
            DataUnit::Megabytes | DataUnit::MB => Data::Megabytes(bytes / Data::BYTES_PER_MB),
            DataUnit::Gigabytes | DataUnit::GB => Data::Gigabytes(bytes / Data::BYTES_PER_GB),
            DataUnit::Terabytes | DataUnit::TB => Data::Terabytes(bytes / Data::BYTES_PER_TB),
            DataUnit::Kibibytes | DataUnit::KiB => Data::Kibibytes(bytes / Data::BYTES_PER_KBI),
            DataUnit::Mebibytes | DataUnit::MiB => Data::Mebibytes(bytes / Data::BYTES_PER_MBI),
            DataUnit::Gibibytes | DataUnit::GiB => Data::Gibibytes(bytes / Data::BYTES_PER_GBI),
            DataUnit::Tebibytes | DataUnit::TiB => Data::Tebibytes(bytes / Data::BYTES_PER_TBI),
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Data) -> bool {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).abs() < f64::EPSILON
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Data) -> Option<std::cmp::Ordering> {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).partial_cmp(&0.0)
    }
}


impl Add for Data {
    type Output = Data;

    fn add(self, other: Data) -> Data {
        let a = self.to_base();
        let b = other.to_base();
        let bytes = match a {
            Data::Bytes(x) => match b {
                Data::Bytes(y) => Data::Bytes(x + y),
                _ => panic!("unsupported operation"),
            },
            _ => panic!("expected self to be of variant Data::Bytes"),
        };
        match self {
            Data::Bytes(_) => bytes,
            Data::Kilobytes(_) => bytes.to(DataUnit::Kilobytes),
            Data::Megabytes(_) => bytes.to(DataUnit::Megabytes),
            Data::Gigabytes(_) => bytes.to(DataUnit::Gigabytes),
            Data::Terabytes(_) => bytes.to(DataUnit::Terabytes),
            Data::Kibibytes(_) => bytes.to(DataUnit::Kibibytes),
            Data::Mebibytes(_) => bytes.to(DataUnit::Mebibytes),
            Data::Gibibytes(_) => bytes.to(DataUnit::Gibibytes),
            Data::Tebibytes(_) => bytes.to(DataUnit::Tebibytes),
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, other: Data) -> Data {
        let a = self.to_base();
        let b = other.to_base();
        let bytes = match a {
            Data::Bytes(x) => match b {
                Data::Bytes(y) => Data::Bytes(x - y),
                _ => panic!("unsupported operation"),
            },
            _ => panic!("expected self to be of variant Data::Bytes"),
        };
        match self {
            Data::Bytes(_) => bytes,
            Data::Kilobytes(_) => bytes.to(DataUnit::Kilobytes),
            Data::Megabytes(_) => bytes.to(DataUnit::Megabytes),
            Data::Gigabytes(_) => bytes.to(DataUnit::Gigabytes),
            Data::Terabytes(_) => bytes.to(DataUnit::Terabytes),
            Data::Kibibytes(_) => bytes.to(DataUnit::Kibibytes),
            Data::Mebibytes(_) => bytes.to(DataUnit::Mebibytes),
            Data::Gibibytes(_) => bytes.to(DataUnit::Gibibytes),
            Data::Tebibytes(_) => bytes.to(DataUnit::Tebibytes),
        }
    }
}


impl Mul<f64> for Data {
    type Output = Data;

    fn mul(self, other: f64) -> Data {
        match self {
            Data::Bytes(x) => Data::Bytes(x * other),
            Data::Kilobytes(x) => Data::Kilobytes(x * other),
            Data::Megabytes(x) => Data::Megabytes(x * other),
            Data::Gigabytes(x) => Data::Gigabytes(x * other),
            Data::Terabytes(x) => Data::Terabytes(x * other),
            Data::Kibibytes(x) => Data::Kibibytes(x * other),
            Data::Mebibytes(x) => Data::Mebibytes(x * other),
            Data::Gibibytes(x) => Data::Gibibytes(x * other),
            Data::Tebibytes(x) => Data::Tebibytes(x * other),
        }
    }
}


impl Mul<Data> for f64 {
    type Output = Data;

    fn mul(self, other: Data) -> Data {
        match other {
            Data::Bytes(x) => Data::Bytes(x * self),
            Data::Kilobytes(x) => Data::Kilobytes(x * self),
            Data::Megabytes(x) => Data::Megabytes(x * self),
            Data::Gigabytes(x) => Data::Gigabytes(x * self),
            Data::Terabytes(x) => Data::Terabytes(x * self),
            Data::Kibibytes(x) => Data::Kibibytes(x * self),
            Data::Mebibytes(x) => Data::Mebibytes(x * self),
            Data::Gibibytes(x) => Data::Gibibytes(x * self),
            Data::Tebibytes(x) => Data::Tebibytes(x * self),
        }
    }
}


impl Div<f64> for Data {
    type Output = Data;

    fn div(self, other: f64) -> Data {
        match self {
            Data::Bytes(x) => Data::Bytes(x / other),
            Data::Kilobytes(x) => Data::Kilobytes(x / other),
            Data::Megabytes(x) => Data::Megabytes(x / other),
            Data::Gigabytes(x) => Data::Gigabytes(x / other),
            Data::Terabytes(x) => Data::Terabytes(x / other),
            Data::Kibibytes(x) => Data::Kibibytes(x / other),
            Data::Mebibytes(x) => Data::Mebibytes(x / other),
            Data::Gibibytes(x) => Data::Gibibytes(x / other),
            Data::Tebibytes(x) => Data::Tebibytes(x / other),
        }
    }
}


impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Data::Bytes(x) => write!(f, "{} bytes", x),
            Data::Kilobytes(x) => write!(f, "{} KB", x),
            Data::Megabytes(x) => write!(f, "{} MB", x),
            Data::Gigabytes(x) => write!(f, "{} GB", x),
            Data::Terabytes(x) => write!(f, "{} TB", x),
            Data::Kibibytes(x) => write!(f, "{} KiB", x),
            Data::Mebibytes(x) => write!(f, "{} MiB", x),
            Data::Gibibytes(x) => write!(f, "{} GiB", x),
            Data::Tebibytes(x) => write!(f, "{} TiB", x),
        }
    }
}


/*--------------------( Unit Tests )--------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base() {
        let mem = Data::Kilobytes(128.0);
        let base = mem.to_base();
        match base {
            Data::Bytes(x) => assert_eq!(x, 128.0 * Data::BYTES_PER_KB),
            _ => panic!("expected Data::Bytes variant"),
        }
    }

    #[test]
    fn test_from_base() {
        let mem = Data::Bytes(1024.0);
        match mem {
            Data::Bytes(x) => assert_eq!(x, 1024.0),
            _ => panic!("expected Data::Bytes variant"),
        }
        let as_kb = mem.to(DataUnit::KiB);
        match as_kb {
            Data::Kibibytes(x) => assert_eq!(x, 1.0),
            _ => panic!("expected Data::Kibibytes variant"),
        }
    }

    #[test]
    fn test_eq() {
        let x = Data::Bytes(1024.0);
        let y = Data::KiB(1.0);
        assert_eq!(x, y);
    }

    #[test]
    fn test_gt() {
        let x = Data::Kilobytes(2048.0);
        let y = Data::Megabytes(2.0);
        assert!(x > y);
    }

    #[test]
    fn test_lt() {
        let x = Data::Megabytes(1000.0);
        let y = Data::Gigabytes(1.5);
        assert!(x < y);
    }

    #[test]
    fn test_add_same_units() {
        let x = Data::Megabytes(1000.0);
        let y = Data::Megabytes(1000.0);
        let z = x + y;
        assert_eq!(z, Data::Megabytes(2000.0));
        match z {
            Data::Megabytes(x) => assert_eq!(x, 2000.0),
            _ => panic!("expected Data::Megabytes variant"),
        }
    }

    #[test]
    fn test_add_different_units() {
        let x = Data::Gibibytes(1.0);
        let y = Data::Mebibytes(1024.0);
        let z = x + y;
        assert_eq!(z, Data::Mebibytes(2048.0));
        match z {
            Data::Gibibytes(x) => assert_eq!(x, 2.0),
            _ => panic!("expected Data::Megabytes variant"),
        }
    }

    #[test]
    fn test_sub_same_units() {
        let x = Data::Gigabytes(2.0);
        let y = Data::Gigabytes(1.0);
        let z = x - y;
        assert_eq!(z, Data::Gigabytes(1.0));
        match z {
            Data::Gigabytes(x) => assert_eq!(x, 1.0),
            _ => panic!("expected Data::Gigabytes variant"),
        }
    }

    #[test]
    fn test_sub_different_units() {
        let x = Data::Gibibytes(2.0);
        let y = Data::Mebibytes(1024.0);
        let z = x - y;
        assert_eq!(z, Data::Gibibytes(1.0));
        match z {
            Data::Gibibytes(x) => assert_eq!(x, 1.0),
            _ => panic!("expected Data::Gibibytes variant"),
        }
    }

    #[test]
    fn test_display() {
        let bytes = Data::Bytes(1024.0);
        assert_eq!(format!("{}", bytes), "1024 bytes");
        let kb = Data::Kilobytes(1.0);
        assert_eq!(format!("{}", kb), "1 KB");
        let mb = Data::Megabytes(1.0);
        assert_eq!(format!("{}", mb), "1 MB");
        let gb = Data::Gigabytes(1.0);
        assert_eq!(format!("{}", gb), "1 GB");
        let tb = Data::Terabytes(1.0);
        assert_eq!(format!("{}", tb), "1 TB");
        let kib = Data::Kibibytes(1.0);
        assert_eq!(format!("{}", kib), "1 KiB");
        let mib = Data::Mebibytes(1.0);
        assert_eq!(format!("{}", mib), "1 MiB");
        let gib = Data::Gibibytes(1.0);
        assert_eq!(format!("{}", gib), "1 GiB");
        let tib = Data::Tebibytes(1.0);
        assert_eq!(format!("{}", tib), "1 TiB");
    }

    #[test]
    fn test_multiply_scalar() {
        let memory = Data::GiB(2.0);
        let doubled = memory * 2.0;
        assert_eq!(doubled, Data::MiB(4096.0));

        let memory_2 = Data::GiB(2.0);
        let doubled_2 = 2.0 * memory_2;
        assert_eq!(doubled_2, Data::MiB(4096.0));
    }
}