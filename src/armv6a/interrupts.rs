use core::{
    arch::asm,
    sync::atomic::{compiler_fence, Ordering},
};

/// Check if we are in an interrupt
///
/// # Returns
/// * True if we are in an interrupt
/// * False otherwise
#[inline]
pub fn are_enabled() -> bool {
    let mut cpsr: u32; // Current processor execution state
    unsafe { asm!("mrs {}, cpsr", out(reg) cpsr, options(nomem, nostack, preserves_flags)) };
    cpsr & 1 << 7 == 0
}

/// Disable interrupts
///
/// Thanks rpi-devenv, although you have a misnamed function
pub fn disable() {
    // Safety: The instruction is defined in the ARMv6 manual. See section A4.1.16.
    unsafe {
        asm!("cpsid i", options(nomem, nostack));
    }
    compiler_fence(Ordering::SeqCst);
}

/// Enable interrupts
///
/// Thanks rpi-devenv, although you have a misnamed function
pub fn enable() {
    // Safety: The instruction is defined in the ARMv6 manual. See section A4.1.16.
    unsafe {
        asm!("cpsie i", options(nomem, nostack));
    }
    compiler_fence(Ordering::SeqCst);
}

/// Disable interrupts for a small function
///
/// Thanks x86_64, I basically wrote the same function for ARM
pub(crate) fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let intpt_flag = are_enabled();

    if intpt_flag {
        disable();
    }

    // do 'f' while interrupts are disabled
    let ret = f();

    if intpt_flag {
        enable();
    }

    ret
}
