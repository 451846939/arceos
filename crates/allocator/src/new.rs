use crate::{AllocResult, BaseAllocator};
use linked_list_allocator::Heap;
pub struct LinkedListAllocator{
    inner:Heap,
}

impl BaseAllocator for LinkedListAllocator{
    fn init(&mut self, start: usize, size: usize) {
        unsafe { Heap::empty().init(start as *mut u8 , size); }
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        Ok(())
    }
}