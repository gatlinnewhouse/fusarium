#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fusarium::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(target_arch = "x86_64")]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fusarium::println;

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();

    fusarium::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
