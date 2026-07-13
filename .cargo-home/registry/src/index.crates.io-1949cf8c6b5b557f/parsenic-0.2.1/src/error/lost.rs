use core::fmt;

/// Destination lost (from either corruption or disconnection)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct LostError(Option<&'static str>);

/// __*`unstable-error`*__: feature required
#[cfg(feature = "unstable-error")]
impl core::error::Error for LostError {}

impl fmt::Display for LostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("destination lost")?;

        if let Some(message) = self.0 {
            f.write_str(", ")?;
            f.write_str(message)?;
        }

        Ok(())
    }
}

impl LostError {
    /// Create a new [`LostError`].
    pub fn new() -> Self {
        Self(None)
    }

    /// Create a new [`LostError`] from message payload.
    pub fn from_message(message: &'static str) -> Self {
        Self(Some(message))
    }

    /// Return the message payload.
    pub fn message(&self) -> Option<&'static str> {
        self.0
    }
}
