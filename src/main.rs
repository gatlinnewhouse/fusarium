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

#[cfg(target_arch = "x86_64")]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
#[cfg(feature = "exec-mine")]
use fusarium::task::executor::Executor;
#[cfg(feature = "exec-simple")]
use fusarium::task::simple_executor::SimpleExecutor;
use fusarium::{
    println,
    task::{keyboard, Task},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Import allocator and memory types
    use fusarium::allocator;
    use fusarium::memory::{self, BootInfoFrameAllocator};
    #[cfg(target_arch = "x86_64")]
    use x86_64::VirtAddr;
    // Hello World! debug print
    println!("Hello World{}", "!");

    // Initialize the global descriptor table and interrupts
    fusarium::init();

    // Initialize virtual memory and the heap allocator
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = memory::init(phys_mem_offset);
    let mut frame_allocator = BootInfoFrameAllocator::init(&boot_info.memory_map);
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Initialize an async executor
    #[cfg(feature = "exec-simple")]
    let mut executor = SimpleExecutor::new();
    #[cfg(feature = "exec-mine")]
    let mut executor = Executor::new();

    #[cfg(test)]
    test_main();

    // Test asynchronous runtime
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // Status debug print
    println!("It did not crash!");
    fusarium::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
