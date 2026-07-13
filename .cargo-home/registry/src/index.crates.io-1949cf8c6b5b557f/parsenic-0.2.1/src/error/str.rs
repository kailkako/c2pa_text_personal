use core::fmt;

use crate::error::{LenError, Utf8Error};

/// String parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StrError {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Invalid UTF-8
    Utf8(Utf8Error),
}

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for StrError {}

impl fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("string parsing error: ")?;

        match self {
            Self::Len(err) => fmt::Display::fmt(err, f),
            Self::Utf8(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<LenError> for StrError {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<Utf8Error> for StrError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}
