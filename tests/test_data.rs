use unit_types::{Data, Time, DataRate};
use unit_types::prelude::*;


#[test]
fn itest_data() {
    let x1 = Data::Gigabytes(1.0);
    let x2 = Data::Megabytes(1000.0);
    let x3 = Data::Gibibytes(1.0);
    let x4 = Data::Mebibytes(1024.0);

    let x5 = x1 + x1;
    let x6 = x1 + x2;
    assert_eq!(x5, x6);
    match x5 {
        Data::Gigabytes(x) => assert_eq!(x, 2.0),
        _ => panic!("expected Data::Gigabytes variant"),
    }
    match x6 {
        Data::Gigabytes(x) => assert_eq!(x, 2.0),
        _ => panic!("expected Data::Gigabytes variant"),
    }

    let x7 = x3 + x3;
    let x8 = x3 + x4;
    assert_eq!(x7, x8);
    match x7 {
        Data::Gibibytes(x) => assert_eq!(x, 2.0),
        _ => panic!("expected Data::Gibibytes variant"),
    }
    match x8 {
        Data::Gibibytes(x) => assert_eq!(x, 2.0),
        _ => panic!("expected Data::Gibibytes variant"),
    }
    
}


#[test]
fn itest_data_conversation() {
    let memory = Data::GiB(2.0);
    let memory_in_mb = memory.to(unit::MiB).value();
    assert_eq!(memory_in_mb, 2048.0);
}

#[test]
fn itest_data_rates() {
    let data = Data::MB(2048.0);
    let time = Time::Seconds(1.0);
    let rate = data / time;

    // Make sure what we got is in MBps and the value is correct
    match rate {
        DataRate::MBps(x) => assert_eq!(x, 2048.0),
        _ => panic!("expected DataRate::MBps variant"),
    }

    let gibps = rate.to(unit::GiBps).value();
    assert_eq!(gibps, 2.0);
}
