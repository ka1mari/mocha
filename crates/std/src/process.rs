//! A module for working with processes.

use core::arch;

/// Terminates the current process with the specified exit code.
pub fn exit(code: i32) -> ! {
    unsafe {
        arch::asm!(
            "syscall",
            in("rax") 231_usize,
            in("rdi") code,
            options(noreturn, nostack),
        )
    }
}
