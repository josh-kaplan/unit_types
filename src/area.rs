

pub enum AreaUnit {
    SqMillimeters,
    SqMeters,
    SqKilometers,
    SqFeet,
}

pub enum Area {
    SqMillimeters(f64),
    SqMeters(f64),
    SqKilometers(f64),
    SqFeet(f64),
}