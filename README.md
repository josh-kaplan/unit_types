# Unit Types

> **DO NOT USE IN PRODUCTION!** This is a learning project and is not intended for production use. Some of the conversion numbers were automatically filled out by GitHub Copilot and have not yet been validated.
> 
> I have not yet determined how far I will take this project. Forks or 
> contributions are welcome.

The `unit_types` crate is a dimensionally aware type system for Rust.


**Example:**

```rust
use unit_types::prelude::*; // brings in "unit"
use unit_types::Length;
let x = Length::Kilometers(1.0);
let y = Length::Meters(200.0);
assert_eq!(x - y, Length::Meters(800.0));
assert_eq!(x + y, Length::Meters(1200.0));

let z = 2.0 * (x + y); // should be in Kilometers
assert_eq!(z.value(), 2.4);
assert_eq!(z.to(unit::Meters).value(), 2400.0);
```