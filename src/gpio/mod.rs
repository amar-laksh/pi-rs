#![allow(non_snake_case)]
#![allow(dead_code)]

use consts::*;
use comms::*;
use time::sleep_cycle;

fn get_offset(pin: i8) -> u32 {
    let mut offset: u32 = 21;
    match pin % 10 {
            0 => offset = 0,
            1 => offset = 3,
            2 => offset = 6,
            3 => offset = 9,
            4 => offset = 12,
            5 => offset = 15,
            6 => offset = 18,
            7 => offset = 21,
            8 => offset = 24,
            9 => offset = 27,
            _ => errorsn("Sorry you choose an invalid pin number")
        }
    return offset;
}

fn get_selector(pin: i8) -> u32 {
    let mut sel: u32 = 0;
    match pin {
        0...9 => sel = GPFSEL0,
        10...19 => sel = GPFSEL1,
        20...29 => sel = GPFSEL2,
        30...39 => sel = GPFSEL3,
        40...49 => sel = GPFSEL4,
        50...53 => sel = GPFSEL5,
        _ => errorsn("I can't set the gpio selector")
    }
    return sel;
}

fn set_pin_function(sel: u32, offset: u32, value :u32)  {
    let mut ra: u32;
    ra = mmio_read(sel);
    ra &= !(7<<offset);
    ra |= value<<offset;
    mmio_write(sel, ra);
}
pub fn getPinMode(pin: i8) -> u32 {
    let value: u32;
    let offset: u32 = get_offset(pin);
    let sel: u32 = get_selector(pin);
    value = mmio_read(sel);
    return value & (7<<offset);
}

pub fn pinMode(pin: i8, state :i8) {
    let offset: u32 = get_offset(pin);
    let sel: u32 = get_selector(pin);
    match state {
        OUTPUT => {
            set_pin_function(sel, offset, 1);
        },

        INPUT => {
            set_pin_function(sel, offset, 0);

            mmio_write(GPPUD, 2);
            sleep_cycle(150);
            mmio_write(GPPUDCLK0, (1<<pin));
            sleep_cycle(150);
            mmio_write(GPPUD, 0);
            mmio_write(GPPUDCLK0, 0);
        },

        ALT0 => {
            set_pin_function(sel, offset, 4);
        },

        ALT1 => {
            set_pin_function(sel, offset, 5);
        },

        ALT2 => {
            set_pin_function(sel, offset, 6);
        },

        ALT4 => {
            set_pin_function(sel, offset, 3);
        },

        ALT5 => {
            set_pin_function(sel, offset, 2);
        },

        _ => errorsn("Sorry there is no such function")
    }
}


