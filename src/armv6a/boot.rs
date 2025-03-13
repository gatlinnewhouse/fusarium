use fusarium::armv6a::{
    self,
    drivers::{self, Mgmnt, DRIVERS},
};

// Import the assembly file
core::arch::global_asm!(include_str!("boot.s"), options(raw));

/// Help from <https://github.com/carloskiki/rpi-devenv>
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    // Safety: This value is provided by the linker script.
    unsafe extern "C" {
        #[link_name = "__physical_load_address"]
        static LOAD_ADDRESS: u8;
    }
    kernel_init()
}

fn kernel_init() -> ! {
    armv6a::interrupts::enable();
    unsafe { DRIVERS.init() }
    crate::kernel_main()
}
