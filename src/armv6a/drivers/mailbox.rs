// Mailbox implementation from
// https://github.com/BRA1L0R/raspberrypi-1-rust

use crate::armv6a::{
    interrupts::data_memory_barrier,
    memory::map::mmio::video::{MAIL0_READ, MAIL0_STATUS, MAIL0_WRITE},
};

#[derive(Debug)]
pub struct Mail(u32);
pub struct MailChannel(Mail);
pub struct MailData(Mail);
pub struct MailBox();
pub struct MailStatus(u32);

pub trait Mailable {
    fn write_mail(&mut self) -> Mail;
}

impl MailStatus {
    /// Check if second to last bit is not filled
    pub fn is_empty(&self) -> bool {
        ((self.0 & (1 << 30)) != 0) as bool
    }
    /// Check if last bit is filled
    pub fn is_full(&self) -> bool {
        ((self.0 & (1 << 31)) != 0) as bool
    }
}

impl Mail {
    pub fn new() -> Mail {
        Mail(0)
    }
    pub fn data(self) -> MailData {
        MailData(self)
    }
    pub fn chan(self) -> MailChannel {
        MailChannel(self)
    }
}

impl MailData {
    pub fn set(mut self, data: u32) -> Mail {
        // I think these 0b1111 values are the masks described by the ARM manual
        self.0 .0 = (self.0 .0 & 0b1111) + (data << 4);
        self.0
    }
    pub fn set_noshift(mut self, data: u32) -> Mail {
        self.0 .0 = (self.0 .0 & 0b1111) + (data & !(0b1111));
        self.0
    }
    pub fn get(self) -> u32 {
        self.0 .0 >> 4
    }
}

impl MailChannel {
    pub fn set(mut self, channel: u8) -> Mail {
        self.0 .0 = (self.0 .0 & !(0b1111)) + (channel as u32 & 0b1111);
        self.0
    }
    pub fn get(self) -> u8 {
        (self.0 .0 & 0b1111) as u8
    }
}

impl MailBox {
    pub fn new() -> MailBox {
        MailBox()
    }

    pub fn write_mail(&mut self, mail: &mut impl Mailable) {
        while self.mail_status().is_full() {}
        unsafe {
            MAIL0_WRITE.write_volatile(mail.write_mail().into());
        }
        data_memory_barrier();
    }

    pub fn read_mail(&mut self) -> Mail {
        while self.mail_status().is_empty() {}
        data_memory_barrier();
        unsafe { MAIL0_READ.read_volatile().into() }
    }

    pub fn mail_status(&self) -> MailStatus {
        data_memory_barrier();
        unsafe { MAIL0_STATUS.read_volatile().into() }
    }
}

impl Default for Mail {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MailBox {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Mail> for u32 {
    fn from(value: Mail) -> Self {
        value.0
    }
}

impl From<u32> for Mail {
    fn from(value: u32) -> Self {
        Mail(value)
    }
}

impl From<u32> for MailStatus {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
