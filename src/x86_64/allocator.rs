#[cfg(feature = "alloc-bump")]
use bump::BumpAllocator;
#[cfg(feature = "alloc-fixed-block")]
use fixed_size_block::FixedSizeBlockAllocator;
#[cfg(feature = "alloc-my-free-list")]
use linked_list::LinkedListAllocator;
#[cfg(feature = "alloc-linked-list")]
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

#[cfg(feature = "alloc-bump")]
#[path = "../allocator/bump.rs"]
pub mod bump;
#[cfg(feature = "alloc-fixed-block")]
#[path = "../allocator/fixed_size_block.rs"]
pub mod fixed_size_block;
#[cfg(feature = "alloc-my-free-list")]
#[path = "../allocator/linked_list.rs"]
pub mod linked_list;

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

#[cfg(feature = "alloc-linked-list")]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(feature = "alloc-bump")]
#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[cfg(feature = "alloc-my-free-list")]
#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

#[cfg(feature = "alloc-fixed-block")]
#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

#[cfg(any(feature = "alloc-linked-list", feature = "alloc-fixed-block"))]
pub const HEAP_START: *mut u8 = 0x_4444_4444_0000 as *mut u8;
#[cfg(all(not(feature = "alloc-linked-list"), not(feature = "alloc-fixed-block")))]
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // Create a page range
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Map all heap pages to physical frames
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

#[cfg(any(feature = "alloc-bump", feature = "alloc-my-free-list"))]
/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
