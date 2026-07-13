//! This library provides utility traits for data types that can be used
//! to define ranges of values. Default implementations exists for
//! primitive numeric types and `char`.
//! The defined traits can help define basic tasks on ranges such as
//! enumerating the elements of the range (`PartialEnum`),
//! measure the size of the range (`Measure`), etc.
//!
//! Its primary use is through the [`btree-range-map`](https://crates.io/crates/btree-range-map) crate
//! that define data-structures indexed by ranges.
//! By implementing the traits defined in here, one can extend the type of
//! ranges supported by `btree-range-map`, without necessarily depending on it.
#![no_std]

mod bounded;
mod enumerable;
mod measure;

pub use bounded::*;
pub use enumerable::*;
pub use measure::*;
