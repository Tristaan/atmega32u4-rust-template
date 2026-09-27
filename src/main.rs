#![no_std]
#![no_main]
use core::panic::PanicInfo;
use avr_device::atmega32u4::Peripherals;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let _peripherals = Peripherals::take().unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
