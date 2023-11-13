mod early_page;
mod early_byte;



use self::early_byte::EarlyByteAllocator;
use self::early_page::EarlyPageAllocator;

use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
use spinlock::SpinNoIrq;

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    byte_alloc: SpinNoIrq<EarlyByteAllocator>,
    page_alloc: SpinNoIrq<EarlyPageAllocator<PAGE_SIZE>>,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty `EarlyAllocator`.
    pub const fn new() -> Self {
        Self {
            byte_alloc: SpinNoIrq::new(EarlyByteAllocator::new()),
            page_alloc: SpinNoIrq::new(EarlyPageAllocator::new())
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.page_alloc.lock().init(start, size);
        self.byte_alloc.lock().init(start, size);
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        self.page_alloc.lock().alloc_pages(num_pages, align_pow2)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        // TODO: not decrease `used_pages` if deallocation failed
        self.page_alloc.lock().dealloc_pages(pos, num_pages);
    }

    fn total_pages(&self) -> usize {
        self.page_alloc.lock().total_pages()
    }

    fn used_pages(&self) -> usize {
        self.page_alloc.lock().used_pages()
    }

    fn available_pages(&self) -> usize {
        self.page_alloc.lock().available_pages()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.byte_alloc.lock().alloc(layout)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.byte_alloc.lock().dealloc(pos, layout);
    }

    fn total_bytes(&self) -> usize {
        self.byte_alloc.lock().total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.byte_alloc.lock().used_bytes()
    }

    fn available_bytes(&self) -> usize {
        self.byte_alloc.lock().available_bytes()
    }
}