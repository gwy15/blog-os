#![no_std] // no std in OS
#![no_main] // override entry point
// for tests
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

// main entry, no name mangle
#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("Hello, {}!", "world");
    blog_os::halt_loop();
}

/// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::halt_loop();
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
