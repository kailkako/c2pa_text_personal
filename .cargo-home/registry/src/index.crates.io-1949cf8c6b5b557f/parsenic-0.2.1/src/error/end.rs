use core::{fmt, num::NonZeroUsize};

/// Expected buffer to end, but it didn't
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct EndError(Option<NonZeroUsize>);

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for EndError {}

impl fmt::Display for EndError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected buffer to end, but it didn't")?;

        if let Some(remaining) = self.0 {
            write!(f, ", {remaining} bytes remaining")?;
        }

        Ok(())
    }
}

impl EndError {
    /// Create a new [`EndError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`EndError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
