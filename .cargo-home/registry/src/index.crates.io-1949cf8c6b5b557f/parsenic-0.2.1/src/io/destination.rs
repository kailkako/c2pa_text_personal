use core::{
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::LostResult;

/// [`Sender`] asynchronous destination
///
/// [`Sender`]: crate::io::Sender
pub trait Destination {
    /// Attempt to send `buf` bytes.
    ///
    /// Returns the number of bytes sent when ready, or zero when destination is
    /// full.
    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<LostResult<usize>>;
}

impl<D> Destination for &mut D
where
    D: Destination + Unpin + ?Sized,
{
    fn poll_send(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<LostResult<usize>> {
        D::poll_send(Pin::new(self.as_mut().get_mut()), cx, buf)
    }
}

impl<D, T> Destination for Pin<D>
where
    D: DerefMut<Target = T>,
    T: Destination,
{
    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<LostResult<usize>> {
        <D::Target as Destination>::poll_send(self.as_deref_mut(), cx, buf)
    }
}
