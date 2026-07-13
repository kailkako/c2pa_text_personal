use core::{fmt, num::NonZeroUsize};

/// Source ran over the end of the buffer
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct LenError(Option<NonZeroUsize>);

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for LenError {}

impl fmt::Display for LenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("source ran over the end of the buffer")?;

        if let Some(remaining) = self.0 {
            write!(f, ", {remaining} bytes remaining")?;
        }

        Ok(())
    }
}

impl LenError {
    /// Create a new [`LenError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`LenError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
