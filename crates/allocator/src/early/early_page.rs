use crate::{AllocError, AllocResult, BaseAllocator, PageAllocator};

pub struct EarlyPageAllocator<const PAGE_SIZE: usize> {
    base: usize,  //用来判断与byte的重叠
    pos: usize,
    total_pages: usize,
    used_pages: usize,
    end: usize,
}

impl<const PAGE_SIZE: usize> EarlyPageAllocator<PAGE_SIZE> {
    /// Creates a new empty `EarlyPageAllocator`.
    pub const fn new() -> Self {
        Self {
            base: 0,
            pos: 0,
            total_pages: 0,
            used_pages: 0,
            end: 0,
        }
    }

}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyPageAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        assert!(PAGE_SIZE.is_power_of_two());
        self.end = crate::align_down(start + size, PAGE_SIZE);
        let start = crate::align_up(start, PAGE_SIZE);        
        self.base = start;
        self.total_pages = (self.end - start) / PAGE_SIZE;
        self.pos = self.end;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyPageAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if self.pos < self.base + num_pages * PAGE_SIZE {
            return Err(AllocError::NoMemory);
        }
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let align_log2 = align_pow2.trailing_zeros() as usize;
        match num_pages.cmp(&1) {
            core::cmp::Ordering::Equal =>  Some(self.pos - PAGE_SIZE),
            core::cmp::Ordering::Greater => Some(self.pos - PAGE_SIZE * num_pages),
            _ => return Err(AllocError::InvalidParam),
        }
        .ok_or(AllocError::NoMemory)
        .inspect(|_| {
            self.used_pages += num_pages;
            self.pos -= num_pages * PAGE_SIZE;
        })
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        self.used_pages -= num_pages;
        if self.used_pages == 0 {
            self.pos = self.end;    
        }
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn used_pages(&self) -> usize {
        self.used_pages
    }

    fn available_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}
