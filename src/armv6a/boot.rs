// Import the assembly file
core::arch::global_asm!(include_str!("boot.s"), options(raw));

/// Help from https://github.com/carloskiki/rpi-devenv
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    crate::kernel_main()
}
