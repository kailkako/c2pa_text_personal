use core::{
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::LostResult;

/// [`Receiver`] asynchronous source
///
/// [`Receiver`]: crate::io::Receiver
pub trait Source {
    /// Attempt to receive bytes into `buf`.
    ///
    /// Returns the number of bytes received when ready, or zero when no more
    /// data is available.
    fn poll_recv(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<LostResult<usize>>;
}

impl<S> Source for &mut S
where
    S: Source + Unpin + ?Sized,
{
    fn poll_recv(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<LostResult<usize>> {
        S::poll_recv(Pin::new(&mut **self), cx, buf)
    }
}

impl<S, T> Source for Pin<S>
where
    S: DerefMut<Target = T>,
    T: Source,
{
    fn poll_recv(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<LostResult<usize>> {
        <S::Target as Source>::poll_recv(self.as_deref_mut(), cx, buf)
    }
}
