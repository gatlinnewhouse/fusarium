#[cfg(target_arch = "arm")]
use core::fmt::Result;
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

//lazy_static! {
//    pub static ref SERIAL1: Mutex<Uart> = {
//        let mut serial_port = unsafe { arm_pl011_uart::OwnedMmioPointer::new(arm_pl011_uart::PL011Registers::) };
//        serial_port.init();
//        Mutex::new(serial_port)
//    };
//}

#[cfg(target_arch = "arm")]
pub(crate) struct QEMUOutput;

#[cfg(target_arch = "arm")]
impl Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(crate::memory::UART_BASE as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

#[cfg(target_arch = "arm")]
lazy_static! {
    pub(crate) static ref SERIAL1: Mutex<QEMUOutput> = Mutex::new(QEMUOutput {});
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
