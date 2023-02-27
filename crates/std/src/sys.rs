//! Credits
//!
//! - Error decoding: [relibc](https://github.com/redox-os/relibc/blob/master/src/platform/linux/mod.rs).
//! - Invocation and IDs: [syscall.rs](https://github.com/japaric/syscall.rs).

use core::arch;

#[repr(usize)]
enum Id {
    Write = 1,
}

#[derive(Debug)]
pub struct Error {
    pub code: u8,
}

impl Error {
    #[inline]
    fn map_result(result: usize) -> Result<usize, Error> {
        if matches!(result as isize, -256..=-1) {
            Err(Error {
                code: (result as isize).unsigned_abs() as u8,
            })
        } else {
            Ok(result)
        }
    }
}

#[inline(never)]
#[no_mangle]
pub(crate) unsafe fn write(file: u32, bytes: &[u8]) -> Result<usize, Error> {
    let result: usize;

    arch::asm!(
        "syscall",
        inout("rax") Id::Write as usize => result,
        in("rdi") file,
        in("rsi") bytes.as_ptr(),
        in("rdx") bytes.len(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    Error::map_result(result)
}

use core::intrinsics;

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *mut u8, len: isize) -> *mut u8 {
    let mut i = 0;

    while i < len {
        *(intrinsics::offset(destination, i) as *mut u8) = *intrinsics::offset(source, i);

        i += 1;
    }

    destination
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memset(destination: *mut u8, value: i32, len: isize) -> *mut u8 {
    let mut i = 0;

    while i < len {
        *(intrinsics::offset(destination, i) as *mut u8) = value as u8;

        i += 1;
    }

    destination
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memcmp(this: *const u8, other: *const u8, len: isize) -> i32 {
    let mut i = 0;

    while i < len {
        let a = *intrinsics::offset(this, i);
        let b = *intrinsics::offset(other, i);

        if a != b {
            return a as i32 - b as i32;
        }

        i += 1;
    }

    0
}
