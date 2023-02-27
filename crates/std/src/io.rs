//! A module for working with processes.

use core::fmt;

pub use self::{buf::BufWriter, file::File};
pub use crate::sys::Error;

mod buf;
mod file;

pub type Result<T> = core::result::Result<T, Error>;

pub trait Write {
    fn write(&mut self, bytes: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
        struct Proxy<'writer, W: Write + ?Sized> {
            inner: &'writer mut W,
        }

        impl<'writer, W: Write + ?Sized> fmt::Write for Proxy<'writer, W> {
            fn write_str(&mut self, string: &str) -> fmt::Result {
                let _ = self.inner.write(string.as_bytes());

                Ok(())
            }
        }

        let mut proxy = Proxy { inner: self };

        match fmt::write(&mut proxy, fmt) {
            Ok(()) => Ok(()),
            Err(_error) => Err(Error { code: 0 }),
        }
    }
}

macro_rules! stdio_writers {
    ($(#[doc = $doc:literal] $method:ident -> $struct:ident: $file:expr;)*) => {$(
        #[doc = concat!("A handle to ", $doc, ".")]
        pub struct $struct {
            pub(crate) inner: BufWriter<File, 1024>,
        }

        impl $struct {
            pub fn file_mut(&mut self) -> &mut File {
                &mut self.inner.inner
            }
        }

        impl Write for $struct {
            fn write(&mut self, bytes: &[u8]) -> Result<usize> {
                self.inner.write(bytes)
            }

            fn flush(&mut self) -> Result<()> {
                self.inner.flush()
            }
        }

        #[doc = concat!("Obtain a handle to ", $doc, ".")]
        pub fn $method() -> $struct {
            $struct {
                inner: BufWriter::new(File::new($file)),
            }
        }
    )*};
}

stdio_writers! {
    #[doc = "standard output"]
    stdout -> Stdout: 1;

    #[doc = "standard error"]
    stderr -> Stderr: 2;
}
