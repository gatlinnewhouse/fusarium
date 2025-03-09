/// Physical memory map
// From https://github.com/thanoskoutr/armOS/wiki/Raspberry-Pi-Hardware
pub(super) mod map {
    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    /// Physical device addresses
    pub mod mmio {
        use super::*;
        pub const BASE: usize = 0x2000_0000;
        pub const GPIO_START: usize = BASE + GPIO_OFFSET;
        pub const PL011_UART_START: usize = BASE + UART_OFFSET;
    }
}
