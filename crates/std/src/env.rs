use core::{
    ffi::{self, CStr},
    ptr, slice,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

static ARGS_PTR: AtomicPtr<*const ffi::c_char> = AtomicPtr::new(ptr::null_mut());
static ARGS_LEN: AtomicUsize = AtomicUsize::new(0);

static VARS_PTR: AtomicPtr<*const ffi::c_char> = AtomicPtr::new(ptr::null_mut());
static VARS_LEN: AtomicUsize = AtomicUsize::new(0);

/// Setup the enviroment.
///
/// # Safety
///
/// Caller must only invoke this once, and provide a valid stack pointer.
pub unsafe fn init_env(sp: *const isize) {
    // skip argc
    let args = sp.add(1).cast::<*const ffi::c_char>();
    let mut end = args;

    while !(*end).is_null() {
        end = end.add(1);
    }

    let len = end.sub_ptr(args);

    ARGS_PTR.store(args as *mut *const ffi::c_char, Ordering::SeqCst);
    ARGS_LEN.store(len, Ordering::SeqCst);

    // skip argv null terminator
    let vars = end.add(1);
    let mut end = vars;

    while !(*end).is_null() {
        end = end.add(1);
    }

    let len = end.sub_ptr(vars);

    VARS_PTR.store(vars as *mut *const ffi::c_char, Ordering::SeqCst);
    VARS_LEN.store(len, Ordering::SeqCst);
}

pub fn args() -> impl Iterator<Item = &'static str> {
    unsafe {
        let ptr = ARGS_PTR.load(Ordering::SeqCst);
        let len = ARGS_LEN.load(Ordering::SeqCst);

        slice::from_raw_parts(ptr, len)
            .iter()
            .copied()
            .flat_map(|arg| CStr::from_ptr(arg).to_str())
            .fuse()
    }
}

pub fn vars() -> impl Iterator<Item = (&'static str, &'static str)> {
    unsafe {
        let ptr = VARS_PTR.load(Ordering::SeqCst);
        let len = VARS_LEN.load(Ordering::SeqCst);

        slice::from_raw_parts(ptr, len)
            .iter()
            .copied()
            .flat_map(|arg| CStr::from_ptr(arg).to_str())
            .flat_map(|arg| arg.split_once('='))
            .fuse()
    }
}
