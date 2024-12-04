// no std
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// rebuild start point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
