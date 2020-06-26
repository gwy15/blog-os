use lazy_static::lazy_static;

use crate::{print, println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// idt setup
mod idt {
    use super::*;
    use crate::gdt;

    // the IDT
    lazy_static! {
        // set handler in lazy_static macro so we don't need a static mutable object.
        static ref IDT: InterruptDescriptorTable = {
            let mut idt = InterruptDescriptorTable::new();
            // breakpoint handler
            idt.breakpoint.set_handler_fn(breakpoint_handler);
            // double fault handler
            unsafe {
                idt.double_fault
                    .set_handler_fn(double_fault_handler)
                    .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            };
            // external interrupts
            idt[external::InterruptIndex::Timer.as_usize()].set_handler_fn(external::timer_interrupt_handler);
            idt[external::InterruptIndex::Keyboard.as_usize()].set_handler_fn(external::keyboard_interrupt_handler);
            // return idt to static object
            idt
        };
    }

    // interrupt handlers
    extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
        println!("EXCEPTION: BREAKPOINT {:#?}", stack_frame);
        // TODO:
    }

    extern "x86-interrupt" fn double_fault_handler(
        stack_frame: &mut InterruptStackFrame,
        _error_code: u64,
    ) -> ! {
        // TODO:
        panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    }

    pub fn init() {
        IDT.load();
    }
}

// external interrupts
mod external {
    use super::*;
    use pic8259_simple::ChainedPics;
    // use spin;
    const PIC_1_OFFSET: u8 = 32;
    const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
    static PICS: spin::Mutex<ChainedPics> =
        spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum InterruptIndex {
        Timer = PIC_1_OFFSET,
        Keyboard,
    }

    impl InterruptIndex {
        pub fn as_u8(self) -> u8 {
            self as u8
        }

        pub fn as_usize(self) -> usize {
            usize::from(self.as_u8())
        }
    }

    #[inline]
    fn end_interrupt(idx: InterruptIndex) {
        unsafe {
            PICS.lock().notify_end_of_interrupt(idx.as_u8());
        }
    }

    pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
        print!(".");
        end_interrupt(InterruptIndex::Timer);
    }

    pub extern "x86-interrupt" fn keyboard_interrupt_handler(
        _stack_frame: &mut InterruptStackFrame,
    ) {
        use x86_64::instructions::port::Port;
        const IO_PORT: u16 = 0x60;
        let mut port = Port::new(IO_PORT);
        let scan_code: u8 = unsafe { port.read() };

        use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
        lazy_static! {
            static ref KEYBOARD: spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
                spin::Mutex::new(Keyboard::new(
                    layouts::Us104Key,
                    ScancodeSet1,
                    HandleControl::Ignore
                ));
        }

        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(ch) => {
                        print!("{}", ch);
                    }
                    DecodedKey::RawKey(key_code) => {
                        println!("keycode ({:?})", key_code);
                    }
                }
            }
        }

        end_interrupt(InterruptIndex::Keyboard);
    }

    pub fn init() {
        // initialize programmable interrupt controller
        unsafe {
            PICS.lock().initialize();
        }
        // enable external interrupts
        x86_64::instructions::interrupts::enable();
    }
}

// public methods
pub fn init() {
    // initialize interrupt descriptor table
    idt::init();
    //
    external::init();
}

// tests
#[cfg(test)]
mod test {
    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }

    // double fault are tested in tests/kernel_stack_overflow
}
