#![no_main]
#![no_std]
//#![warn(missing_docs)]
//#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]
#![feature(custom_test_frameworks)]
#![test_runner(fusarium::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(target_arch = "x86_64")]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
#[cfg(target_arch = "x86_64")]
use fusarium::println;
#[cfg(all(target_arch = "x86_64", feature = "exec-mine", not(test)))]
use fusarium::task::executor::Executor;
#[cfg(all(target_arch = "x86_64", feature = "exec-simple", not(test)))]
use fusarium::task::simple_executor::SimpleExecutor;
#[cfg(all(target_arch = "x86_64", not(test)))]
use fusarium::task::{keyboard, Task};

#[cfg(target_arch = "arm")]
#[path = "armv6a/boot.rs"]
pub mod boot;

#[cfg(target_arch = "arm")]
fn kernel_main() -> ! {
    use fusarium::{println, serial_println};

    serial_println!("Hello");
    println!("Hello");
    println!("From {}", "Rust");
    println!("My lines scroll now, {} lines", 25);
    println!("2+2={}", 2 + 2);
    panic!("Stopping");
}

#[cfg(target_arch = "x86_64")]
entry_point!(kernel_main);

#[cfg(target_arch = "x86_64")]
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
    #[cfg(all(feature = "exec-simple", not(test)))]
    let mut executor = SimpleExecutor::new();
    #[cfg(all(feature = "exec-mine", not(test)))]
    let mut executor = Executor::new();

    #[cfg(test)]
    test_main();

    // Test asynchronous runtime
    #[cfg(not(test))]
    executor.spawn(Task::new(example_task()));
    #[cfg(not(test))]
    executor.spawn(Task::new(keyboard::print_keypresses()));
    #[cfg(not(test))]
    executor.run();

    // Status debug print
    println!("It did not crash!");
    fusarium::hlt_loop();
}

#[cfg(all(target_arch = "x86_64", not(test)))]
async fn async_number() -> u32 {
    42
}

#[cfg(all(target_arch = "x86_64", not(test)))]
async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/// This function is called on panic.
#[cfg(all(target_arch = "x86_64", not(test)))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fusarium::hlt_loop();
}

#[cfg(all(target_arch = "arm", not(test)))]
fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    fusarium::hlt_loop();
}

#[cfg(all(target_arch = "arm", not(test)))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use fusarium::serial_println;
    panic_prevent_reenter();

    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    serial_println!(
        "Kernel panic!\n\n\
        Panic location:\n      File '{}', line {}, column {}\n\n\
        {}",
        location,
        line,
        column,
        info.message()
    );

    fusarium::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}
