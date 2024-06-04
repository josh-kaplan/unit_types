use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;
use crate::unit_traits::UnitConversion;
use crate::unit_traits::Unit;


#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum LengthUnit {
    Millimeters,
    mm,
    Centimeters,
    cm,
    Meters,
    m,
    Kilometers,
    km,
    Feet,
    ft,
    Inches,
    inch,
    Yards,
    yd,
    NauticalMiles,
    nmi,
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Millimeters(f64),
    Centimeters(f64),
    Meters(f64),
    Kilometers(f64),
    Feet(f64),
    Inches(f64),
    Yards(f64),
    NauticalMiles(f64),
}

impl Unit for Length {
    fn value(&self) -> f64 {
        match self {
            Length::Millimeters(val)   => *val,
            Length::Centimeters(val)   => *val,
            Length::Meters(val)        => *val,
            Length::Kilometers(val)    => *val,
            Length::Feet(val)          => *val,
            Length::Inches(val)        => *val,
            Length::Yards(val)         => *val,
            Length::NauticalMiles(val) => *val,
        }
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Length) -> bool {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).abs() < f64::EPSILON
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Length) -> Option<std::cmp::Ordering> {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).partial_cmp(&0.0)
    }
}

impl UnitConversion for Length {
    type Units = LengthUnit;

    fn to_base(&self) -> Length {
        let value = match self {
            Length::Millimeters(val)   => val * 1000.0,
            Length::Centimeters(val)   => val * 10.0,
            Length::Meters(val)        => *val,
            Length::Kilometers(val)    => val * 1_000.0,
            Length::Feet(val)          => val / 3.28084,
            Length::Inches(val)        => val / 39.3701,
            Length::Yards(val)         => val / 1.09361,
            Length::NauticalMiles(val) => val * 1_852.0,
        };
        Length::Meters(value)
    }

    fn to(&self, variant: LengthUnit) -> Length {
        use LengthUnit::*;
        let base = self.to_base().value();
        match variant {
            Millimeters   | mm   => Length::Millimeters(base * 1000.0),
            Centimeters   | cm   => Length::Centimeters(base * 10.0),
            Meters        | m    => Length::Meters(base),
            Kilometers    | km   => Length::Kilometers(base / 1_000.0),
            Feet          | ft   => Length::Feet(base * 3.28084),
            Inches        | inch => Length::Inches(base * 39.3701),
            Yards         | yd   => Length::Yards(base * 1.09361),
            NauticalMiles | nmi  => Length::NauticalMiles(base / 1_852.0),
        }
    }
}


impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Length::Millimeters(val)   => write!(f, "{} mm", val),
            Length::Centimeters(val)   => write!(f, "{} cm", val),
            Length::Meters(val)        => write!(f, "{} m", val),
            Length::Kilometers(val)    => write!(f, "{} km", val),
            Length::Feet(val)          => write!(f, "{} ft", val),
            Length::Inches(val)        => write!(f, "{} in", val),
            Length::Yards(val)         => write!(f, "{} yd", val),
            Length::NauticalMiles(val) => write!(f, "{} nmi", val),
        }
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        let other_val = match self {
            Length::Millimeters(_) => other.to(LengthUnit::Millimeters).value(),
            Length::Centimeters(_) => other.to(LengthUnit::Centimeters).value(),
            Length::Meters(_)      => other.to(LengthUnit::Meters).value(),
            Length::Kilometers(_)  => other.to(LengthUnit::Kilometers).value(),
            Length::Feet(_)        => other.to(LengthUnit::Feet).value(),
            Length::Inches(_)      => other.to(LengthUnit::Inches).value(),
            Length::Yards(_)       => other.to(LengthUnit::Yards).value(),
            Length::NauticalMiles(_) => other.to(LengthUnit::NauticalMiles).value(),
        };
        match self {
            Length::Millimeters(val) => Length::Millimeters(val + other_val),
            Length::Centimeters(val) => Length::Centimeters(val + other_val),
            Length::Meters(val)      => Length::Meters(val + other_val),
            Length::Kilometers(val)  => Length::Kilometers(val + other_val),
            Length::Feet(val)        => Length::Feet(val + other_val),
            Length::Inches(val)      => Length::Inches(val + other_val),
            Length::Yards(val)       => Length::Yards(val + other_val),
            Length::NauticalMiles(val) => Length::NauticalMiles(val + other_val),
        }
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        let other_val = match self {
            Length::Millimeters(_) => other.to(LengthUnit::Millimeters).value(),
            Length::Centimeters(_) => other.to(LengthUnit::Centimeters).value(),
            Length::Meters(_)      => other.to(LengthUnit::Meters).value(),
            Length::Kilometers(_)  => other.to(LengthUnit::Kilometers).value(),
            Length::Feet(_)        => other.to(LengthUnit::Feet).value(),
            Length::Inches(_)      => other.to(LengthUnit::Inches).value(),
            Length::Yards(_)       => other.to(LengthUnit::Yards).value(),
            Length::NauticalMiles(_) => other.to(LengthUnit::NauticalMiles).value(),
        };
        match self {
            Length::Millimeters(val) => Length::Millimeters(val - other_val),
            Length::Centimeters(val) => Length::Centimeters(val - other_val),
            Length::Meters(val)      => Length::Meters(val - other_val),
            Length::Kilometers(val)  => Length::Kilometers(val - other_val),
            Length::Feet(val)        => Length::Feet(val - other_val),
            Length::Inches(val)      => Length::Inches(val - other_val),
            Length::Yards(val)       => Length::Yards(val - other_val),
            Length::NauticalMiles(val) => Length::NauticalMiles(val - other_val),
        }
    }
}

