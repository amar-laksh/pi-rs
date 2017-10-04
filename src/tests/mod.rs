use comms::*;
use time::*;
use consts::*;
use gpio::*;

pub fn led_test() {
    writesn("Starting:\tGPIO TEST");
    pinMode(35, OUTPUT);
    pinMode(47, OUTPUT);
    pinMode(24, OUTPUT);
    for _ in 1..10 {
        digitalWrite(35, HIGH);
        digitalWrite(47, LOW);
        digitalWrite(24, LOW);
        delay(20);
        digitalWrite(35, LOW);
        digitalWrite(47, HIGH);
        digitalWrite(24, HIGH);
        delay(20);
    }
    digitalWrite(35, LOW);
    digitalWrite(47, LOW);
    digitalWrite(24, LOW);
    writesn("Ending:\tGPIO TEST");
}
