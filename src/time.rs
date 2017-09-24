pub fn sleep(count: i32) {
    let _count = count * 100000;
    for _ in 1.._count {
        unsafe { asm!(""); }
    }
}

pub fn sleep_cycle(count: i32) {
    for _ in 1..count {
        unsafe { asm!(""); }
    }
}
