extern crate alloc;
use alloc::boxed::Box;
#[cfg(target_arch = "arm")]
use core::sync::atomic::{AtomicU32, Ordering};
#[cfg(target_arch = "x86_64")]
use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "exec-mine")]
pub mod executor;
pub mod keyboard;
#[cfg(feature = "exec-simple")]
pub mod simple_executor;

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

#[cfg(target_arch = "arm")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u32);

impl TaskId {
    /// New unique ID for a task to prevent CPU hogging
    fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        #[cfg(target_arch = "arm")]
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new Task that is pinned to a place in memory
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    /// Poll a task for its result
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
