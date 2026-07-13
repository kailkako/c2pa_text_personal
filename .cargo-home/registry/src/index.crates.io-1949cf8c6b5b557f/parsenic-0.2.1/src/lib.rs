//! #### A simple no-std/no-alloc I/O and parsing crate
//!
//! The main two traits for parsing are [`Read`] and [`Write`], implemented by
//! [`Reader`], which reads from a fixed-size [`slice`] of bytes, and
//! [`Writer`], which writes to a fixed-size [`slice`] of bytes.  The [`Read`]
//! and [`Write`] traits are designed to be extended with extension traits,
//! using [`traitful::extend#extend-a-trait`].
//!
//! Extension traits for big-endian an little-endian parsing are provided in
//! this crate as well; [`be::Read`], [`be::Write`], [`le::Read`],
//! [`le::Write`].  When importing a `Read` or `Write` extension trait, using
//! `as _` will avoid namespace conflicts.
//!
//! # Synchronous and Asynchronous
//!
//! The [`Read`] and [`Write`] traits are synchronous by design.  This makes it
//! simple to read and write on in-memory data without touching I/O.  For I/O
//! bound reading and writing, there are the [`io::Source`] and
//! [`io::Destination`] traits which work by buffering the bytes to be sent on
//! an [`io::Sender`] or received on an [`io::Receiver`].
//!
//! # Features
//!
//! Some non-default features can enable unstable (no API stability guarantees)
//! functionality.
//!
//!  - __*`unstable-io`*__: Bumps MSRV to 1.84, enables the [`io`] module
//!  - __*`unstable-error`*__: Bumps MSRV to 1.81, implements [`Error`] for
//!    error types
//!
//! [`Error`]: core::error::Error

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/parsenic"
)]
#![no_std]
#![forbid(unsafe_code, missing_docs)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

pub mod be;
pub mod class;
mod empty;
pub mod error;
#[cfg(feature = "unstable-io")]
pub mod io;
pub mod le;
mod purge;
mod read;
mod reader;
pub mod result;
mod write;
mod writer;

pub use self::{
    empty::{empty, Empty},
    purge::{purge, Purge},
    read::Read,
    reader::Reader,
    write::Write,
    writer::Writer,
};
