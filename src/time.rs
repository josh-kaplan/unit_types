use std::ops::{Add, Sub, Mul, Div};

use crate::prelude::UnitConversion;
use crate::prelude::Unit;

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Years,
}

#[derive(Debug, Copy, Clone)]
pub enum Time {
    Seconds(f64),
    Minutes(f64),
    Hours(f64),
    Days(f64),
    Years(f64),
}


impl Time {
    pub const SECONDS_PER_MINUTE: f64 = 60.0;
    pub const SECONDS_PER_HOUR: f64 = 3600.0;
    pub const SECONDS_PER_DAY: f64 = 86400.0;
    pub const SECONDS_PER_YEAR: f64 = 31536000.0;
}


impl Unit for Time {
    fn value(&self) -> f64 {
        match self {
            Time::Seconds(x) => *x,
            Time::Minutes(x) => *x,
            Time::Hours(x) => *x,
            Time::Days(x) => *x,
            Time::Years(x) => *x,
        }
    }
}


impl UnitConversion for Time {
    type Units = TimeUnit;

    fn to_base(&self) -> Time {
        match self {
            Time::Seconds(x) => Time::Seconds(*x),
            Time::Minutes(x) => Time::Seconds(x * Time::SECONDS_PER_MINUTE),
            Time::Hours(x) => Time::Seconds(x * Time::SECONDS_PER_HOUR),
            Time::Days(x) => Time::Seconds(x * Time::SECONDS_PER_DAY),
            Time::Years(x) => Time::Seconds(x * Time::SECONDS_PER_YEAR),
        }
    }

    fn to(&self, unit: TimeUnit) -> Time {
        let base = self.to_base();
        match unit {
            TimeUnit::Seconds => base,
            TimeUnit::Minutes => Time::Minutes(base.value() / Time::SECONDS_PER_MINUTE),
            TimeUnit::Hours => Time::Hours(base.value() / Time::SECONDS_PER_HOUR),
            TimeUnit::Days => Time::Days(base.value() / Time::SECONDS_PER_DAY),
            TimeUnit::Years => Time::Years(base.value() / Time::SECONDS_PER_YEAR),
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).abs() < f64::EPSILON
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        let a = self.to_base().value();
        let b = other.to_base().value();
        (a - b).partial_cmp(&0.0)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        match self {
            Time::Seconds(x) => match other {
                Time::Seconds(y) => Time::Seconds(x + y),
                Time::Minutes(y) => Time::Seconds(x + y * Time::SECONDS_PER_MINUTE),
                Time::Hours(y)   => Time::Seconds(x + y * Time::SECONDS_PER_HOUR),
                Time::Days(y)    => Time::Seconds(x + y * Time::SECONDS_PER_DAY),
                Time::Years(y)   => Time::Seconds(x + y * Time::SECONDS_PER_YEAR),
            },
            Time::Minutes(x) => match other {
                Time::Seconds(y) => Time::Minutes(x + y / Time::SECONDS_PER_MINUTE),
                Time::Minutes(y) => Time::Minutes(x + y),
                Time::Hours(y)   => Time::Minutes(x + y * 60.0),
                Time::Days(y)    => Time::Minutes(x + y * 60.0 * 24.0),
                Time::Years(y)   => Time::Minutes(x + y * 60.0 * 24.0 * 365.0),
            },
            Time::Hours(x) => match other {
                Time::Seconds(y) => Time::Hours(x + y / 3600.0),
                Time::Minutes(y) => Time::Hours(x + y / 60.0),
                Time::Hours(y)   => Time::Hours(x + y),
                Time::Days(y)    => Time::Hours(x + y * 24.0),
                Time::Years(y)   => Time::Hours(x + y * 24.0 * 365.0),
            },
            Time::Days(x) => match other {
                Time::Seconds(y) => Time::Days(x + y / 86400.0),
                Time::Minutes(y) => Time::Days(x + y / 1440.0),
                Time::Hours(y)   => Time::Days(x + y / 24.0),
                Time::Days(y)    => Time::Days(x + y),
                Time::Years(y)   => Time::Days(x + y * 365.0),
            },
            Time::Years(x) => match other {
                Time::Seconds(y) => Time::Years(x + y / (365.0 * 24.0 * 3600.0)),
                Time::Minutes(y) => Time::Years(x + y / (365.0 * 24.0 * 60.0)),
                Time::Hours(y)   => Time::Years(x + y / (365.0 * 24.0)),
                Time::Days(y)    => Time::Years(x + y / 365.0),
                Time::Years(y)   => Time::Years(x + y),
            },
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        match self {
            Time::Seconds(x) => match other {
                Time::Seconds(y) => Time::Seconds(x - y),
                Time::Minutes(y) => Time::Seconds(x - y * 60.0),
                Time::Hours(y)   => Time::Seconds(x - y * 3600.0),
                Time::Days(y)    => Time::Seconds(x - y * 3600.0 * 24.0),
                Time::Years(y)   => Time::Seconds(x - y * 3600.0 * 24.0  * 365.0),
            },
            Time::Minutes(x) => match other {
                Time::Seconds(y) => Time::Minutes(x - y / 60.0),
                Time::Minutes(y) => Time::Minutes(x - y),
                Time::Hours(y)   => Time::Minutes(x - y * 60.0),
                Time::Days(y)    => Time::Minutes(x - y * 60.0 * 24.0),
                Time::Years(y)   => Time::Minutes(x - y * 60.0 * 24.0 * 365.0),
            },
            Time::Hours(x) => match other {
                Time::Seconds(y) => Time::Hours(x - y / 3600.0),
                Time::Minutes(y) => Time::Hours(x - y / 60.0),
                Time::Hours(y)   => Time::Hours(x - y),
                Time::Days(y)    => Time::Hours(x - y * 24.0),
                Time::Years(y)   => Time::Hours(x - y * 24.0 * 365.0),
            },
            Time::Days(x) => match other {
                Time::Seconds(y) => Time::Days(x - y / 86400.0),
                Time::Minutes(y) => Time::Days(x - y / 1440.0),
                Time::Hours(y)   => Time::Days(x - y / 24.0),
                Time::Days(y)    => Time::Days(x - y),
                Time::Years(y)   => Time::Days(x - y * 365.0),
            },
            Time::Years(x) => match other {
                Time::Seconds(y) => Time::Years(x - y / (365.0 * 24.0 * 3600.0)),
                Time::Minutes(y) => Time::Years(x - y / (365.0 * 24.0 * 60.0)),
                Time::Hours(y)   => Time::Years(x - y / (365.0 * 24.0)),
                Time::Days(y)    => Time::Years(x - y / 365.0),
                Time::Years(y)   => Time::Years(x - y),
            },
        }
    }
}

impl Mul<f64> for Time {
    type Output = Time;

