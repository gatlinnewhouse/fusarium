#![no_std]
#![no_main]

#[cfg(target_arch = "x86_64")]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fusarium::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    #[allow(clippy::empty_loop)]
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    fusarium::hlt_loop();
}
