#![no_std]
#![no_main]

#[cfg(target_arch = "x86_64")]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fusarium::exit_qemu;

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    use fusarium::serial_print;
    serial_print!("address_translations::virt_to_phys...\t");
    virt_to_phys(boot_info);

    fusarium::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fusarium::test_panic_handler(info)
}

fn virt_to_phys(boot_info: &'static BootInfo) {
    use fusarium::{memory, serial_println};
    #[cfg(target_arch = "x86_64")]
    use x86_64::{structures::paging::Translate, VirtAddr};
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = memory::init(phys_mem_offset);
    // the identity-mapped vga buffer page
    #[cfg(target_arch = "x86_64")]
    let v = VirtAddr::new(0xb8000);
    let p = mapper
        .translate_addr(v)
        .expect("Could not translate virtual address to physical address\n");
    assert_eq!(v.as_u64(), p.as_u64());
    serial_println!("[ok]");
    exit_qemu(fusarium::QemuExitCode::Success);
}
