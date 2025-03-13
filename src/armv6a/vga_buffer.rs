use crate::armv6a::drivers::{
    framebuffer::{HEIGHT, WIDTH},
    video::VideoDriver,
};
use conquer_once::spin::Once;
use core::{
    fmt::{self, Write},
    str::FromStr,
};
use heapless::{String, Vec};
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

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER: usize = BUFFER_HEIGHT * BUFFER_WIDTH;

pub struct Writer {
    buffer_queue: Vec<String<BUFFER_WIDTH>, BUFFER_HEIGHT>,
    buffer: String<BUFFER>,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer_queue: Vec::<String<BUFFER_WIDTH>, BUFFER_HEIGHT>::new(),
        buffer: String::new()
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
    pub fn write_string(&mut self, s: &str) {
        // Pop the last element, is None if not full
        if self.buffer_queue.is_full() {
            let _ = self.buffer_queue.pop();
        }
        if let Ok(t) = String::from_str(s) {
            _ = self.buffer.push_str(t.as_str());
            _ = self.buffer_queue.push(t);
        }
    }

    pub fn to_str(&self) -> &str {
        self.buffer.as_str()
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
