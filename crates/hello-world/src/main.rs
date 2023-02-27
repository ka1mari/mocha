#![no_main]
#![no_std]

use core::arch;
use mocha_std::{
    io::{self, Write},
    process,
};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let stack_top: *const u8;

    arch::asm!(
        // mark outer stack frame
        "xor rbp, rbp",
        "mov {}, rsp",
        // align for sse
        "and rsp, -16",
        out(reg) stack_top,
        options(nostack),
    );

    let _stack_top = stack_top;

    let mut stdout = io::stdout();
    let _ = writeln!(&mut stdout, "hello world!");
    let _ = stdout.flush();

    None::<&str>.unwrap();

    process::exit(0);
}
