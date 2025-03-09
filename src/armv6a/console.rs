use core::fmt::{Result, Write};

pub trait Statistics {
    /// Return the number of characters written.
    fn chars_written(&self) -> usize {
        0
    }
}

#[derive(Default)]
pub struct QEMUOutput {
    chars_writen: usize,
}

impl QEMUOutput {
    /// Create a new QEMU output object
    pub const fn new() -> QEMUOutput {
        QEMUOutput { chars_writen: 0 }
    }

    /// Write chars to QEMUOuput
    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(
                crate::memory::map::mmio::PL011_UART_START as *mut u8,
                c as u8,
            );
        }
    }
}

impl Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            // Convert newline to carrige return + newline.
            if c == '\n' {
                self.write_char('\r')
            }
            self.write_char(c);
        }
        Ok(())
    }
}

impl Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.chars_writen
    }
}
