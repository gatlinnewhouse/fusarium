#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fusarium::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

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
fn page_read() {
    let ptr = 0x2031b2 as *mut u8;

    // read from a code page
    let x;
    unsafe {
        x = *ptr;
        assert_eq!(x, *ptr);
    }
    assert_eq!(x, unsafe { *ptr });
}
