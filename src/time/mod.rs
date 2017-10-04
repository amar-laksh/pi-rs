#![allow(dead_code)]
pub fn delay(count: i32) {
    let _count = count * 100000;
    for _ in 1.._count {
        unsafe { asm!(""); }
    }
}

//pub fn sleep_cycle(count: i32) {
//    for _ in 1..count {
//        unsafe { asm!(""); }
//    }
//}


pub fn delay_cycle(_count: i32) {
    let mut count = &_count;
    unsafe {
        asm!("__delay_%=:subs %[count], %[count], #1; bne __delay_%=\n"
             : "=r"(count)
             : "0"(count)
             : "cc"
             );
    }
}

pub fn delayMicroseconds(count: i32) {
}

pub fn millis(count: i32) {
}

pub fn micros(count: i32) {
}
