#![allow(dead_code)]
#![allow(non_snake_case)]

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

pub fn mini_uart_init(beginValue: i64) {
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
    delay_cycle(150);
    mmio_write(GPPUDCLK0,(1<<14));
    delay_cycle(150);
    mmio_write(GPPUD, 0);
    mmio_write(GPPUDCLK0,0);

    pinMode(15, ALT5);
    mmio_write(GPPUD, 0);
    delay_cycle(150);
    mmio_write(GPPUDCLK0,(1<<15));
    delay_cycle(150);
    mmio_write(GPPUD, 0);
    mmio_write(GPPUDCLK0,0);


    mmio_write(AUX_MU_CNTL_REG,3);
}

struct Serial {

}

impl Serial {
    fn available(&self) -> i64 {
        return 32;
    }

    fn availableForWrite(&self) -> i64 {
        return 32;
    }

    fn begin<T>(&self, beginValue: i64, config: T) {
        mini_uart_init(beginValue);
    }

    fn end(&self) {
    }

    fn find(&self, target: &str) -> bool {
        return true;
    }

    fn findUntil(&self, target: &str, terminal: &str) -> bool {
        return true;
    }

    fn flush(&self) {
    }

    fn parseFloat(&self) -> f64 {
        return 32.0;
    }

    fn parseInt(&self) -> i64 {
        return 32;
    }

    fn peek(&self) -> i64 {
        return 32;
    }

    fn print<T>(&self, val: T, format: T) -> i64 {
        return 32;
    }

    fn println<T>(&self, val: T, format: T) -> i64 {
        return 32;
    }

    fn read(&self) -> i64 {
        return 32;
    }

//    fn readBytes(&self, buffer: vec![char], length: i64) ->  u8 {
//        return 0;
//    }

//    fn readBytesUntil(&self, character: char, buffer: vec![char], length: i64) -> u8 {
//        return 0;
//    }

    fn setTimeout(&self, time: i64) {
    }

    fn write<T>(&self, val: T) {

    }

//    fn write(&self, buffer: vec![char], length: i64) -> u8 {
//        return 32;
//    }
}

pub fn serialEvent<T>(arg: T) {

}


