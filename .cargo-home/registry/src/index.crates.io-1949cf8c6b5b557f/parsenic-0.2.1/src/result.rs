//! [`Result`](core::result::Result) type aliases

use crate::error::{
    EndError, Error, FlushError, FullError, LenError, LostError, OverflowError,
    StrError, Uleb128Error, Utf8Error,
};

/// Type alias for `Result` of [`Error`]
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Type alias for `Result` of [`LenError`]
pub type LenResult<T = (), E = LenError> = Result<T, E>;

/// Type alias for `Result` of [`EndError`]
pub type EndResult<T = (), E = EndError> = Result<T, E>;

/// Type alias for `Result` of [`Utf8Error`]
pub type Utf8Result<T = (), E = Utf8Error> = Result<T, E>;

/// Type alias for `Result` of [`OverflowError`]
pub type OverflowResult<T = (), E = OverflowError> = Result<T, E>;

/// Type alias for `Result` of [`FullError`]
pub type FullResult<T = (), E = FullError> = Result<T, E>;

/// Type alias for `Result` of [`LostError`]
pub type LostResult<T = (), E = LostError> = Result<T, E>;

/// Type alias for `Result` of [`StrError`]
pub type StrResult<T = (), E = StrError> = Result<T, E>;

/// Type alias for `Result` of [`Uleb128Error`]
pub type Uleb128Result<T = (), E = Uleb128Error> = Result<T, E>;

/// Type alias for `Result` of [`FlushError`]
pub type FlushResult<T = (), E = FlushError> = Result<T, E>;
