use crate::{
    io::{self, Write},
    process,
};
use core::panic::PanicInfo;

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    let mut stderr = io::stderr();

    let _ = writeln!(&mut stderr, "\x1b[38;5;9mmocha:\x1b[m panic: {info}");
    let _ = stderr.flush();

    process::exit(1)
}
