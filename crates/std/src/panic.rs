use crate::{
    io::{self, Write},
    process,
};
use core::{
    panic::PanicInfo,
    sync::atomic::{AtomicUsize, Ordering},
};

static PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    let mut stderr = io::stderr();

    // prevent stack overflow from panics within panics.
    if PANIC_COUNT.fetch_add(1, Ordering::SeqCst) > 0 {
        // bypass anything that could panic.
        let _ = writeln!(
            &mut stderr.inner.inner,
            "\x1b[38;5;9mmocha:\x1b[m attempted panic within panic"
        );
    } else {
        let _ = writeln!(&mut stderr, "\x1b[38;5;9mmocha:\x1b[m panic: {info}");
    }

    let _ = stderr.flush();

    process::exit(1)
}
