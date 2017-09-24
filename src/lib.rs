#![no_std]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8);
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    write("Hello Rust Kernel world!");
    loop {
        let c: u8 = getc();
        if c == '1' as u8 {
            write("YOU WROTE 1\n");
        } else {
            write("YOU WROTE SOMETHING ELSE\n");
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
