# tomb

Tomb is a minimal crate that provides dice rolling mechanisms for games.

[![Rust Checks](https://github.com/matanlurey/tomb/actions/workflows/rust.yml/badge.svg)](https://github.com/matanlurey/tomb/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/matanlurey/tomb/badge.svg)](https://coveralls.io/github/matanlurey/tomb)
[![Current Crates.io Version](https://img.shields.io/crates/v/tomb.svg)](https://crates.io/crates/tomb)
[![Docs](https://docs.rs/tomb/badge.svg)](https://docs.rs/tomb/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Examples

```rs
// Creating and rolling a D20

// Optional dependency, exclude to implement your own RNG.
use fastrand::Rng;

// It is possible to define your own dice, rollers, and to use immutable die as well!
use tomb::{D20, RngRoller, RollMut};

// Pre-defined seed so the result of this example is predictable.
let roller = RngRoller::from(Rng::with_seed(7194422452970863838));
let mut d20 = D20::new();

roller.roll_mut(&mut d20);
assert_eq!(d20.value(), 10);
```
