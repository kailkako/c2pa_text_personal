use core::{fmt, str};

/// Invalid UTF-8
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub struct Utf8Error(str::Utf8Error);

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for Utf8Error {}

impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid utf8: ")?;
        fmt::Display::fmt(&self.0, f)
    }
}

impl Utf8Error {
    /// Return the offset from the given reader's cursor up to which valid UTF-8
    /// was verified.
    pub const fn valid_up_to(&self) -> usize {
        self.0.valid_up_to()
    }

    /// Return more information about the failure:
    ///
    /// `None`: the end of the input was reached unexpectedly.
    /// `self.valid_up_to()` is 1 to 3 bytes from the end of the input.
    ///
    /// `Some(len)`: an unexpected byte was encountered.  The length provided is
    /// that of the invalid byte sequence that starts at the index given by
    /// `valid_up_to()`.  Decoding should resume after that sequence (after
    /// inserting a U+FFFD REPLACEMENT CHARACTER) in case of lossy decoding.
    pub const fn error_len(&self) -> Option<usize> {
        self.0.error_len()
    }
}

impl From<str::Utf8Error> for Utf8Error {
    fn from(error: str::Utf8Error) -> Self {
        Self(error)
    }
}
