#[cfg(target_arch = "arm")]
use super::armv6a::{
    drivers::{gpio::GPIO, DRIVERS},
    interrupts::{self, without_interrupts},
    memory::map::mmio::PL011_UART_START,
};
#[cfg(target_arch = "arm")]
use arm_pl011_uart::{DataBits, Interrupts, LineConfig, OwnedMmioPointer, Parity, StopBits, Uart};
use core::{fmt::Write, ptr::write_volatile};
use lazy_static::lazy_static;
use spin::Mutex;
#[cfg(target_arch = "x86_64")]
use uart_16550::SerialPort;

#[cfg(target_arch = "x86_64")]
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    #[cfg(target_arch = "arm")]
    use crate::armv6a::interrupts;
    #[cfg(target_arch = "x86_64")]
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        #[cfg(target_arch = "x86_64")]
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
        #[cfg(target_arch = "arm")]
        unsafe {
            DRIVERS
                .get_uart()
                .lock()
                .write_fmt(args)
                .expect("Printing to serial failed");
        }
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
