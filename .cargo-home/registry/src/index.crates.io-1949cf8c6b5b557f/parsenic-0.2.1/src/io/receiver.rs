use core::{future, pin::Pin};

use crate::{
    io::{Seek, Source},
    result::LostResult,
    Reader,
};

/// [`Extend`] buffered receiver.
///
/// Receives data from a [`Source`].
///
/// The `BUF` const generic is used to specify the size of the internal buffer,
/// defaulting to 8kB.
#[derive(Debug)]
pub struct Receiver<S, T, const BUF: usize = 8192> {
    source: S,
    destination: T,
    cursor: usize,
    buffer: [u8; BUF],
}

impl<S, T, const BUF: usize> Receiver<S, T, BUF> {
    /// Create a new receiver (asynchronous source, synchronous destination).
    pub fn new(source: S, destination: T) -> Self {
        Self {
            source,
            destination,
            cursor: BUF,
            buffer: [0; BUF],
        }
    }
}

impl<S, T, const BUF: usize> Receiver<S, T, BUF>
where
    S: Source + Unpin,
    T: Extend<u8> + AsRef<[u8]>,
{
    /// Receive up to `count` bytes from the source to be read.
    ///
    /// May return less if there aren't enough in the source.
    pub async fn recv(&mut self, count: usize) -> LostResult<Reader<'_>> {
        let mut count = count;
        let mut more = true;

        // While not all bytes are written
        while count != 0 && more {
            // If cursor has reached the buffer length, overwrite
            if BUF == self.cursor {
                let mut written = 0;

                'more: while written < BUF {
                    let bytes = future::poll_fn(|cx| {
                        Pin::new(&mut self.source).poll_recv(
                            cx,
                            self.buffer.get_mut(written..).unwrap_or(&mut []),
                        )
                    })
                    .await?;

                    if bytes == 0 {
                        more = false;
                        break 'more;
                    }

                    written += bytes;
                }

                self.cursor = 0;
            }

            // Extend destination with buffer contents
            let leftovers = self
                .buffer
                .get(self.cursor..)
                .unwrap_or(self.buffer.as_slice());
            let leftovers = leftovers.get(..count).unwrap_or(leftovers);

            count = count.saturating_sub(leftovers.len());
            self.cursor = self.cursor.saturating_add(leftovers.len());
            self.destination.extend(leftovers.iter().cloned());
        }

        Ok(Reader::new(self.destination.as_ref()))
    }
}

impl<S, T, const BUF: usize> Seek for Receiver<S, T, BUF>
where
    S: Seek,
{
    fn seek(&mut self, pos: u64) {
        self.source.seek(pos);
    }

    fn position(&self) -> u64 {
        self.source.position()
    }

    fn len(&self) -> u64 {
        self.source.len()
    }
}
