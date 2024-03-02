//! HashMap like interface for enumerations backed by an array.
//!
//! `enumap` is `no_std` compatible and proc macro free for fast compilation speeds.
//!
//! ```
//! use enumap::EnumMap;
//!
//! enumap::enumap! {
//!     /// A beautiful fruit, ready to be sold.
//!     #[derive(Debug)]
//!     enum Fruit {
//!         Orange,
//!         Banana,
//!         Grape,
//!     }
//! }
//!
//! // A fruit shop: fruit -> stock.
//! let mut shop = EnumMap::new();
//! shop.insert(Fruit::Orange, 100);
//! shop.insert(Fruit::Banana, 200);
//!
//! for (fruit, amount) in &shop {
//!     println!("There are {amount} {fruit:?}s in stock!");
//! }
//!
//! if !shop.contains_key(Fruit::Grape) {
//!     println!("Sorry no grapes in stock :(");
//! }
//! ```
//!
//! # Implementing Enum
//!
//! While the crate was built with enums in mind, it is just a generic map
//! implementation backed by an array which only requires a bijective mapping
//! of items to an array index.
//!
//! ```
//! use enumap::{Enum, EnumMap};
//!
//! #[derive(Copy, Clone)]
//! struct ZeroToTen(u8);
//!
//! impl ZeroToTen {
//!     fn new(num: u8) -> Option<Self> {
//!         matches!(num, 0..=9).then_some(Self(num))
//!     }
//! }
//!
//! impl Enum<10> for ZeroToTen {
//!     fn from_index(index: usize) -> Option<Self> {
//!         index.try_into().ok().and_then(Self::new)
//!     }
//!
//!     fn to_index(value: Self) -> usize {
//!         value.0 as usize
//!     }
//! }
//!
//! let zero = ZeroToTen::new(0).unwrap();
//! let five = ZeroToTen::new(5).unwrap();
//! let nine = ZeroToTen::new(9).unwrap();
//!
//! let mut map = EnumMap::from([
//!     (zero, "foo"),
//!     (nine, "bar"),
//! ]);
//!
//! assert_eq!(map[zero], "foo");
//! assert_eq!(map[nine], "bar");
//! assert_eq!(map.get(five), None);
//! ```
//!
//! # Use Niches
//!
//! The map is backed by an array of options `[Option<V>; N]`,
//! consider using values with a gurantueed niche to optimize the size of the map:
//!
//! ```
//! use enumap::EnumMap;
//! use std::num::NonZeroUsize;
//!
//! enumap::enumap! {
//!     #[derive(Debug)]
//!     enum Fruit {
//!         Orange,
//!         Banana
//!     }
//! }
//!
//! assert_eq!(std::mem::size_of::<EnumMap<2, Fruit, usize>>(), 32);
//! assert_eq!(std::mem::size_of::<EnumMap<2, Fruit, NonZeroUsize>>(), 16);
//! ```
//!
#![no_std]

mod enum_macro;

pub mod map;

pub use self::map::EnumMap;

/// Enum type, usually implemented using the [`enumap`] macro.
///
/// Any enumeration used by [`EnumMap`] must implement this trait.
///
/// Failures in implementing the trait will not result in undefined behaviour
/// but may result in panics and invalid results.
pub trait Enum<const LENGTH: usize>: Copy + Sized {
    /// Length of the enum.
    ///
    /// Equivalent to the const generic length.
    const LENGTH: usize = LENGTH;

    /// Converts an index to an enum variant.
    ///
    /// Passed `index` must be in range `0..LENGTH`.
    fn from_index(index: usize) -> Option<Self>;

    /// Converts an enum variant to an index.
    ///
    /// Returned index must be in range `0..LENGTH`.
    fn to_index(value: Self) -> usize;
}