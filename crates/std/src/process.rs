//! A module for working with processes.

use crate::sys;

/// Terminates the current process with the specified exit code.
pub fn exit(code: i32) -> ! {
    unsafe { sys::exit(code) }
}
