#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
pub mod allocator;
#[cfg(target_arch = "arm")]
pub mod armv6a;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64/gdt.rs"]
pub mod gdt;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64/interrupts.rs"]
pub mod interrupts;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64/memory.rs"]
pub mod memory;
pub mod serial;
#[cfg(target_arch = "x86_64")]
pub mod task;
#[cfg(target_arch = "x86_64")]
pub mod vga_buffer;

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    {
        gdt::init();
        interrupts::init_idt();
        unsafe { interrupts::PICS.lock().initialize() };
        x86_64::instructions::interrupts::enable();
    }
}

pub fn hlt_loop() -> ! {
    loop {
        #[cfg(target_arch = "x86_64")]
        x86_64::instructions::hlt();
        #[cfg(target_arch = "arm")]
        aarch64_cpu::asm::wfe();
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    #[cfg(target_arch = "x86_64")]
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    #[cfg(target_arch = "x86_64")]
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(target_arch = "x86_64")]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
use bootloader::{entry_point, BootInfo};

#[cfg(all(test, target_arch = "x86_64"))]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    // like before
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    #[cfg(target_arch = "x86_64")]
    x86_64::instructions::interrupts::int3();
    #[cfg(target_arch = "arm")]
    aarch64_cpu::asm::bkpt();
}
