mod io;
pub mod io {
    use core::intrinsics::volatile_store;
    use core::intrinsics::volatile_load;
    pub fn write(reg: u32, val: u32) {
        unsafe { volatile_store(reg as *mut u32, val) }
    }

    pub fn read(reg: u32) -> u32 {
        unsafe { volatile_load(reg as *const u32) }
    }
}
