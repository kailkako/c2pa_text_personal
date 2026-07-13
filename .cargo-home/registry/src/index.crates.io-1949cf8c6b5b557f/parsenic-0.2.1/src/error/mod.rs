//! Error types

mod end;
mod flush;
mod full;
mod len;
mod lost;
mod overflow;
mod parsing;
mod str;
mod uleb128;
mod utf8;

pub use self::{
    end::EndError, flush::FlushError, full::FullError, len::LenError,
    lost::LostError, overflow::OverflowError, parsing::Error, str::StrError,
    uleb128::Uleb128Error, utf8::Utf8Error,
};
