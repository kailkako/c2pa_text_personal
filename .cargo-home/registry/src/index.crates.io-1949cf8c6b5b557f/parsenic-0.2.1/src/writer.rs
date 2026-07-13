use crate::{error::FullError, result::FullResult, Write};

/// [`Write`]r that writes to a [`slice`] of bytes
#[derive(Debug)]
pub struct Writer<'a>(&'a mut [u8]);

impl<'a> Writer<'a> {
    /// Create a new `Writer` into the provided growable `buffer`.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self(buffer)
    }

    fn subslice<'b>(&mut self, len: usize) -> FullResult<&'b mut [u8]>
    where
        'a: 'b,
    {
        let slice;
        let mut tmp: &'a mut [u8] = &mut [];

        if let Some(remaining) = len.checked_sub(self.0.len()) {
            if remaining != 0 {
                return Err(FullError::from_remaining(remaining));
            }
        }

        core::mem::swap(&mut tmp, &mut self.0);
        (slice, self.0) = tmp.split_at_mut(len);

        Ok(slice)
    }
}

impl Write for Writer<'_> {
    fn remaining(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn take(&mut self, len: usize) -> FullResult<Self> {
        Ok(Writer(self.subslice(len)?))
    }

    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FullResult {
        let bytes = bytes.as_ref();

        self.subslice(bytes.len())
            .map(|buf| buf.copy_from_slice(bytes))
    }
}
