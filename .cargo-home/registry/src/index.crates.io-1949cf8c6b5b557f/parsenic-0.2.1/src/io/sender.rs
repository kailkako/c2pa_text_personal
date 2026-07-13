use core::{future, pin::Pin};

use crate::{
    error::{FlushError, FullError},
    io::{Destination, Seek, Truncate},
    result::{FlushResult, LostResult},
};

/// [`slice`] buffered sender.
///
/// Sends data to a [`Destination`].
///
/// The `BUF` const generic is used to specify the size of the internal buffer,
/// defaulting to 8kB.
#[derive(Debug)]
pub struct Sender<T, const BUF: usize = 8192> {
    #[allow(dead_code)]
    destination: T,
    #[allow(dead_code)]
    cursor: usize,
    #[allow(dead_code)]
    buffer: [u8; BUF],
}

impl<T, const BUF: usize> Sender<T, BUF> {
    /// Create a new receiver (synchronous source, asynchronous destination).
    pub fn new(destination: T) -> Self {
        Self {
            destination,
            cursor: 0,
            buffer: [0; BUF],
        }
    }
}

impl<T, const BUF: usize> Sender<T, BUF>
where
    T: Destination + Unpin,
{
    /// Send `bytes` to the destination.
    ///
    /// May not send the full amount of bytes until either the buffer is full or
    /// [`flush()`](Self::flush()) is called.
    pub async fn send(&mut self, bytes: &[u8]) -> LostResult<usize> {
        let mut total_sent = 0;

        for byte in bytes.iter().cloned() {
            if self.cursor == BUF {
                self.cursor = 0;

                let sent = future::poll_fn(|cx| {
                    Pin::new(&mut self.destination)
                        .poll_send(cx, self.buffer.as_ref())
                })
                .await?;

                total_sent += sent;

                if sent != BUF {
                    return Ok(total_sent);
                }
            }

            self.buffer[self.cursor] = byte;
            self.cursor += 1;
        }

        Ok(total_sent)
    }

    /// Send buffered data with the destination.
    pub async fn flush(&mut self) -> FlushResult {
        let old_cursor = self.cursor;
        let sent = future::poll_fn(|cx| {
            Pin::new(&mut self.destination)
                .poll_send(cx, &self.buffer[..self.cursor])
        })
        .await?;

        self.cursor -= sent;

        if self.cursor != 0 {
            self.buffer.copy_within(sent..old_cursor, 0);
            return Err(FlushError::Full(FullError::from_remaining(
                self.cursor,
            )));
        }

        Ok(())
    }
}

impl<S, const BUF: usize> Seek for Sender<S, BUF>
where
    S: Seek,
{
    fn seek(&mut self, pos: u64) {
        self.destination.seek(pos);
    }

    fn position(&self) -> u64 {
        self.destination.position()
    }

    fn len(&self) -> u64 {
        self.destination.len()
    }
}

impl<T, const BUF: usize> Truncate for Sender<T, BUF>
where
    T: Truncate,
{
    fn truncate(&mut self) {
        self.destination.truncate()
    }
}
