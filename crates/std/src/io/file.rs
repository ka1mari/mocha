use crate::{
    io::{self, Write},
    sys,
};

/// A raw file.
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(0xff_ff_ff_fe)]
#[rustc_nonnull_optimization_guaranteed]
pub struct File {
    descriptor: u32,
}

impl File {
    /// Create a new file from a raw descriptor.
    ///
    /// # Panics
    ///
    /// If `descriptor` is invalid.
    pub const fn new(descriptor: u32) -> Self {
        assert!(
            matches!(descriptor, 0..=0xff_ff_ff_fe),
            "descriptor is invalid"
        );

        unsafe { Self::new_unchecked(descriptor) }
    }

    /// Create a new file from a raw descriptor.
    ///
    /// # Safety
    ///
    /// Caller must ensure `descriptor` is valid.
    pub const unsafe fn new_unchecked(descriptor: u32) -> Self {
        Self { descriptor }
    }
}

impl Write for File {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        unsafe { sys::write(self.descriptor, bytes) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
