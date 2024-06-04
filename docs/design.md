

## Structure & Organization

**Each modules contains a types of measurement.** For example, the `length.rs`
module contains everything related to length measurements. 

**Types are enums wrapping a single `f64` value.** There is an enum type for 
each type of measure. For example, the `data.rs` module contains the `Data`
enum which consists of `Byte`, `Kilobyte`, `Megabyte`, `Gigabyte`,

**Unit names are enums without a value.** The SHOULD have one-to-one alignment
with the type enums and MAY also include unit aliases.


## Measures and Derived Measures

- [x] Data
- [x] Data Rate
- [ ] Length
- [ ] Area
- [ ] Volume
- [X] Time
- [ ] Speed
- [ ] Acceleration
- [ ] Mass
- [ ] Force
- [ ] Pressure
- [ ] Work / Energy
- [ ] Power
- [ ] Temperature
- [ ] Frequency
- [ ] Anglar Distance
- [ ] Angular Speed
- [ ] Angular Acceleration
