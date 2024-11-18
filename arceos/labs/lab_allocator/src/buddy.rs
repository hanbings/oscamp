use crate::prev_power_of_two;
use core::alloc::Layout;
use core::cmp::{max, min};
use core::fmt;
use core::mem::size_of;
use core::ptr::NonNull;

use crate::linked_list::LinkedList;

pub struct Heap<const ORDER: usize> {
    pub free_list: [LinkedList; ORDER],
    pub allocated: usize,
    pub total: usize,
}

impl<const ORDER: usize> Heap<ORDER> {
    pub const fn new() -> Self {
        Heap {
            free_list: [LinkedList::new(); ORDER],
            allocated: 0,
            total: 0,
        }
    }

    pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end &= !size_of::<usize>() + 1;
        assert!(start <= end);

        let mut total = 0;
        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            let mut size = min(lowbit, prev_power_of_two(end - current_start));
            let mut order = size.trailing_zeros() as usize;
            if order > ORDER - 1 {
                order = ORDER - 1;
                size = 1 << order;
            }
            total += size;

            self.free_list[order].push(current_start as *mut usize);
            current_start += size;
        }

        self.total += total;
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;
        for i in class..self.free_list.len() {
            if !self.free_list[i].is_empty() {
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_list[j].pop() {
                        unsafe {
                            self.free_list[j - 1]
                                .push((block as usize + (1 << (j - 1))) as *mut usize);
                            self.free_list[j - 1].push(block);
                        }
                    } else {
                        return Err(());
                    }
                }

                let result = NonNull::new(
                    self.free_list[class]
                        .pop()
                        .expect("current block should have free space now")
                        as *mut u8,
                );
                if let Some(result) = result {
                    self.allocated += size;
                    return Ok(result);
                } else {
                    return Err(());
                }
            }
        }
        Err(())
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;

        unsafe {
            self.free_list[class].push(ptr.as_ptr() as *mut usize);

            let mut current_ptr = ptr.as_ptr() as usize;
            let mut current_class = class;

            while current_class < self.free_list.len() - 1 {
                let buddy = current_ptr ^ (1 << current_class);
                let mut flag = false;
                for block in self.free_list[current_class].iter_mut() {
                    if block.value() as usize == buddy {
                        block.pop();
                        flag = true;
                        break;
                    }
                }

                if flag {
                    self.free_list[current_class].pop();
                    current_ptr = min(current_ptr, buddy);
                    current_class += 1;
                    self.free_list[current_class].push(current_ptr as *mut usize);
                } else {
                    break;
                }
            }
        }

        self.allocated -= size;
    }

    pub fn stats_alloc_actual(&self) -> usize {
        self.allocated
    }

    pub fn stats_total_bytes(&self) -> usize {
        self.total
    }
}

impl<const ORDER: usize> fmt::Debug for Heap<ORDER> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Heap")
            .field("allocated", &self.allocated)
            .field("total", &self.total)
            .finish()
    }
}
