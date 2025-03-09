use embedded_alloc::TlsfHeap;

#[global_allocator]
static ALLOCATOR: TlsfHeap = TlsfHeap::empty();

pub fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    unsafe { ALLOCATOR.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
}
