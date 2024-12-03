//! Allocator algorithm in lab.
#![no_std]
#![allow(unused_variables)]

mod buddy;
mod linked_list;
mod chaos;

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use chaos::Chaos;
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct LabByteAllocator {
    inner: Chaos,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Chaos::new(),
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unsafe { self.inner.init(start, size) };
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.inner.add_to_heap(start, start + size);
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.inner.alloc(layout).map_err(|_| AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.inner.dealloc(pos, layout)
    }

    fn total_bytes(&self) -> usize {
        self.inner.total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.inner.used_bytes()
    }

    fn available_bytes(&self) -> usize {
        self.inner.available_bytes()
    }
}