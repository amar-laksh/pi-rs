use comms::*;
use time::*;
use consts::*;
use gpio::*;

const GPIO_BASE: u32 = 0x20200000;

pub fn led_test() {
    writes("\nStarting:\tLED TEST\n");
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };
    for _ in 1..10  {
        writes("LED ON\n");
        unsafe { *(led_on) = 1 << 15; }
        sleep(500000);
        writes("LED OFF\n");
        unsafe { *(led_off) = 1 << 15; }
        sleep(500000);
    }
    writes("\nEnding:\tLED TEST\n");
}

pub fn gpio_test() {
    writes("\nStarting:\tGPIO TEST\n");
    pinMode(35, OUTPUT);
    pinMode(47, OUTPUT);
    pinMode(24, OUTPUT);
    for _ in 1..10 {
        digitalWrite(35, HIGH);
        digitalWrite(47, LOW);
        digitalWrite(24, LOW);
        sleep(20);
        digitalWrite(35, LOW);
        digitalWrite(47, HIGH);
        digitalWrite(24, HIGH);
        sleep(20);
    }
    digitalWrite(35, LOW);
    digitalWrite(47, LOW);
    digitalWrite(24, LOW);
    writes("\nEnding:\tGPIO TEST\n");
}
