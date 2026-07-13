//! __*`unstable-io`*__ feature required; I/O primitives (MSRV 1.84)

mod destination;
mod receiver;
mod seek;
mod sender;
mod source;
mod truncate;

pub use self::{
    destination::Destination, receiver::Receiver, seek::Seek, sender::Sender,
    source::Source, truncate::Truncate,
};
