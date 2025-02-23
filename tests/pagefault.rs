#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use fusarium::{exit_qemu, serial_print, serial_println, QemuExitCode};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("pagefault::pagefault...\t");

    fusarium::gdt::init();
    init_test_idt();

    // trigger a page fault
    pagefault();

    panic!("Execution continued after stack overflow");
}

#[cfg(target_arch = "x86_64")]
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(fusarium::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[cfg(target_arch = "x86_64")]
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}

fn pagefault() {
    // returns a 0 error_code, not sure how to test it yet
    let ptr = 0xdeadbeaf as *mut u8;
    unsafe {
        *ptr = 42;
    }
}
