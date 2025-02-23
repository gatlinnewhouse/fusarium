#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fusarium::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fusarium::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    fusarium::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
