#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
// tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    halt_loop();
}

pub fn init() {
    // initialize global descriptor table
    gdt::init();
    // initialize interrupts (idt and external interrupts)
    interrupts::init();
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("test {}...\t", core::any::type_name::<T>());
        self();
        serial_println!("\x1B[32m[ok]\x1B[0m");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("\x1B[31m[failed]\x1B[0m");
    serial_println!("Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Failed);
    halt_loop();
}

pub mod qemu {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ExitCode {
        Success = 0x10,
        Failed = 0x11,
    }

    pub fn exit(exit_code: ExitCode) {
        use x86_64::instructions::port::Port;

        unsafe {
            let mut port = Port::new(0xf4);
            port.write(exit_code as u32);
        }
    }
}
