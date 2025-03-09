use lazy_static::lazy_static;
#[cfg(target_arch = "arm")]
use rpi::eio::Write;
use spin::Mutex;
#[cfg(target_arch = "arm")]
use uart_16550::MmioSerialPort;
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
    pub static ref SERIAL1: Mutex<MmioSerialPort> = {
        let mut serial_port = unsafe { MmioSerialPort::new(0x3F20_1000) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

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
    critical_section::with(|_| unsafe {
        rpi::interrupt::disable();
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
        rpi::interrupt::enable();
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
