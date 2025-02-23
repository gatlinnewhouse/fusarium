use crate::println;
use lazy_static::lazy_static;
#[cfg(target_arch = "x86_64")]
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[cfg(target_arch = "x86_64")]
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

#[cfg(target_arch = "x86_64")]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
