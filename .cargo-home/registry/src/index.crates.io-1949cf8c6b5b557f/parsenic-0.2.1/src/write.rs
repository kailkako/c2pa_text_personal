use traitful::seal;

use crate::{
    class::UInt,
    error::EndError,
    result::{EndResult, FullResult},
    Purge, Writer,
};

/// Basic writing methods
#[seal(Writer<'_>, Purge)]
pub trait Write {
    /// Return the number of bytes remaining in this writer, or `None` if
    /// infinite.
    fn remaining(&self) -> Option<usize>;

    /// Write a number of bytes as a new writer.
    ///
    /// Advances `len` bytes regardless of how many bytes the returned writer
    /// writes.
    fn take(&mut self, len: usize) -> FullResult<Self>
    where
        Self: Sized;

    /// Write out raw bytes.
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FullResult;

    /// Write out a UTF-8 string slice (does not include length).
    fn str(&mut self, string: impl AsRef<str>) -> FullResult {
        self.bytes(string.as_ref().as_bytes())
    }

    /// Write out a byte
    fn u8(&mut self, byte: u8) -> FullResult {
        self.bytes([byte])
    }

    /// Write out a signed byte
    fn i8(&mut self, byte: i8) -> FullResult {
        let [byte] = byte.to_ne_bytes();

        self.u8(byte)
    }

    /// Write out `value` in ULEB128 encoding.
    fn uleb128<T: UInt>(&mut self, value: T) -> FullResult {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != T::ZERO;

            self.u8(if more { byte | 0x80 } else { byte & !0x80 })?;

            more
        } {}

        Ok(())
    }

    /// Return [`Ok`] if end of buffer.
    fn end(&self) -> EndResult {
        (self.remaining() == Some(0))
            .then_some(())
            .ok_or(EndError::from_remaining(self.remaining().unwrap_or(0)))
    }
}
