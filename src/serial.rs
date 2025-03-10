#[cfg(target_arch = "arm")]
use super::armv6a::memory::map::mmio::PL011_UART_START;
#[cfg(target_arch = "arm")]
use arm_pl011_uart::{DataBits, LineConfig, OwnedMmioPointer, Parity, StopBits, Uart};
use core::fmt::Write;
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

#[cfg(target_arch = "arm")]
lazy_static! {
    pub static ref SERIAL1: Mutex<Uart<'static>> = {
        let mut serial_port = unsafe {
            Uart::new(OwnedMmioPointer::new(
                core::ptr::NonNull::new(PL011_UART_START as *mut _)
                    .expect("Unable to take serial port"),
            ))
        };
        // Baud rate and sysclock found here:
        // https://github.com/thanoskoutr/armOS/blob/6ae7f6bf5a5e812a35e731fc95e29e2cc1e3e7a8/src/kernel/uart.c#L86
        // Parity, data, and stop bits found here
        // https://krinkinmu.github.io/2020/11/29/PL011.html
        //
        // Already enabled??? Does not work with this config for QEMU
        //serial_port.enable(LineConfig {
        //    parity: Parity::None,
        //    data_bits: DataBits::Bits8,
        //    stop_bits: StopBits::One,
        //}, 115200, 270).expect("Unable to enable serial port");
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    #[cfg(target_arch = "x86_64")]
    use x86_64::instructions::interrupts;

    #[cfg(target_arch = "x86_64")]
    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });

    #[cfg(target_arch = "arm")]
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
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
