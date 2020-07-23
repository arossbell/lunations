# Lunation Date Calculator [![](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saurvs/astro-rust/blob/master/LICENSE.md)
This library is only a couple of functions that are related: something to number lunations (back and forth).
**It's not hour-precise by itself.** It will land within +/- 6 hours.

The equation was derived from [this Wolfram page](https://scienceworld.wolfram.com/astronomy/Lunation.html) ([archive link](https://web.archive.org/web/20181105235912/https://scienceworld.wolfram.com/astronomy/Lunation.html)) to give me *some sort* of lunation numbering.

It looks like lunation #0 is the 1993-05-21 lunation (lunation #871 with a year 1923 epoch).

## Usage Example
Make your Cargo.toml do something like this:
```toml
[dependencies]
lunations = {git = "https://github.com/arossbell/lunations"}
```

Inclusion:
```rust
extern crate lunations;

use lunations::*;
```

JD to lunation (f64 -> f64):
```rust
println!("{}", jd_lunation(2450744.365));
```
*(Prints `54.715299381818`)*

Lunation to JD (f64 -> f64):
```rust
println!("{}", lunation_jd(55.0));
```
*(Prints `2450752.77237685`)*

To make the JD of a lunation more precise, you could use the [`time_of_phase`](https://saurvs.github.io/astro-rust/astro/lunar/fn.time_of_phase.html) function from the [astro-rust](https://github.com/arossbell/astro-rust) crate.

## Dependencies:
- This code isn't (yet) that sophisticated.
