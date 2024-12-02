use core::{alloc::Layout, ptr::NonNull};

pub struct Chaos {
    pub vec_reserve_size: usize,
    pub vec_reserve_start: usize,
    pub start: usize,
    pub end: usize,
    pub head: usize,
    pub tail: usize,

    pub allocated: usize,
    pub total: usize,
}

impl Chaos {
    pub const fn new() -> Self {
        Chaos {
            vec_reserve_size: 0,
            vec_reserve_start: 0,
            start: 0,
            end: 0,
            head: 0,
            tail: 0,
            allocated: 0,
            total: 0,
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn add_to_heap(&mut self, start: usize, end: usize) {
        
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        Err(())
    }

    pub fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        
    }

    pub fn total_bytes(&self) -> usize {
        self.total
    }

    pub fn used_bytes(&self) -> usize {
        self.allocated
    }

    pub fn available_bytes(&self) -> usize {
        self.total - self.allocated
    }
}