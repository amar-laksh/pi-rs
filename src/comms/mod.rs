#![allow(dead_code)]

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use consts::*;
use time::*;
use gpio::*;
use core::fmt::*;

pub fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

pub fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

pub fn mini_uart_putc(c: u8) {
	loop {
		if mmio_read(AUX_MU_LSR_REG) & 0x20 ==  1 { break; }
	}
	mmio_write(AUX_MU_IO_REG, c as u32);
}


pub fn mini_uart_getc() -> u8 {
	loop {
		if mmio_read(AUX_MU_LSR_REG) & 0x01 == 1 { break; }
	}
	return mmio_read(AUX_MU_IO_REG) as u8;
}

pub fn writes(msg: &str) {
    for c in msg.chars() {
        mini_uart_putc(c as u8);
    }
}

pub fn errors(msg: &str) {
}

pub fn writesn(msg: &str) {

}

pub fn errorsn(msg: &str) {

}

pub fn mini_uart_init() {
    mmio_write(AUX_ENABLES, 0x007);
    mmio_write(AUX_MU_IER_REG,0);
	mmio_write(AUX_MU_CNTL_REG,0);
	mmio_write(AUX_MU_LCR_REG,3);
	mmio_write(AUX_MU_MCR_REG,0);
	mmio_write(AUX_MU_IER_REG,0);

	mmio_write(AUX_MU_IIR_REG,0xC6);
	mmio_write(AUX_MU_BAUD_REG,270);

	pinMode(14, ALT5);
	mmio_write(GPPUD, 0);
	sleep_cycle(150);
	mmio_write(GPPUDCLK0,(1<<14));
	sleep_cycle(150);
	mmio_write(GPPUD, 0);
	mmio_write(GPPUDCLK0,0);

	pinMode(15, ALT5);
	mmio_write(GPPUD, 0);
	sleep_cycle(150);
	mmio_write(GPPUDCLK0,(1<<15));
	sleep_cycle(150);
	mmio_write(GPPUD, 0);
	mmio_write(GPPUDCLK0,0);


	mmio_write(AUX_MU_CNTL_REG,3);
}


