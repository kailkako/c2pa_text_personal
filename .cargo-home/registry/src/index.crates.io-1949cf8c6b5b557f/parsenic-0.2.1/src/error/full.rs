use core::{fmt, num::NonZeroUsize};

/// Destination has run out of space
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct FullError(Option<NonZeroUsize>);

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for FullError {}

impl fmt::Display for FullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("destination has run out of space")?;

        if let Some(remaining) = self.0 {
            write!(f, ", {remaining} bytes remaining")?;
        }

        Ok(())
    }
}

impl FullError {
    /// Create a new [`FullError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`FullError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
