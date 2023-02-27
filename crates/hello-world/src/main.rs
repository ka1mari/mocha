#![feature(naked_functions)]
#![no_main]
#![no_std]

use core::arch;
use mocha_std::{
    env,
    io::{self, Write},
    process,
};

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    arch::asm!(
        // clear the frame pointer
        // marks this as the outer frame
        "xor rbp, rbp",
        // copy rsp to rdi as the first arg for main.
        "mov rdi, rsp",
        "call {}",
        sym main,
        options(noreturn),
    )
}

#[inline(always)]
unsafe extern "C" fn main(sp: *const isize) -> ! {
    unsafe {
        env::init_env(sp);
    }

    let mut stdout = io::stdout();
    let _ = writeln!(&mut stdout, "hello world!");
    let _ = stdout.flush();

    None::<&str>.unwrap();

    process::exit(0);
}
