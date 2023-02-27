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

#[derive(Debug)]
pub enum Command {
    Help,
    Yes,
}

impl Command {
    pub fn from_arg(arg: &str) -> Option<Self> {
        let command = match arg {
            "mocha-utils" => Command::Help,
            "help" => Command::Help,
            "yes" => Command::Yes,
            _ => return None,
        };

        Some(command)
    }
}

fn help() -> ! {
    let mut stdout = io::stdout();
    let _ = writeln!(&mut stdout, "mocha-utils [command]");
    let _ = stdout.flush();

    process::exit(0)
}

#[inline(always)]
unsafe extern "C" fn main(sp: *const isize) -> ! {
    unsafe {
        env::init_env(sp);
    }

    let mut args = env::args();
    let program = args.next().unwrap();
    let program = program
        .rsplit_once('/')
        .map(|(_ancestors, program)| program)
        .unwrap_or(program);

    let command = if let Some(command) = Command::from_arg(program) {
        command
    } else if let Some(command) = args.next().and_then(Command::from_arg) {
        command
    } else {
        help();
    };

    let mut stdout = io::stdout();

    match command {
        Command::Help => help(),
        Command::Yes => loop {
            let _ = writeln!(&mut stdout, "y");
        },
    }

    let _ = stdout.flush();

    process::exit(0)
}
