#![feature(iter_intersperse)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(maybe_uninit_write_slice)]
#![feature(naked_functions)]
#![no_main]
#![no_std]

use crate::yes::Yes;
use core::arch;
use mocha_std::{
    env,
    io::{self, Write},
    process,
};

mod yes;

#[derive(Debug)]
pub enum Command {
    Echo,
    Env,
    Help,
    Yes,
}

impl Command {
    pub fn from_arg(arg: &str) -> Option<Self> {
        let command = match arg {
            "echo" => Command::Echo,
            "env" => Command::Env,
            "help" => Command::Help,
            "yes" => Command::Yes,
            _ => return None,
        };

        Some(command)
    }
}

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

fn help() -> ! {
    let mut stdout = io::stdout();
    let _ = writeln!(&mut stdout, "mocha-utils [command]");
    let _ = writeln!(&mut stdout);
    let _ = writeln!(&mut stdout, "commands:");
    let _ = writeln!(
        &mut stdout,
        "  echo  repeats the given argument(s) to stdout",
    );

    let _ = writeln!(&mut stdout, "  env  display the current environment",);

    let _ = writeln!(
        &mut stdout,
        "  yes  spam stdout with \"y\" or the given argument",
    );

    let _ = writeln!(&mut stdout);
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

    match command {
        Command::Echo => {
            let mut stdout = io::stdout();

            for arg in args.intersperse(" ") {
                let _ = stdout.write(arg.as_bytes());
            }

            let _ = stdout.write(b"\n");
            let _ = stdout.flush();

            process::exit(0)
        }
        Command::Env => {
            let mut stdout = io::stdout();

            for (key, val) in env::vars() {
                let _ = writeln!(&mut stdout, "{key}={val}");
            }

            let _ = stdout.flush();

            process::exit(0)
        }
        Command::Help => help(),
        Command::Yes => {
            let mut stdout = io::stdout();

            // bypass buffering of stdout, we're doing a better job
            let stdout = stdout.file_mut();

            // yes
            let mut yes = Yes::new();

            // \n is appended to the end of the pattern
            let pattern = args.next().unwrap_or("y");

            yes.set_pattern(pattern);

            loop {
                let _ = stdout.write(yes.as_bytes());
            }
        }
    }
}
