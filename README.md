# nom-packrat
An extension of [nom](https://docs.rs/nom) to apply "Packrat Parsing".

[![Build Status](https://dev.azure.com/dalance/procs/_apis/build/status/dalance.nom-packratprocs?branchName=master)](https://dev.azure.com/dalance/nom-packrat/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/nom-packrat.svg)](https://crates.io/crates/nom-packrat)
[![Docs.rs](https://docs.rs/nom-packrat/badge.svg)](https://docs.rs/nom-packrat)

## Usage

```Cargo.toml
[dependencies]
nom-packrat = "0.1.0"
```

## Example

```rust
use nom::character::complete::char;
use nom::IResult;
use nom_packrat::{init, packrat_parser, storage};

// Declare storage used by packrat_parser
storage!(String);

// Apply packrat_parser by custom attribute
#[packrat_parser]
pub fn parser(s: &str) -> IResult<&str, String> {
    let (s, x) = char('a')(s)?;
    Ok((s, x.to_string()))
}

fn main() {
    let input = "a";

    // Initialize before parsing
    init!();
    let result = parser(input);

    println!("{:?}", result);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
