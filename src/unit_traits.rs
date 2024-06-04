pub trait UnitConversion {
    type Units;

    /// It is up to the implementor to decide which variant of the unit is the base unit.
    fn to_base(&self) -> Self;

    /// Converts between units of the same dimension.
    fn to(&self, unit: Self::Units) -> Self;
}

pub trait Unit {
    fn value(&self) -> f64;
}