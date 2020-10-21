#![no_std]
#![no_main]

use gba::io::display::{DisplayControlSetting, DisplayMode, DISPCNT};
use panic_abort as _;

mod sio;

#[no_mangle]
pub fn main() {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode3)
            .with_bg2(true),
    );
    unsafe {
        (0x0600_0000 as *mut u16)
            .add(240)
            .add(1)
            .write_volatile(0b11111);
    }

    sio::init(sio::BaudRate::Bps9600);

    loop {
        let c = sio::getc();
        sio::putc(c);
    }
}
