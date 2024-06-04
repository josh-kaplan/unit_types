# Quality

This document describes the steps taken to make the *units* crate 
production-ready and maintain quality. As a brief overview, the following
items are covered:

- Tests & Code Coverage
- Linting with Clippy
- Documentation 
- SBOM
- Cargo Deny

## Tests & Code Coverage

```
cargo test
```

```
cargo tarpaulin --out Html --output-dir target/tarpaulin
```

## Linting with Clippy

```
cargo clippy
```

## Documentation

```
cargo doc --no-deps --open
```

## Cargo Deny

## Cargo SBOM