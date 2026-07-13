use core::fmt;

use crate::error::{
    EndError, FlushError, FullError, LenError, LostError, OverflowError,
    StrError, Uleb128Error, Utf8Error,
};

/// Parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Expected buffer to end, but it didn't
    End(EndError),
    /// Invalid UTF-8
    Utf8(Utf8Error),
    /// Overflow (variable can't contain parsed value)
    Overflow(OverflowError),
    /// Destination has run out of space
    Full(FullError),
    /// Destination lost (from either corruption or disconnection)
    Lost(LostError),
}

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("parsing error: ")?;

        match self {
            Self::Len(err) => fmt::Display::fmt(err, f),
            Self::End(err) => fmt::Display::fmt(err, f),
            Self::Utf8(err) => fmt::Display::fmt(err, f),
            Self::Overflow(err) => fmt::Display::fmt(err, f),
            Self::Full(err) => fmt::Display::fmt(err, f),
            Self::Lost(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<LenError> for Error {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<EndError> for Error {
    fn from(error: EndError) -> Self {
        Self::End(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<OverflowError> for Error {
    fn from(error: OverflowError) -> Self {
        Self::Overflow(error)
    }
}

impl From<FullError> for Error {
    fn from(error: FullError) -> Self {
        Self::Full(error)
    }
}

impl From<LostError> for Error {
    fn from(error: LostError) -> Self {
        Self::Lost(error)
    }
}

impl From<FlushError> for Error {
    fn from(error: FlushError) -> Self {
        match error {
            FlushError::Full(error) => Self::Full(error),
            FlushError::Lost(error) => Self::Lost(error),
        }
    }
}

impl From<Uleb128Error> for Error {
    fn from(error: Uleb128Error) -> Self {
        match error {
            Uleb128Error::Len(error) => Self::Len(error),
            Uleb128Error::Overflow(error) => Self::Overflow(error),
        }
    }
}

impl From<StrError> for Error {
    fn from(error: StrError) -> Self {
        match error {
            StrError::Len(error) => Self::Len(error),
            StrError::Utf8(error) => Self::Utf8(error),
        }
    }
}
