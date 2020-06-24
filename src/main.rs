#![no_std] // no std in OS
#![no_main] // override entry point
// for tests
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

// main entry, no name mangle
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    blog_os::qemu::exit(blog_os::qemu::ExitCode::Success);

    println!("Hello, {}!", "world");
    loop {}
}

/// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

// tests
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
