EnuMap
=======

[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]
[![docs.rs][docsrs-badge]][docsrs-url]

[crates-badge]: https://img.shields.io/crates/v/enumap.svg
[crates-url]: https://crates.io/crates/enumap
[license-badge]: https://img.shields.io/crates/l/enumap.svg
[license-url]: https://github.com/Dav1dde/enumap/blob/master/LICENSE
[actions-badge]: https://github.com/Dav1dde/enumap/workflows/CI/badge.svg
[actions-url]: https://github.com/Dav1dde/enumap/actions?query=workflow%3ACI+branch%3Amaster
[docsrs-badge]: https://img.shields.io/docsrs/enumap
[docsrs-url]: https://docs.rs/enumap


HashMap like interface for enumerations backed by an array.

`enumap` is `no_std` compatible, dependency and proc macro free for blazingly fast compilation speeds.


## Usage

This crate is [on crates.io](https://crates.io/crates/enumap) and can be
used by adding it to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
enumap = "0.1"
```

## Example

```rust
use enumap::EnumMap;

enumap::enumap! {
    /// A beautiful fruit, ready to be sold.
    #[derive(Debug)]
    enum Fruit {
        Orange,
        Banana,
        Grape,
    }
}

// A fruit shop: fruit -> stock.
let mut shop = EnumMap::new();
shop.insert(Fruit::Orange, 100);
shop.insert(Fruit::Banana, 200);

for (fruit, amount) in &shop {
    println!("There are {amount} {fruit:?}s in stock!");
}

if !shop.contains_key(Fruit::Grape) {
    println!("Sorry no grapes in stock :(");
}
```

Browse the docs for more examples!
