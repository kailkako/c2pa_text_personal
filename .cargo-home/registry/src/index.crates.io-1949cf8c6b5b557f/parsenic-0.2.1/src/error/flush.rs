use core::fmt;

use crate::error::{FullError, LostError};

/// Flush error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FlushError {
    /// Destination has run out of space
    Full(FullError),
    /// Destination lost (from either corruption or disconnection)
    Lost(LostError),
}

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for FlushError {}

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("flush error: ")?;

        match self {
            Self::Full(err) => fmt::Display::fmt(err, f),
            Self::Lost(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<FullError> for FlushError {
    fn from(error: FullError) -> Self {
        Self::Full(error)
    }
}

impl From<LostError> for FlushError {
    fn from(error: LostError) -> Self {
        Self::Lost(error)
    }
}
