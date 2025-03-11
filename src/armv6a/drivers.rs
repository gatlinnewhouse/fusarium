use core::{marker::PhantomData, ops};

/// Framebuffer
pub mod framebuffer;
/// GPIO pins
pub mod gpio;
/// Mailbox
pub mod mailbox;
/// PL011 UART
pub mod uart;
/// Video driver
pub mod video;

/// Memory mapped IO dereference wrapper
pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create a new MMIO deref wrapper
    pub const fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;
    /// Return the wrapped type from the wrapper
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}
