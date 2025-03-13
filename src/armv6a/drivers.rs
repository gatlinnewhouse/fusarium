use arm_pl011_uart::Uart;
use core::{marker::PhantomData, ops};
use gpio::GPIO;
use lazy_static::lazy_static;
use spin::Mutex;
use uart::Pl011Uart;

use super::memory::map::mmio::GPIO_START;

/// Framebuffer
pub mod framebuffer;
/// GPIO pins
pub mod gpio;
/// Mailbox
pub mod mailbox;
/// PL011 UART
pub mod uart;
/// Video driver
pub mod video;

/// Memory mapped IO dereference wrapper
pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create a new MMIO deref wrapper
    pub const fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;
    /// Return the wrapped type from the wrapper
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}

pub static mut DRIVERS: Mgmnt = Mgmnt::new();

pub struct Mgmnt {
    gpio: Option<GPIO>,
    pub uart: Option<Pl011Uart>,
}

impl Mgmnt {
    const fn new() -> Self {
        Self {
            gpio: None,
            uart: None,
        }
    }
    pub fn init(&mut self) {
        self.gpio = Some(GPIO::new(GPIO_START));
        self.uart = Some(Pl011Uart::new(self.gpio.as_ref().unwrap()));
    }
    pub fn get_uart(&self) -> &'static Mutex<Uart> {
        &self.uart.as_ref().unwrap().inner
    }
}
