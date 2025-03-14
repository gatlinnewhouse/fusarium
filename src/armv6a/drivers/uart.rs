use core::{arch::asm, fmt::Write};

use arm_pl011_uart::{DataBits, Interrupts, LineConfig, OwnedMmioPointer, Parity, StopBits, Uart};
use spin::Mutex;

use crate::armv6a::{
    interrupts::{self, data_memory_barrier},
    memory::map::mmio::PL011_UART_START,
};

use super::gpio::GPIO;

pub struct Pl011Uart {
    pub inner: Mutex<Uart<'static>>,
}

impl Pl011Uart {
    pub fn new(gpio: &GPIO) -> Self {
        interrupts::without_interrupts(|| -> Pl011Uart {
            // map uart pins?
            gpio.map_pl011_uart();

            data_memory_barrier();
            // initialize uart connection
            let mut serial_port = Uart::new(unsafe {
                OwnedMmioPointer::new(
                    core::ptr::NonNull::new(PL011_UART_START as *mut _)
                        .expect("Unable to take serial port"),
                )
            });

            // Disable it in case it is already enabled
            //serial_port.disable();

            // Clear all pending interrupts
            //serial_port.clear_interrupts(Interrupts::all());

            // Baud rate and sysclock found here:
            // https://github.com/thanoskoutr/armOS/blob/6ae7f6bf5a5e812a35e731fc95e29e2cc1e3e7a8/src/kernel/uart.c#L86
            // Parity, data, and stop bits found here
            // https://krinkinmu.github.io/2020/11/29/PL011.html
            serial_port
                .enable(
                    LineConfig {
                        parity: Parity::None,
                        data_bits: DataBits::Bits8,
                        stop_bits: StopBits::One,
                    },
                    115_200,     // standard baud rate tbh
                    250_000_000, // in MHz
                )
                .expect("Unable to enable serial port");

            (0..300).for_each(|_| unsafe { asm!("nop") });

            serial_port.write_str("Ready!\n");

            // Return mutex with serial port
            Self {
                inner: Mutex::new(serial_port),
            }
        })
    }
}
