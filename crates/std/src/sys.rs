//! Credits
//!
//! - Error decoding: [relibc](https://github.com/redox-os/relibc/blob/master/src/platform/linux/mod.rs).
//! - Invocation and IDs: [syscall.rs](https://github.com/japaric/syscall.rs).

use core::arch;

mod mem;

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

pub(crate) unsafe fn exit(code: i32) -> ! {
    arch::asm!(
        "syscall",
        in("rax") 231_usize,
        in("rdi") code,
        options(noreturn, nostack),
    )
}
