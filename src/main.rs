// no std
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

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
    println!();
    // Conditional Compilation
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    print!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    /// new
    exit_qemu(QemuExitcode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(2 + 2, 4);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitcode {
    Success = 0x10,
    Fail = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitcode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