impl Mul<f64> for Length {
    type Output = Length;

    fn mul(self, rhs: f64) -> Length {
        match self {
            Length::Millimeters(val) => Length::Millimeters(val * rhs),
            Length::Centimeters(val) => Length::Centimeters(val * rhs),
            Length::Meters(val)      => Length::Meters(val * rhs),
            Length::Kilometers(val)  => Length::Kilometers(val * rhs),
            Length::Feet(val)        => Length::Feet(val * rhs),
            Length::Inches(val)      => Length::Inches(val * rhs),
            Length::Yards(val)       => Length::Yards(val * rhs),
            Length::NauticalMiles(val) => Length::NauticalMiles(val * rhs),
        }
    }
}

impl Mul<Length> for f64 {
    type Output = Length;

    fn mul(self, rhs: Length) -> Length {
        match rhs {
            Length::Millimeters(val) => Length::Millimeters(val * self),
            Length::Centimeters(val) => Length::Centimeters(val * self),
            Length::Meters(val)      => Length::Meters(val * self),
            Length::Kilometers(val)  => Length::Kilometers(val * self),
            Length::Feet(val)        => Length::Feet(val * self),
            Length::Inches(val)      => Length::Inches(val * self),
            Length::Yards(val)       => Length::Yards(val * self),
            Length::NauticalMiles(val) => Length::NauticalMiles(val * self),
        }
    }
}


impl Div<f64> for Length {
    type Output = Length;

    fn div(self, rhs: f64) -> Length {
        match self {
            Length::Millimeters(val) => Length::Millimeters(val / rhs),
            Length::Centimeters(val) => Length::Centimeters(val / rhs),
            Length::Meters(val)      => Length::Meters(val / rhs),
            Length::Kilometers(val)  => Length::Kilometers(val / rhs),
            Length::Feet(val)        => Length::Feet(val / rhs),
            Length::Inches(val)      => Length::Inches(val / rhs),
            Length::Yards(val)       => Length::Yards(val / rhs),
            Length::NauticalMiles(val) => Length::NauticalMiles(val / rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let x1 = Length::Meters(1.0);
        let x2 = Length::Meters(1000.0);
        let x3 = Length::Kilometers(1.0);

        let x4 = x1 + x1;
        let x5 = x1 + x2;
        assert_eq!(x2, x3);
        match x4 {
            Length::Meters(x) => assert_eq!(x, 2.0),
            _ => panic!("expected Length::Meters variant"),
        }
        match x5 {
            Length::Meters(x) => assert_eq!(x, 1001.0),
            _ => panic!("expected Length::Meters variant"),
        }

        let x6 = x2 + x3;
        let x7 = x3 + x2;
        match x6 {
            Length::Meters(x) => assert_eq!(x, 2000.0),
            _ => panic!("expected Length::Meters variant"),
        }
        match x7 {
            Length::Kilometers(x) => assert_eq!(x, 2.0),
            _ => panic!("expected Length::Meters variant"),
        }
    }

    #[test]
    fn itest_length_conversation() {
        let length = Length::Meters(2.0);
        let length_in_km = length.to(LengthUnit::Kilometers).value();
        assert_eq!(length_in_km, 0.002);
    }
}