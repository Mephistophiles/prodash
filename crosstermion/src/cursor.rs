#[cfg(feature = "crossterm")]
mod _impl {
    pub use crossterm::cursor::MoveUp;
}
#[cfg(feature = "crossterm")]
pub use _impl::*;

#[cfg(feature = "crossterm")]
#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        // Queue each command, then flush
        $crate::crossterm::queue!($writer $(, $command)*).and_then(|()| {
            $writer.flush().map_err($crate::crossterm::ErrorKind::IoError)
        }).map_err($crate::crossterm_utils::into_io_error)
    }
}

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {
        Ok(()) $(
            .and_then(|()| write!($writer, "{}", $command))
        )*.and_then(|_| $writer.flush())
    }
}

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
mod _impl {
    pub use termion::cursor::Up as MoveUp;
}
#[cfg(all(feature = "termion", not(feature = "crossterm")))]
pub use _impl::*;