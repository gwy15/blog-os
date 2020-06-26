#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use blog_os::{qemu, serial_print, serial_println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("test kernel_stack_overflow...\t");

    blog_os::init();
    TEST_IDT.load();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
        volatile::Volatile::new(0).read();
    }
    stack_overflow();

    panic!("Execution continued after kernel stack overflowed.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("\x1B[32m[ok]\x1B[0m");
    qemu::exit(qemu::ExitCode::Success);
    panic!("Qemu should exit.");
}
