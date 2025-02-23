#![no_std]
#![no_main]

use core::panic::PanicInfo;
use fusarium::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail_doublefetch();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    #[allow(clippy::empty_loop)]
    loop {}
}

fn should_fail_doublefetch() {
    serial_print!("should_panic::should_fail_doublefetch...\t");
    // returns a 0 error_code, not sure how to test it yet
    #[cfg(target_arch = "x86_64")]
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
}
