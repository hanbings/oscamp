//! Allocator algorithm in lab.
#![no_std]
#![allow(unused_variables)]

mod buddy;
mod linked_list;

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use buddy::Heap;
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct LabByteAllocator {
    inner: Heap<32>,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Heap::<32>::new(),
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unsafe { self.inner.init(start, size) };
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe { self.inner.add_to_heap(start, start + size) };
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
        self.inner.stats_total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.inner.stats_alloc_actual()
    }

    fn available_bytes(&self) -> usize {
        self.inner.stats_total_bytes() - self.inner.stats_alloc_actual()
    }
}

pub(crate) fn prev_power_of_two(num: usize) -> usize {
    1 << (usize::BITS as usize - num.leading_zeros() as usize - 1)
}
