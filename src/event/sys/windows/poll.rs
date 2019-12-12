use std::io;
use std::time::Duration;

use crossterm_winapi::Handle;
use winapi::{
    shared::winerror::WAIT_TIMEOUT,
    um::{
        synchapi::WaitForMultipleObjects,
        winbase::{INFINITE, WAIT_ABANDONED_0, WAIT_OBJECT_0},
    },
};

use crate::Result;

use super::super::Waker;

pub(crate) struct WinApiPoll;

impl WinApiPoll {
    pub(crate) fn new() -> Result<WinApiPoll> {
        Ok(WinApiPoll {})
    }
}

impl WinApiPoll {
    pub fn poll(&mut self, timeout: Option<Duration>, waker: Option<&Waker>) -> Result<Option<bool>> {
        let dw_millis = timeout
            .map_or(INFINITE, |duration| duration.as_millis() as u32);

        let console_handle = Handle::current_in_handle()?;

        let (size, handles) = if let Some(waker) = waker {
            (1, &[*console_handle, **waker.semaphore().handle()])
        }else {
            &[*console_handle]
        };

        // Wait for handles to trigger for the given duration.
        let output =
            unsafe { WaitForMultipleObjects(handles.len() as u32, handles.as_ptr(), 0, dw_millis) };

        match output {
            // Input handle triggered.
            output if output == WAIT_OBJECT_0 => {
                Ok(Some(true))
            }
            // Semaphore handle triggered.
            #[cfg(feature = "event-stream")]
            output if output == WAIT_OBJECT_0 + 1 => {
                let _ = self.waker.reset();
                Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Poll operation was woken up by `Waker::wake`",
                )
                .into())
            }
            // Timeout elapsed.
            WAIT_TIMEOUT | WAIT_ABANDONED_0 => {
                Ok(None)
            }
            // Unexpected error during waiting.
            _ => Err(io::Error::last_os_error().into()),
        }
    }

    #[cfg(feature = "event-stream")]
    pub fn waker(&self) -> Waker {
        self.waker.clone()
    }
}
