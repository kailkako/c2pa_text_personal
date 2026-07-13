use traitful::extend;

use crate::result::FullResult;

/// Big endian writer extension trait
#[extend]
pub trait Write: crate::Write {
    /// Write out a big endian encoded 2-byte unsigned integer.
    fn u16(&mut self, int: u16) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 4-byte unsigned integer.
    fn u32(&mut self, int: u32) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 8-byte unsigned integer.
    fn u64(&mut self, int: u64) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 16-byte unsigned integer.
    fn u128(&mut self, int: u128) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 2-byte signed integer.
    fn i16(&mut self, int: i16) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 4-byte signed integer.
    fn i32(&mut self, int: i32) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 8-byte signed integer.
    fn i64(&mut self, int: i64) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 16-byte signed integer.
    fn i128(&mut self, int: i128) -> FullResult {
        self.bytes(int.to_be_bytes())
    }

    /// Write out a big endian encoded 32-bit float.
    fn f32(&mut self, float: f32) -> FullResult {
        self.bytes(float.to_be_bytes())
    }

    /// Write out a big endian encoded 64-bit float.
    fn f64(&mut self, float: f64) -> FullResult {
        self.bytes(float.to_be_bytes())
    }
}
