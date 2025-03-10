/// Physical memory map
// From https://github.com/thanoskoutr/armOS/wiki/Raspberry-Pi-Hardware
pub mod map {
    pub const INTERRUPT_OFFSET: usize = 0x0000_B200;
    pub const VIDEOCORE_MBOX_OFFSET: usize = 0x0000_B880;
    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    /// Physical device addresses
    pub mod mmio {
        use super::*;
        pub const BASE: usize = 0x2000_0000;
        pub const INTERRUPT_BASE: usize = BASE + INTERRUPT_OFFSET;
        pub const VIDEOCORE_MBOX_BASE: usize = BASE + VIDEOCORE_MBOX_OFFSET;
        pub const GPIO_START: usize = BASE + GPIO_OFFSET;
        pub const PL011_UART_START: usize = BASE + UART_OFFSET;

        /// IRQs
        pub mod irq {
            use super::INTERRUPT_BASE;
            pub const IRQ_BASIC_PENDING: *mut u32 = INTERRUPT_BASE as *mut u32;
            pub const IRQ_PENDING_1: *mut u32 = (INTERRUPT_BASE + 0x04) as *mut u32;
            pub const IRQ_PENDING_2: *mut u32 = (INTERRUPT_BASE + 0x08) as *mut u32;
            pub const ENABLE_IRQS_1: *mut u32 = (INTERRUPT_BASE + 0x10) as *mut u32;
            pub const ENABLE_IRQS_2: *mut u32 = (INTERRUPT_BASE + 0x14) as *mut u32;
        }
    }
}
