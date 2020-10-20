#![no_std]
#![no_main]

use panic_abort as _;

#[no_mangle]
pub fn main() {
    unsafe {
        (0x0400_0000 as *mut u16).write_volatile(3 | (1 << 10));
        (0x0600_0000 as *mut u16)
            .add(240)
            .add(1)
            .write_volatile(0b11111);
        panic!()
    }
}
