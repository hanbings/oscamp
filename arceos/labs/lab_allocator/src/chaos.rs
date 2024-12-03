use core::{alloc::Layout, ptr::NonNull};

pub struct Chaos {
    pub vec_reserve: (usize, usize, usize),

    pub start: usize,
    pub end: usize,
    pub head: usize,
    pub tail: usize,
    pub is_even: bool,

    pub allocated: usize,
    pub total: usize,
}

impl Chaos {
    pub const fn new() -> Self {
        Chaos {
            vec_reserve: (0, 0, 0),
            start: 0,
            end: 0,
            head: 0,
            tail: 0,
            is_even: false,
            allocated: 0,
            total: 0,
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.vec_reserve = (start, start + 96, start + 96 + 192);
        
        self.start = start + 96 + 192 + 384;
        self.head = start + 96 + 192 + 384;

        self.end = start + size;
        self.tail = start + size;

        self.allocated = 96 + 192 + 384;
        self.total = self.end - self.start;
    }

    pub fn add_to_heap(&mut self, start: usize, end: usize) {
        self.end = end;
        self.tail = end;

        self.total += end - start;
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let vec_reserve_ptr = match layout.size() {
            96 => Some(self.vec_reserve.0 as *const u8),
            192 => Some(self.vec_reserve.1 as *const u8),
            384 => Some(self.vec_reserve.2 as *const u8),
            _ => None,
        };

        if vec_reserve_ptr.is_some() {
            return Ok(NonNull::new(vec_reserve_ptr.unwrap() as *mut u8).unwrap());
        }

        // check if memory is overflow
        if self.tail - layout.size() < self.head {
            log::warn!("head, tail: 0x{:x}, 0x{:x}", self.head, self.tail);

            return Err(());
        }

         log::warn!("add_memory: head=0x{:x}, tail=0x{:x}, size=0x{:x}", self.start, self.tail, layout.size());

        if self.is_even {
            
        }

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