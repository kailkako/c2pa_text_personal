use traitful::extend;

use crate::result::LenResult;

/// Little endian reader extension trait
#[extend]
pub trait Read: crate::Read {
    /// Read the next little endian `u16`
    fn u16(&mut self) -> LenResult<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `u32`
    fn u32(&mut self) -> LenResult<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `u64`
    fn u64(&mut self) -> LenResult<u64> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `u128`
    fn u128(&mut self) -> LenResult<u128> {
        Ok(u128::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `i16`
    fn i16(&mut self) -> LenResult<i16> {
        Ok(i16::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `i32`
    fn i32(&mut self) -> LenResult<i32> {
        Ok(i32::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `i64`
    fn i64(&mut self) -> LenResult<i64> {
        Ok(i64::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `i128`
    fn i128(&mut self) -> LenResult<i128> {
        Ok(i128::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `f32`
    fn f32(&mut self) -> LenResult<f32> {
        Ok(f32::from_le_bytes(self.array()?))
    }

    /// Read the next little endian `f64`
    fn f64(&mut self) -> LenResult<f64> {
        Ok(f64::from_le_bytes(self.array()?))
    }
}
