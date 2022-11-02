pub use owned_buf::OwnedBuf;
pub use std::io::Result;

use core::fmt;
use std::io::{BorrowedCursor, IoSliceMut, SeekFrom};

// A simple async-ification of sync Read plus downcasting methods.
pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        unimplemented!()
    }
    async fn read_buf(&mut self, buf: &mut BorrowedCursor<'_>) -> Result<()> {
        unimplemented!()
    }
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        unimplemented!()
    }
    async fn read_buf_exact(&mut self, buf: &mut BorrowedCursor<'_>) -> Result<()> {
        unimplemented!()
    }
    // async fn read_buf_vectored(&mut self, bufs: &mut BorrowedSliceCursor<'_>) -> Result<usize> { unimplemented!() }
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        unimplemented!()
    }
    async fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        unimplemented!()
    }

    // TODO iterator methods

    fn is_read_vectored(&self) -> bool {
        false
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    // fn bytes(self) -> Bytes<Self>
    // where
    //     Self: Sized,
    // { unimplemented!() }

    // fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    // where
    //     Self: Sized,
    // { unimplemented!() }

    // fn take(self, limit: u64) -> Take<Self>
    // where
    //     Self: Sized,
    // { unimplemented!() }

    // TODO would be nice to have default impls
    fn as_ready(&mut self) -> Option<&mut impl ReadyRead>;
    fn as_owned(&mut self) -> Option<&mut impl OwnedRead>;

    // fn as_ready_dyn(&mut self) -> Option<&mut dyn ReadyRead> {
    //     None
    // }

    // fn as_owned_dyn(&mut self) -> Option<&mut dyn OwnedRead> {
    //     None
    // }
}

// Used for completion model systems.
pub trait OwnedRead: Read {
    async fn read(&mut self, buf: OwnedBuf) -> (OwnedBuf, Result<()>);
    async fn read_exact(&mut self, buf: OwnedBuf) -> (OwnedBuf, Result<()>) {
        unimplemented!()
    }
    async fn read_to_end(&mut self, buf: Vec<u8>) -> (Vec<u8>, Result<usize>) {
        unimplemented!()
    }

    // read_vectored - future work
}

// Used for epoll-like systems
pub trait Ready {
    async fn ready(&mut self, interest: Interest) -> Result<Readiness>;
}

pub trait ReadyRead: Ready + Read {
    fn non_blocking_read(&mut self, buf: &mut BorrowedCursor<'_>) -> Result<NonBlocking<()>>;
    // fn non_blocking_read_vectored(&mut self, bufs: &mut BorrowedSliceCursor<'_>) -> Result<NonBlocking<usize>> { unimplemented!() }

    // TODO do we want async convenience methods here? Or should use those on Read?
    // read, read_vectored, read_exact, read_to_end
}

/// Express which notifications the user is interested in receiving.
#[derive(Copy, Clone)]
pub struct Interest(u32);

/// Describes which operations are ready for an IO resource.
#[derive(Copy, Clone)]
pub struct Readiness(u32);

/// Whether an IO operation is ready for reading/writing or would block.
#[derive(Copy, Clone, Debug)]
pub enum NonBlocking<T> {
    Ready(T),
    WouldBlock,
}

impl Interest {
    pub const READ: Interest = Interest(1);
    pub const WRITE: Interest = Interest(2);
    pub const READ_WRITE: Interest = Interest(Interest::READ.0 | Interest::WRITE.0);
}

impl fmt::Debug for Interest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Readiness {
    /// The resource is ready to read from.
    pub fn read(self) -> bool {
        unimplemented!()
    }

    ///  The resource is ready to write to.
    pub fn write(self) -> bool {
        unimplemented!()
    }

    /// The resource has hung up.
    ///
    /// Note there may still be data to read.
    /// Note that the user does not *need* to check this method, even if the resource has hung up,
    /// the behaviour of `non_blocking_read` and `non_blocking_write` is defined and they should not
    /// panic.
    /// Note that the user does not need to request an interest in hup notifications, they may always
    /// be returned
    pub fn hup(self) -> bool {
        unimplemented!()
    }
}

impl fmt::Debug for Readiness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

pub trait BufRead: Read {
    async fn fill_buf(&mut self) -> Result<&[u8]>;
    fn consume(&mut self, amt: usize);

    async fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize> {
        unimplemented!()
    }
    async fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        unimplemented!()
    }
    // #[unstable]
    async fn has_data_left(&mut self) -> Result<bool> {
        unimplemented!()
    }
}

pub trait Seek {
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64>;

    async fn rewind(&mut self) -> Result<()> {
        unimplemented!()
    }
    async fn stream_len(&mut self) -> Result<u64> {
        unimplemented!()
    }
    async fn stream_position(&mut self) -> Result<u64> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
