use core::arch::asm;

use super::MMIODerefWrapper;
use crate::armv6a::{interrupts::data_memory_barrier, memory::map::mmio::GPIO_START};
use spin::Mutex;
use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

// rust-raspberrypi-OS-tutorials
// and this blog: https://litchipi.site/post/17611351315151745365
// helped me understand what was going on here
register_bitfields! {
    u32,

    /// GPIO Function select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART RX
        ],

        /// Pin 14
        FSEL14 OFFSET(14) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART TX
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _unused1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _unused2),
        (0x94 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

struct GPIOInner {
    regs: Registers,
}

pub struct GPIO {
    inner: Mutex<GPIOInner>,
}

impl GPIOInner {
    pub const fn new(mmio_start_addr: usize) -> Self {
        Self {
            regs: Registers::new(mmio_start_addr),
        }
    }
    pub fn map_pl011_uart(&mut self) {
        //data_memory_barrier();
        self.regs
            .GPFSEL1
            .modify(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);
        //(0..300).for_each(|_| unsafe { asm!("nop") });
    }
}

impl GPIO {
    pub const fn new(start_addr: usize) -> Self {
        Self {
            inner: Mutex::new(GPIOInner::new(start_addr)),
        }
    }
    pub fn map_pl011_uart(&self) {
        self.inner.lock().map_pl011_uart();
    }
}
