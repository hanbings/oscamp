use core::{alloc::Layout, mem::size_of, ptr::NonNull};

use crate::linked_list::LinkedList;

pub struct Chaos {
    pub free: LinkedList,
    pub head: LinkedList,
    pub head_size: usize,
    pub tail: LinkedList,
    pub tail_size: usize,
    pub allocated: usize,
    pub total: usize,
}

impl Chaos {
    pub const fn new() -> Self {
        Chaos {
            free: LinkedList::new(),
            head: LinkedList::new(),
            head_size: 0,
            tail: LinkedList::new(),
            tail_size: 0,
            allocated: 0,
            total: 0,
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end &= !size_of::<usize>() + 1;
        assert!(start <= end);

        let mut total = self.total;
        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            if self.head_size >= self.tail_size {
                unsafe { self.head.push(current_start as *mut usize) };
                self.head_size += size_of::<usize>();
            } else {
                unsafe { self.tail.push(current_start as *mut usize) };
                self.tail_size += size_of::<usize>();
            }

            current_start += size_of::<usize>();
            total += size_of::<usize>();
        }

        self.total += total;
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = layout.size();
        let align = layout.align();
        let alloc_size = if size == 0 { 1 } else { size };
        let alloc_align = if align == 0 { 1 } else { align };

        if alloc_size > self.available_bytes() {
            return Err(());
        }

        if self.head_size >= self.tail_size {
            self.head_size -= size_of::<usize>();
            self.head.pop()
        } else {
            self.tail_size -= size_of::<usize>();
            self.tail.pop()
        }
        .ok_or(())
        .and_then(|addr| {
            self.allocated += alloc_size;
            Ok(NonNull::new(addr as *mut u8).unwrap())
        })
    }

    pub fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        let alloc_size = if size == 0 { 1 } else { size };
        let alloc_align = if align == 0 { 1 } else { align };

        self.allocated -= alloc_size;
        unsafe { self.free.push(pos.as_ptr() as *mut usize) };
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