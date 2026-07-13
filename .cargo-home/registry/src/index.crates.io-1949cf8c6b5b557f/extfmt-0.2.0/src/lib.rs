//! A crate with extended formatting options for ceratin types.

extern crate core;

mod hexdump;
mod slice;

pub use hexdump::*;
pub use slice::*;
