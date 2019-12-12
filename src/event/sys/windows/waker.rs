use std::sync::{Arc, Mutex};

use crossterm_winapi::Semaphore;

use crate::Result;


pub struct CancelTx(Semaphore);

pub struct CancelRx(Semaphore);

/// Allows to wake up the `WinApiPoll::poll()` method.
#[derive(Clone)]
pub(crate) struct Waker {
    inner: Semaphore,
}

impl Waker {
    /// Creates a new waker.
    ///
    /// `Waker` is based on the `Semaphore`. You have to use the semaphore
    /// handle along with the `WaitForMultipleObjects`.
    pub(crate) fn new() -> Result<Self> {
        let inner = Semaphore::new()?;

        Ok(Self {
            inner
        })
    }

    /// Wakes the `WaitForMultipleObjects`.
    pub(crate) fn wake(&self) -> Result<()> {
        self.inner.release()?;
        Ok(())
    }

    /// Returns the semaphore associated with the waker.
    pub(crate) fn semaphore(&self) -> Semaphore {
        self.inner.clone()
    }
}
