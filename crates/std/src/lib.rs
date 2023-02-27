#![feature(format_args_nl)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(maybe_uninit_write_slice)]
#![feature(ptr_sub_ptr)]
#![feature(rustc_attrs)]
#![no_std]

pub(crate) mod sys;

pub mod env;
pub mod io;
pub mod panic;
pub mod process;
