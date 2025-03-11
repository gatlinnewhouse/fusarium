use crate::armv6a::drivers::{
    framebuffer::{HEIGHT, WIDTH},
    video::VideoDriver,
};
use conquer_once::spin::Once;
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use super::{drivers::framebuffer::Painter, interrupts};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::armv6a::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

//TODO: Add ansi support, unify vga_buffer codebase with x86_64

//TODO: fix the buffer to write subsequent lines to

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[derive(Clone)]
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<u8>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    buffer: Buffer,
}

lazy_static! {
    static ref BUF: Buffer = Buffer {
        chars: core::array::from_fn::<_, BUFFER_HEIGHT, _>(|_| {
            core::array::from_fn::<_, BUFFER_WIDTH, _>(|_| Volatile::new(b' '))
        }),
    };
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        buffer: BUF.clone(),
    });
}

static mut VIDEO: VideoDriver<'static> = VideoDriver::new();
static VID_INIT: Once = Once::uninit();

lazy_static! {
    pub static ref VGA: Mutex<Painter<'static>> = unsafe {
        interrupts::without_interrupts(|| -> Mutex<Painter<'static>> {
            VID_INIT.init_once(|| {
                VIDEO = VideoDriver::take().unwrap();
                VIDEO.init(WIDTH, HEIGHT);
            });
            Mutex::new(VIDEO.painter())
        })
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use super::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
        VGA.lock().print_text(WRITER.lock().to_str());
    });
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(byte);
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(b' ');
        }
    }
    fn to_str(&self) -> &str {
        static mut BUFFER_STR: [u8; BUFFER_WIDTH * BUFFER_HEIGHT] =
            [b' '; BUFFER_WIDTH * BUFFER_HEIGHT];
        let mut idx = 0;
        for row in self.buffer.chars.iter() {
            for byte in row.iter() {
                unsafe {
                    BUFFER_STR[idx] = byte.read();
                }
                idx += 1;
            }
        }
        #[allow(static_mut_refs)]
        unsafe {
            core::str::from_utf8(&BUFFER_STR).unwrap()
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