    fn mul(self, other: f64) -> Time {
        match self {
            Time::Seconds(x) => Time::Seconds(x * other),
            Time::Minutes(x) => Time::Minutes(x * other),
            Time::Hours(x) => Time::Hours(x * other),
            Time::Days(x) => Time::Days(x * other),
            Time::Years(x) => Time::Years(x * other),
        }
    }
}

impl Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, other: Time) -> Time {
        match other {
            Time::Seconds(x) => Time::Seconds(x * self),
            Time::Minutes(x) => Time::Minutes(x * self),
            Time::Hours(x) => Time::Hours(x * self),
            Time::Days(x) => Time::Days(x * self),
            Time::Years(x) => Time::Years(x * self),
        }
    }
}


impl Div<f64> for Time {
    type Output = Time;

    fn div(self, other: f64) -> Time {
        match self {
            Time::Seconds(x) => Time::Seconds(x / other),
            Time::Minutes(x) => Time::Minutes(x / other),
            Time::Hours(x) => Time::Hours(x / other),
            Time::Days(x) => Time::Days(x / other),
            Time::Years(x) => Time::Years(x / other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_time_ops() {
        let x1 = Time::Seconds(1.0);
        let x2 = Time::Minutes(1.0);
        let x3 = Time::Hours(1.0);
        let x4 = Time::Days(1.0);
        let x5 = Time::Years(1.0);

        let x6 = x1 + x2;
        let x7 = x1 + x3;
        let x8 = x1 + x4;
        let x9 = x1 + x5;

        assert_eq!(x6, Time::Seconds(61.0));
        assert_eq!(x7, Time::Seconds(3601.0));
        assert_eq!(x8, Time::Seconds(86401.0));
        assert_eq!(x9, Time::Seconds(31536001.0));

        let x10 = x1 - x2;
        let x11 = x1 - x3;
        let x12 = x1 - x4;
        let x13 = x1 - x5;

        assert_eq!(x10, Time::Seconds(-59.0));
        assert_eq!(x11, Time::Seconds(-3599.0));
        assert_eq!(x12, Time::Seconds(-86399.0));
        assert_eq!(x13, Time::Seconds(-31535999.0));

        let x14 = x1 * 2.0;
        let x15 = 2.0 * x1;
        let x16 = x1 / 2.0;

        assert_eq!(x14, Time::Seconds(2.0));
        assert_eq!(x15, Time::Seconds(2.0));
        assert_eq!(x16, Time::Seconds(0.5));
    }

    #[test]
    fn test_conversions() {
        let x1 = Time::Seconds(1.0);
        let x2 = Time::Minutes(1.0);
        let x3 = Time::Hours(1.0);
        let x4 = Time::Days(1.0);
        let x5 = Time::Years(1.0);

        assert_eq!(x2.value(), 1.0);
        assert_eq!(x3.value(), 1.0);
        assert_eq!(x4.value(), 1.0);
        assert_eq!(x5.value(), 1.0);
        assert_eq!(x2.to_base().value(), 60.0);
        assert_eq!(x3.to_base().value(), 60.0 * 60.0);
        assert_eq!(x4.to_base().value(), 60.0 * 60.0 * 24.0);
        assert_eq!(x5.to_base().value(), 60.0 * 60.0 * 24.0 * 365.0);
        assert_eq!(x2.to(TimeUnit::Seconds).value(), 60.0);

        let x6 = x1.to(TimeUnit::Minutes);
        let x7 = x1.to(TimeUnit::Hours);
        let x8 = x1.to(TimeUnit::Days);
        let x9 = x1.to(TimeUnit::Years);

        assert_eq!(x6, Time::Minutes(1.0 / 60.0));
        assert_eq!(x7, Time::Hours(1.0 / 3600.0));
        assert_eq!(x8, Time::Days(1.0 / 86400.0));
        assert_eq!(x9, Time::Years(1.0 / 31536000.0));
    }

    #[test]
    fn test_comparison() {
        let x1 = Time::Seconds(1.0);
        let x2 = Time::Minutes(1.0);
        let x3 = Time::Hours(1.0);
        let x4 = Time::Days(1.0);
        let x5 = Time::Years(1.0);

        assert!(x1 < x2);
        assert!(x2 < x3);
        assert!(x3 < x4);
        assert!(x4 < x5);
    }

    #[test]
    fn test_multiply() {
        let sec = Time::Seconds(1.0);
        let min = Time::Minutes(1.0);
        let hrs = Time::Hours(1.0);
        let day = Time::Days(1.0);
        let yrs = Time::Years(1.0);

        let sec2 = sec * 2.0;
        let min2 = min * 2.0;
        let hrs2 = hrs * 2.0;
        let day2 = day * 2.0;
        let yrs2 = yrs * 2.0;

        assert_eq!(sec2, Time::Seconds(2.0));
        assert_eq!(min2, Time::Minutes(2.0));
        assert_eq!(hrs2, Time::Hours(2.0));
        assert_eq!(day2, Time::Days(2.0));
        assert_eq!(yrs2, Time::Years(2.0));

        let sec3 = 3.0 * sec;
        let min3 = 3.0 * min;
        let hrs3 = 3.0 * hrs;
        let day3 = 3.0 * day;
        let yrs3 = 3.0 * yrs;

        assert_eq!(sec3, Time::Seconds(3.0));
        assert_eq!(min3, Time::Minutes(3.0));
        assert_eq!(hrs3, Time::Hours(3.0));
        assert_eq!(day3, Time::Days(3.0));
        assert_eq!(yrs3, Time::Years(3.0));
    }

    #[test]
    fn test_divide() {
        let sec = Time::Seconds(1.0);
        let min = Time::Minutes(1.0);
        let hrs = Time::Hours(1.0);
        let day = Time::Days(1.0);
        let yrs = Time::Years(1.0);

        let sec2 = sec / 2.0;
        let min2 = min / 2.0;
        let hrs2 = hrs / 2.0;
        let day2 = day / 2.0;
        let yrs2 = yrs / 2.0;

        assert_eq!(sec2, Time::Seconds(0.5));
        assert_eq!(min2, Time::Minutes(0.5));
        assert_eq!(hrs2, Time::Hours(0.5));
        assert_eq!(day2, Time::Days(0.5));
        assert_eq!(yrs2, Time::Years(0.5));
    }

    #[test]
    fn test_add() {
        let sec = Time::Seconds(60.0 * 60.0 * 24.0);
        let min = Time::Minutes(60.0 * 24.0);
        let hrs = Time::Hours(24.0);
        let day = Time::Days(1.0);
        let yrs = Time::Years(1.0 / 365.0);

        let times = [sec, min, hrs, day, yrs];

        for i in 0..times.len() {
            for j in 0..times.len() {
                let x = times[i];
                let y = times[j];
                let z = x + y;
                assert_eq!(z.to_base().value(), 2.0 * 60.0 * 60.0 * 24.0);
            }
        }
    }

    #[test]
    fn test_sub() {
        let sec = Time::Seconds(2.0 * 60.0 * 60.0 * 24.0);
        let min = Time::Minutes(2.0 * 60.0 * 24.0);
        let hrs = Time::Hours(2.0 * 24.0);
        let day = Time::Days(2.0 * 1.0);
        let val = utils::round(2.0 * 1.0 / 365.0, 20);
        let yrs = Time::Years(val);

        let times = [sec, min, hrs, day, yrs];

        for i in 0..times.len() {
            for j in 0..times.len() {
                let x = times[i];
                let y = times[j];
                let z = x - (y / 2.0);
                assert_eq!(utils::round(z.to_base().value(), 1), 60.0 * 60.0 * 24.0);
            }
        }

    }
}