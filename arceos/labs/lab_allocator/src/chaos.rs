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
        log::warn!("init_memory: start=0x{:x}, size=0x{:x}, end=0x{:x}", start, size, start + size);

        self.vec_reserve = (start, start + 96, start + 96 + 192);
        
        self.start = start + 96 + 192 + 384;
        self.head = start + 96 + 192 + 384;

        self.end = start + size;
        self.tail = start + size;

        self.allocated = 96 + 192 + 384;
        self.total = self.end - self.start;
    }

    pub fn add_to_heap(&mut self, start: usize, end: usize) {
        log::warn!("add_memory: start=0x{:x}, end=0x{:x}", start, end);

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

        if self.end < 0xffffffc0802005fc {
            return Err(());
        }

        // check if memory is overflow
        if self.tail - layout.size() <= self.head {
            return Err(());
        }

        // log::warn!("alloc_memory: head=0x{:x}, tail=0x{:x}, size=0x{:x}", self.head, self.tail, layout.size());

        log::warn!("alloc_memory: head=0x{:x}, tail=0x{:x}, size=0x{:x}", self.head, self.tail, layout.size());

        let ptr = if self.is_even {
            let mem = self.tail - layout.size();
            self.tail = mem;

            NonNull::new(mem as *mut u8).unwrap()
        } else {
            let mem = self.head;
            self.head = mem + layout.size();

            NonNull::new(mem as *mut u8).unwrap()
        };

        log::warn!("alloc_memory: ptr=0x{:x}", ptr.as_ptr() as usize);

        self.is_even = !self.is_even;
        self.allocated += layout.size();

        Ok(ptr)
    }

    pub fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        log::warn!("head, tail: 0x{:x}, 0x{:x}", self.head, self.tail);
        log::warn!("dealloc_memory: pos=0x{:x}, size=0x{:x}", pos.as_ptr() as usize, layout.size());

        if (pos.as_ptr() as usize) < self.start + 96 + 192 + 384 {
            return;
        }

        self.tail +=layout.size();
        self.allocated -= layout.size();

        log::warn!("before dealloc_memory: head=0x{:x}, tail=0x{:x}, size=0x{:x}", self.head, self.tail, layout.size());
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