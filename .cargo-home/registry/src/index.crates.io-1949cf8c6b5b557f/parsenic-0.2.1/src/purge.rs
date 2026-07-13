use crate::{error::FullError, result::FullResult, Write};

/// Writer that moves data into the void
///
/// Returned by [`purge()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Purge(Option<usize>);

/// Create an instance of a writer which will successfully consume all bytes.
///
/// The returned type implements [`Write`](crate::Write).
///
/// This API takes some inspiration from [`std::io::sink()`].
///
/// [`std::io::sink()`]: https://doc.rust-lang.org/stable/std/io/fn.sink.html
#[must_use]
pub fn purge() -> Purge {
    Purge(None)
}

impl Write for Purge {
    fn remaining(&self) -> Option<usize> {
        self.0
    }

    fn take(&mut self, len: usize) -> FullResult<Self>
    where
        Self: Sized,
    {
        let Some(limit) = self.0 else {
            return Ok(Purge(Some(len)));
        };

        self.0 = Some(
            limit
                .checked_sub(len)
                .ok_or(FullError::from_remaining(len.saturating_sub(limit)))?,
        );

        Ok(Purge(Some(len)))
    }

    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FullResult {
        let len = bytes.as_ref().len();
        let Some(limit) = self.0 else {
            return Ok(());
        };

        self.0 = Some(
            limit
                .checked_sub(len)
                .ok_or(FullError::from_remaining(len.saturating_sub(limit)))?,
        );

        Ok(())
    }
}
