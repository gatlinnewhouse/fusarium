// Shout-out:
// https://github.com/BRA1L0R/raspberrypi-1-rust

use super::{
    framebuffer::{FrameBuffer, Painter},
    mailbox::MailBox,
};
use crate::armv6a::{
    drivers::framebuffer::{FrameBufferMail, Pixel},
    interrupts::without_interrupts,
};
use core::{
    slice::from_raw_parts_mut,
    sync::atomic::{AtomicBool, Ordering},
};

static mut VIDEO_AVAIL: AtomicBool = AtomicBool::new(true);

pub struct VideoDriver<'a> {
    pub buffer: FrameBuffer<'a>,
    mailbox: MailBox,
}

impl<'a> VideoDriver<'a> {
    pub fn take() -> Option<VideoDriver<'a>> {
        without_interrupts(|| unsafe {
            #[allow(static_mut_refs)]
            if !VIDEO_AVAIL.load(Ordering::Relaxed) {
                None
            } else {
                VIDEO_AVAIL.store(false, Ordering::Relaxed);
                without_interrupts(|| {
                    Some(VideoDriver {
                        buffer: FrameBuffer::unitialized(),
                        mailbox: MailBox::new(),
                    })
                })
            }
        })
    }
    pub fn init(&mut self, width: u32, height: u32) {
        const BIT_DEPTH: u32 = 24; // 8 bit RGB
        let fb = unsafe { &mut *((1 << 22) as *mut FrameBufferMail) };
        *fb = FrameBufferMail {
            width,
            height,
            virtual_width: width,
            virtual_height: height,
            pitch: 0,
            depth: BIT_DEPTH,
            x_offset: 0,
            y_offset: 0,
            pointer: 0,
            size: 0,
        };
        loop {
            self.mailbox.write_mail(fb);
            let conf = self.mailbox.read_mail();
            if conf.data().get() == 0 && fb.pointer != 0 {
                break;
            }
        }
        let buffer =
            unsafe { from_raw_parts_mut(fb.pointer as *mut Pixel, (fb.size / 3) as usize) };
        let fb = FrameBuffer::new(fb.pitch, fb.height, fb.width, buffer);
        self.buffer = fb;
    }
    pub fn painter(&'a mut self) -> Painter<'a> {
        Painter::new(&mut self.buffer)
    }
}
