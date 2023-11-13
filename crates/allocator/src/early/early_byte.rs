
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct EarlyByteAllocator {
    start: usize,
    pos: usize,
    total_bytes: usize,
    used_bytes: usize,
}

impl EarlyByteAllocator {
    /// Creates a new empty [`EarlyByteAllocator`].
    pub const fn new() -> Self {
        Self {
            start: 0,
            pos: 0,
            total_bytes: 0,
            used_bytes: 0,
        }
    }

}

impl BaseAllocator for EarlyByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.pos =  start;
        self.total_bytes = size;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl ByteAllocator for EarlyByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        match NonNull :: new(self.pos as *mut u8) {
            Some(ptr) => {
                let size = layout.size();
                self.used_bytes += size;
                if self.available_bytes() < size {
                    return Err(AllocError::NoMemory);
                }
                self.pos += size;
                Ok(ptr)
            }
            None => {
                Err(AllocError::NoMemory)
            }
        }
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        self.used_bytes -= size;
        if self.used_bytes == 0 {
            self.pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }
}
