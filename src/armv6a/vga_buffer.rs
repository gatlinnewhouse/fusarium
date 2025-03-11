use core::fmt;

use crate::armv6a::drivers::{
    framebuffer::{HEIGHT, WIDTH},
    video::VideoDriver,
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::armv6a::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

static mut INIT: bool = false;

//TODO: create a buffer to write subsequent lines to

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use super::interrupts;
    let mut video = VideoDriver::take().unwrap();
    if unsafe { !INIT } {
        video.init(WIDTH, HEIGHT);
        unsafe { INIT = true };
    }
    let mut painter = video.painter();
    interrupts::without_interrupts(|| {
        painter.print_text(args);
    });
}
