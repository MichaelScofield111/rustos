// no std
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffers;
/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World! Rust-os~";

/// rebuild start point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    // test
    //vga_buffers::print_something();

    use core::fmt::Write;
    println!("test println macro");
    vga_buffers::WRITER.lock().write_str("Hello rust-os!");
    write!(
        vga_buffers::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}
