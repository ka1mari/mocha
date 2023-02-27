use core::mem::MaybeUninit;

// yes.
#[repr(align(8192), C)]
pub struct Yes {
    buf: [MaybeUninit<u8>; 8192],
    rem: usize,
}

impl Yes {
    pub const fn new() -> Self {
        Self {
            buf: MaybeUninit::uninit().transpose(),
            rem: 0,
        }
    }

    pub fn set_pattern(&mut self, pattern: &str) {
        let mut chunks = self.buf.chunks_exact_mut(pattern.len() + 1);

        for chunk in chunks.by_ref() {
            let (last, rest) = chunk.split_last_mut().unwrap();

            MaybeUninit::write_slice(rest, pattern.as_bytes());
            *last = MaybeUninit::new(b'\n');
        }

        self.rem = chunks.into_remainder().len();
    }

    pub const fn len(&self) -> usize {
        8192 - self.rem
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.buf[..self.len()]) }
    }
}
