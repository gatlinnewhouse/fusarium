#![no_std]
#![no_main]
//#![warn(missing_docs)]
//#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]
#![feature(custom_test_frameworks)]
#![test_runner(fusarium::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fusarium::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use fusarium::allocator;
    use fusarium::memory::{self, BootInfoFrameAllocator};
    #[cfg(target_arch = "x86_64")]
    use x86_64::VirtAddr;
    println!("Hello World{}", "!");
    fusarium::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = memory::init(phys_mem_offset);
    let mut frame_allocator = BootInfoFrameAllocator::init(&boot_info.memory_map);
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    fusarium::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fusarium::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}
