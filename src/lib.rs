#![no_std]
#![feature(core_intrinsics, lang_items)]
mod comm;
use core::intrinsics::abort;
use comm::io::*;
mod gpio;
use gpio::*;

#[no_mangle]

pub extern fn kernel_main() {
    writes("Hello Rust Kernel world!");
    gpio_test();
    loop {
        let c: u8 = getc();
        if c == '1' as u8 {
            writes("YOU WROTE 1\n");
        } else {
            writes("YOU WROTE SOMETHING ELSE\n");
        }
    }
}

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    unsafe { abort() }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() { loop {} }
