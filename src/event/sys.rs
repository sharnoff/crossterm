#[cfg(unix)]
pub(crate) use self::unix::waker::Waker;
#[cfg(windows)]
pub(crate) use self::windows::waker::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;
