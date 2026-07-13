use crate::{error::LenError, result::LenResult, Read};

/// Reader that is always at EOF
///
/// Returned by [`empty()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Empty();

/// Construct a new handle to an empty reader.
///
/// The returned type implements [`Read`](crate::Read).
///
/// This API takes some inspiration from [`std::io::empty()`].
///
/// [`std::io::empty()`]: https://doc.rust-lang.org/stable/std/io/fn.empty.html
pub fn empty() -> Empty {
    Empty()
}

impl Read for Empty {
    fn remaining(&self) -> usize {
        0
    }

    fn take(&mut self, len: usize) -> LenResult<Self> {
        (len == 0).then(Empty).ok_or(LenError::from_remaining(len))
    }

    fn slice(&mut self, len: usize) -> LenResult<&'_ [u8]> {
        (len == 0)
            .then_some([].as_ref())
            .ok_or(LenError::from_remaining(len))
    }
}
