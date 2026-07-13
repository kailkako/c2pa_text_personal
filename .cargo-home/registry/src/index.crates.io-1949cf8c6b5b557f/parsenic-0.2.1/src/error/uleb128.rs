use core::fmt;

use crate::error::{LenError, OverflowError};

/// ULEB128 parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Uleb128Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Overflow (variable can't contain parsed value)
    Overflow(OverflowError),
}

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for Uleb128Error {}

impl fmt::Display for Uleb128Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("uleb128 parsing error: ")?;

        match self {
            Self::Len(err) => fmt::Display::fmt(err, f),
            Self::Overflow(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<LenError> for Uleb128Error {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<OverflowError> for Uleb128Error {
    fn from(error: OverflowError) -> Self {
        Self::Overflow(error)
    }
}
