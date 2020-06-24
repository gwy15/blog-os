#![no_std] // no std in OS
#![no_main] // override entry point

#[no_mangle] // no name mangle
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
