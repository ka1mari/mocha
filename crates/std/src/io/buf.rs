use crate::io::{self, Write};
use core::{cmp, mem::MaybeUninit};

struct Buf<const CAPACITY: usize> {
    buf: [MaybeUninit<u8>; CAPACITY],
    filled: usize,
}

impl<const CAPACITY: usize> Buf<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            buf: MaybeUninit::uninit().transpose(),
            filled: 0,
        }
    }

    /// Returns a shared reference to the filled portion of the buffer.
    pub fn filled(&self) -> &[u8] {
        let bytes = &self.buf[..self.filled];

        unsafe { MaybeUninit::slice_assume_init_ref(bytes) }
    }

    /// Returns a mutable reference to the uninitialized portion of the buffer.
    pub fn unfilled(&mut self) -> &mut [MaybeUninit<u8>] {
        &mut self.buf[self.filled..]
    }

    /// Returns the number of bytes written to the buffer.
    pub fn written(&self) -> usize {
        self.filled
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.filled = 0;
    }

    /// Appends data to the buffer, advancing it's position.
    pub fn append(&mut self, bytes: &[u8]) {
        assert!(CAPACITY >= self.filled + bytes.len());

        MaybeUninit::write_slice(&mut self.unfilled()[..bytes.len()], bytes);

        self.filled += bytes.len();
    }
}

/// A stack-allocated buffered writer.
pub struct BufWriter<W: Write, const CAPACITY: usize> {
    pub(crate) inner: W,
    buf: Buf<CAPACITY>,
}

impl<W: Write, const CAPACITY: usize> BufWriter<W, CAPACITY> {
    pub const fn new(inner: W) -> Self {
        assert!(CAPACITY != 0, "buffer capacity cannot be zero");

        Self {
            inner,
            buf: Buf::new(),
        }
    }

    /// Writes the buffer to the inner writer, clearing the buffer.
    fn commit(&mut self) -> io::Result<()> {
        self.inner.write(self.buf.filled())?;
        self.buf.clear();

        Ok(())
    }
}

impl<W: Write, const CAPACITY: usize> Write for BufWriter<W, CAPACITY> {
    fn write(&mut self, mut bytes: &[u8]) -> io::Result<usize> {
        let len = bytes.len();

        while !bytes.is_empty() {
            // the uninitialized buffer.
            let uninit = self.buf.unfilled();

            // maximum amount to read.
            let rest = cmp::min(bytes.len(), uninit.len());
            let (head, tail) = bytes.split_at(rest);

            // append to the uninitialized buffer.
            self.buf.append(head);

            if self.buf.written() == CAPACITY {
                self.commit()?;
            }

            // truncate bytes
            bytes = tail;
        }

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.commit()?;
        self.inner.flush()?;

        Ok(())
    }
}
