// Import the assembly file
core::arch::global_asm!(include_str!("boot.s"), options(raw));

/// Help from https://github.com/carloskiki/rpi-devenv
pub extern "C" fn first_stage() -> ! {
    // Safety: This value is provided by the linker script.
    unsafe extern "C" {
        //TODO: fix this to use dtb/etc
        #[link_name = "__physical_load_address"]
        static LOAD_ADDRESS: u8;
    }
    // Safety: We know that the kernel is a function that never returns.
    // We also have loaded it into memory at LOAD_ADDRESS.
    unsafe {
        let kernel: fn() -> ! = core::mem::transmute(&raw const LOAD_ADDRESS);
        kernel();
    }
}
