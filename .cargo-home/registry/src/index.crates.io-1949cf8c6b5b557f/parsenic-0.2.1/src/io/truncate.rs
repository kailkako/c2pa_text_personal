use core::{ops::DerefMut, pin::Pin};

use crate::io::Seek;

/// Trait providing a truncation operation.
///
/// Similar to [`Seek`], the method is infallible, as it should not do any I/O.
///
/// If a file is being concurrently accessed by another process, don’t expect
/// things to work at all.  Files should have either a locking or ownership
/// mechanism to prevent concurrent access from other processes.
pub trait Truncate: Seek {
    /// Truncate the destination at the current cursor position.
    fn truncate(&mut self);
}

impl<T> Truncate for &mut T
where
    T: Truncate + Unpin + ?Sized,
{
    fn truncate(&mut self) {
        (**self).truncate();
    }
}

impl<T, U> Truncate for Pin<T>
where
    T: DerefMut<Target = U>,
    U: Truncate + Unpin,
{
    fn truncate(&mut self) {
        self.as_mut().get_mut().truncate();
    }
}
