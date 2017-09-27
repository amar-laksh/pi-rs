#![allow(non_snake_case)]
#![allow(dead_code)]

use consts::*;
use comms::*;
use time::sleep_cycle;


//////////////////////// HELPER FUNCTIONS /////////////////

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

fn check_pin_sel_read(pin: i8
                      , sel1: u32
                      , sel2: u32) -> u32 {
    return match pin { 0...31 => { sel1 }, _ => { sel2 }};
}

fn check_pin_sel_write(pin: i8
                       , sel1: u32
                       , sel2: u32
                       , value: u32) {
    return match pin {
        0...31 => { mmio_write(sel1, value<<(32 - pin)) },
        _ => { mmio_write(sel2, value<<(pin - 32))}
    }
}

//////////////////////// READ FUNCTIONS ///////////////////

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

pub fn digitalRead(pin: i8) -> u32 {
    let gplev: u32 = check_pin_sel_read(pin, GPLEV0, GPLEV1);
        return mmio_read(gplev)  & (1<<(pin - (32 & pin)));
}

pub fn eventRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPEDS0, GPEDS1);
    return gpsel & (1<<pin);
}

pub fn risingEdgeRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPREN0, GPREN1);
    return gpsel & (1<<pin);
}

pub fn fallingEdgeRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPFEN0, GPFEN1);
    return gpsel & (1<<pin);
}

pub fn highDetectRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPHEN0, GPHEN1);
    return gpsel & (1<<pin);
}

pub fn lowDetectRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPLEN0, GPLEN1);
    return gpsel & (1<<pin);
}

pub fn asyncRisingEdgeRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPAREN0, GPAREN1);
    return gpsel & (1<<pin);
}

pub fn asyncFallingEdgeRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPAFEN0, GPAFEN1);
    return gpsel & (1<<pin);
}

pub fn pullUpDownRead() -> u32 {
    return mmio_read(GPPUD);
}

pub fn pullUpDownClockRead(pin: i8) -> u32 {
    let gpsel: u32 = check_pin_sel_read(pin, GPPUDCLK0, GPPUDCLK1);
    return gpsel & (1<<pin);
}

//////////////////////// WRITE FUNCTIONS //////////////////

pub fn digitalWrite(pin: i8, state: i8) {
    let value: u32 = 1<<(pin - (32 & pin));
    match state {
        HIGH => {
            match pin {
                0...31 => {
                    mmio_write(GPSET0, value);
                },
                32...53 => {
                    mmio_write(GPSET1, value);
                },
                _ => errorsn("invalid pin chosen")
            }
        },

        _ => {
            match pin {
                0...31 => {
                    mmio_write(GPCLR0, value);
                },
                32...53 => {
                    mmio_write(GPCLR1, value);
                },
            _ => errorsn("invalid pin chosen")
            }
        }
    }
}


pub fn eventWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPEDS0, GPEDS1, value);
}

pub fn risingEdgeWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPREN0, GPREN1, value);
}

pub fn fallingEdgeWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPFEN0, GPFEN1, value);
}

pub fn highDetectWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPHEN0, GPHEN1, value);
}

pub fn lowDetectWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPLEN0, GPLEN1, value);
}

pub fn asyncRisingEdgeWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPAREN0, GPAREN1, value);
}

pub fn pullUpDownWrite(value: u32) {
    mmio_write(GPPUD, value);
}

pub fn pullUpDownClockWrite(pin: i8, value: u32) {
    check_pin_sel_write(pin, GPPUDCLK0, GPPUDCLK1, value);
}
