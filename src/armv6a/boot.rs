use fusarium::armv6a::{
    self,
    drivers::{self, Mgmnt, DRIVERS},
};

// Import the assembly file
core::arch::global_asm!(include_str!("boot.s"), options(raw));

#[export_name = "rust_irq_handler"]
pub extern "C" fn irq_handler() {}

/// Help from <https://github.com/carloskiki/rpi-devenv>
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    kernel_init()
}

fn kernel_init() -> ! {
    armv6a::interrupts::enable();
    unsafe { DRIVERS.init() }
    crate::kernel_main()
}
