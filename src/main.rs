#![no_std] // no std in OS
#![no_main] // override entry point

mod vga_buffer;

#[no_mangle] // no name mangle
pub extern "C" fn _start() -> ! {
    println!("This should {}!", "work");
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
