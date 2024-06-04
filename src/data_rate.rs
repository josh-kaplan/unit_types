use crate::prelude::UnitConversion;
use crate::prelude::Unit;
use crate::Data;
use crate::Time;

#[derive(Clone,Copy,Debug)]
#[allow(non_camel_case_types)]
pub enum DataRateUnit {
    bps,
    Kbps,
    KBps,
    Kibps,
    KiBps,
    Mbps,
    MBps,
    Mibps,
    MiBps,
    Gbps,
    GBps,
    Gibps,
    GiBps,
    Tbps,
    TBps,
    Tibps,
    TiBps,
}


#[derive(Clone,Copy,Debug)]
#[allow(non_camel_case_types)]
pub enum DataRate {
    bps(f64),
    Kbps(f64),
    KBps(f64),
    Kibps(f64),
    KiBps(f64),
    Mbps(f64),
    MBps(f64),
    Mibps(f64),
    MiBps(f64),
    Gbps(f64),
    GBps(f64),
    Gibps(f64),
    GiBps(f64),
    Tbps(f64),
    TBps(f64),
    Tibps(f64),
    TiBps(f64),
}

impl Unit for DataRate {
    fn value(&self) -> f64 {
        match self {
            DataRate::bps(x)   => *x,
            DataRate::Kbps(x)  => *x,
            DataRate::KBps(x)  => *x,
            DataRate::Kibps(x) => *x,
            DataRate::KiBps(x) => *x,
            DataRate::Mbps(x)  => *x,
            DataRate::MBps(x)  => *x,
            DataRate::Mibps(x) => *x,
            DataRate::MiBps(x) => *x,
            DataRate::Gbps(x)  => *x,
            DataRate::GBps(x)  => *x,
            DataRate::Gibps(x) => *x,
            DataRate::GiBps(x) => *x,
            DataRate::Tbps(x)  => *x,
            DataRate::TBps(x)  => *x,
            DataRate::Tibps(x) => *x,
            DataRate::TiBps(x) => *x,
        }
    }

}

impl UnitConversion for DataRate {
    type Units = DataRateUnit;

    fn to_base(&self) -> DataRate {
        match self {
            DataRate::bps(x)   => DataRate::bps(*x),
            DataRate::Kbps(x)  => DataRate::bps(x * 1000.0),
            DataRate::KBps(x)  => DataRate::bps(x * 8000.0),
            DataRate::Kibps(x) => DataRate::bps(x * 1024.0 * 1000.0),
            DataRate::KiBps(x) => DataRate::bps(x * 1024.0 * 8.0),
            DataRate::Mbps(x)  => DataRate::bps(x * 1000.0 * 1000.0),
            DataRate::MBps(x)  => DataRate::bps(x * 8000.0 * 1000.0),
            DataRate::Mibps(x) => DataRate::bps(x * 1024.0 * 1000.0 * 1000.0),
            DataRate::MiBps(x) => DataRate::bps(x * 1024.0 * 8.0 * 1000.0),
            DataRate::Gbps(x)  => DataRate::bps(x * 1000.0 * 1000.0 * 1000.0),
            DataRate::GBps(x)  => DataRate::bps(x * 8000.0 * 1000.0 * 1000.0),
            DataRate::Gibps(x) => DataRate::bps(x * 1024.0 * 1000.0 * 1000.0 * 1000.0),
            DataRate::GiBps(x) => DataRate::bps(x * 1024.0 * 8.0 * 1000.0 * 1000.0),
            DataRate::Tbps(x)  => DataRate::bps(x * 1000.0 * 1000.0 * 1000.0 * 1000.0),
            DataRate::TBps(x)  => DataRate::bps(x * 8000.0 * 1000.0 * 1000.0 * 1000.0),
            DataRate::Tibps(x) => DataRate::bps(x * 1024.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0),
            DataRate::TiBps(x) => DataRate::bps(x * 1024.0 * 8.0 * 1000.0 * 1000.0 * 1000.0),
        }
    }

    fn to(&self, other: DataRateUnit) -> DataRate {
        let base = self.to_base().value();
        match other {
            DataRateUnit::bps => DataRate::bps(base),
            DataRateUnit::Kbps => DataRate::Kbps(base / 1000.0),
            DataRateUnit::KBps => DataRate::KBps(base / 8000.0),
            DataRateUnit::Kibps => DataRate::Kibps(base / (1024.0 * 1000.0)),
            DataRateUnit::KiBps => DataRate::KiBps(base / (1024.0 * 8.0)),
            DataRateUnit::Mbps => DataRate::Mbps(base / (1000.0 * 1000.0)),
            DataRateUnit::MBps => DataRate::MBps(base / (8000.0 * 1000.0)),
            DataRateUnit::Mibps => DataRate::Mibps(base / (1024.0 * 1000.0 * 1000.0)),
            DataRateUnit::MiBps => DataRate::MiBps(base / (1024.0 * 8.0 * 1000.0)),
            DataRateUnit::Gbps => DataRate::Gbps(base / (1000.0 * 1000.0 * 1000.0)),
            DataRateUnit::GBps => DataRate::GBps(base / (8000.0 * 1000.0 * 1000.0)),
            DataRateUnit::Gibps => DataRate::Gibps(base / (1024.0 * 1000.0 * 1000.0 * 1000.0)),
            DataRateUnit::GiBps => DataRate::GiBps(base / (1024.0 * 8.0 * 1000.0 * 1000.0)),
            DataRateUnit::Tbps => DataRate::Tbps(base / (1000.0 * 1000.0 * 1000.0 * 1000.0)),
            DataRateUnit::TBps => DataRate::TBps(base / (8000.0 * 1000.0 * 1000.0 * 1000.0)),
            DataRateUnit::Tibps => DataRate::Tibps(base / (1024.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0)),
            DataRateUnit::TiBps => DataRate::TiBps(base / (1024.0 * 8.0 * 1000.0 * 1000.0 * 1000.0)),
        }
    }
}

impl std::ops::Div<Time> for Data {
    type Output = DataRate;

    fn div(self, rhs: Time) -> DataRate {
        let sec = rhs.to_base().value();

        match self {
            Data::Bytes(x) => DataRate::bps(8.0 * x / sec),
            Data::Kilobytes(x) => DataRate::KBps(x / sec),
            Data::Megabytes(x) => DataRate::MBps(x / sec),
            Data::Gigabytes(x) => DataRate::GBps(x / sec),
            Data::Terabytes(x) => DataRate::TBps(x / sec),
            Data::Kibibytes(x) => DataRate::KiBps(x / sec),
            Data::Mebibytes(x) => DataRate::MiBps(x / sec),
            Data::Gibibytes(x) => DataRate::GiBps(x / sec),
            Data::Tebibytes(x) => DataRate::TiBps(x / sec),
        }
    }
}



