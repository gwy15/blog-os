#![no_std] // no std in OS
#![no_main] // override entry point
// for tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
mod serial;
mod vga_buffer;

// main entry, no name mangle
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test mode
    #[cfg(test)]
    test_main();

    println!("Hello, {}!", "world");
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(test)]
    {
        serial_println!("[failed]");
        serial_println!("{}", info);
        test::qemu::exit_qemu(test::qemu::QemuExitCode::Failed);
    }

    // print panic info and do nothing :)
    println!("{}", info);
    loop {}
}

#[cfg(test)]
mod test {
    use super::*;

    pub trait Testable {
        fn run(&self);
    }

    impl<T> Testable for T
    where
        T: Fn(),
    {
        fn run(&self) {
            serial_print!("{}... \t", core::any::type_name::<Self>());
            self();
            serial_println!("[ok]");
        }
    }

    pub fn test_runner(tests: &[&dyn Testable]) {
        println!("Running {} tests", tests.len());
        for test in tests {
            test.run();
        }
        qemu::exit_qemu(qemu::QemuExitCode::Success);
    }

    pub mod qemu {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u32)]
        pub enum QemuExitCode {
            Success = 0x10,
            Failed = 0x11,
        }

        pub fn exit_qemu(exit_code: QemuExitCode) {
            use x86_64::instructions::port::Port;

            unsafe {
                let mut port = Port::new(0xf4);
                port.write(exit_code as u32);
            }
        }
    }

    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
