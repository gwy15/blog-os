#![no_std] // no std in OS
#![no_main] // override entry point

#[no_mangle] // no name mangle
pub extern "C" fn _start() -> ! {
    const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in b"Hello, World".iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
