use core::fmt;

/// Value overflow (variable can't contain parsed value)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct OverflowError();

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for OverflowError {}

impl fmt::Display for OverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("value overflow")
    }
}

impl OverflowError {
    /// Create a new [`OverflowError`].
    pub const fn new() -> Self {
        Self()
    }
}
