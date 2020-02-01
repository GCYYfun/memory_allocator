#![feature(const_fn)]
#![feature(allocator_api)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "use_spin")]
extern crate spin;

extern crate alloc;

use alloc::alloc::{Alloc, AllocErr, Layout};
use core::alloc::{GlobalAlloc};
use core::ptr::NonNull;
#[cfg(feature = "use_spin")]
use core::ops::Deref;
#[cfg(feature = "use_spin")]
use spin::Mutex;

#[cfg(test)]
mod test;

pub mod block;

use block::Block;
#[derive(Debug, Clone, Copy)]
pub enum AllocType{
    First,
    Best,
    Worst,
}

static mut ALLOC_TYPE:AllocType = AllocType::First;

#[derive(Debug, Clone, Copy)]
pub struct Heap {
    start:usize,
    size:usize,
    book:[Option<Block>;32],
}

impl Heap {
    pub const fn empty() -> Heap {
        Heap {
            start:0,
            size:0,
            book:[None;32],
        }
    }

    pub unsafe fn init(&mut self,allocator_type:AllocType,heap_start:usize,heap_size:usize) {
        ALLOC_TYPE = allocator_type;
        self.start = heap_start;
        self.size = heap_size;
        self.book[0] = Some(Block::new(heap_start,heap_size));
    }

    pub fn allocate(&mut self,layout:Layout)->Result<NonNull<u8>,AllocErr> {
        alloc(self,layout)
    }

    pub fn deallocate(&mut self,ptr: NonNull<u8>, layout: Layout) {
        dealloc(self,ptr,layout);
    }
}

pub fn alloc(heap:&mut Heap,layout:Layout)->Result<NonNull<u8>, AllocErr>{

    let required_size = layout.size();
    let required_align = layout.align();

    let align_size = align_up(required_size,required_align);

    for i in 0..heap.book.len() {
        if heap.book[i].is_some() && heap.book[i].unwrap().size > required_size {

                let book_size = heap.book[i].unwrap().size;
                let book_start = heap.book[i].unwrap().start;
                let align_book_start = align_up(book_start, required_align);
                let new_size = book_size - align_size;
                let new_start = align_book_start + align_size;
                heap.book[i] = Some(Block::new(new_start,new_size));
                let result = NonNull::new(align_book_start as *mut u8);
                return Ok(result.unwrap());
        }
    }

    return Err(AllocErr {});
}

fn dealloc(heap:&mut Heap,ptr: NonNull<u8>,layout:Layout) {
    let required_size = layout.size();

    let len = heap.book.len();
    for i in 0..heap.book.len(){
        if heap.book[i].is_none() {
                let new_start = ptr.as_ptr() as usize;
                heap.book[i] = Some(Block::new(new_start,required_size));
                break;
        }
    }
    unsafe{
        match ALLOC_TYPE {
            AllocType::First => quick_sort_for_first(&mut heap.book,0,len-1),
            AllocType::Best => quick_sort_for_best(&mut heap.book,0,len-1),
            AllocType::Worst => quick_sort_for_worst(&mut heap.book,0,len-1),
        }
    }
}

pub fn quick_sort_for_first(nums: &mut [Option<Block>;32], left: usize, right: usize) {
    if left >= right {
        return;
    }
 
    let mut l = left;
    let mut r = right;
    while l < r && !nums[l].is_none()&&!nums[r].is_none(){
        while l < r && nums[r].unwrap().start >= nums[left].unwrap().start {
            r -= 1;
        }
        while l < r && nums[l].unwrap().start <= nums[left].unwrap().start {
            l += 1;
        }
        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort_for_first(nums, left, l - 1);
    }
 
    quick_sort_for_first(nums, r + 1, right);
}

pub fn quick_sort_for_best(nums: &mut [Option<Block>;32], left: usize, right: usize) {
    if left >= right {
        return;
    }
 
    let mut l = left;
    let mut r = right;
    while l < r && !nums[l].is_none()&&!nums[r].is_none(){
        while l < r && nums[r].unwrap().size >= nums[left].unwrap().size {
            r -= 1;
        }
        while l < r && nums[l].unwrap().size <= nums[left].unwrap().size {
            l += 1;
        }
        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort_for_best(nums, left, l - 1);
    }
 
    quick_sort_for_best(nums, r + 1, right);
}
pub fn quick_sort_for_worst(nums: &mut [Option<Block>;32], left: usize, right: usize) {
    if left >= right {
        return;
    }
 
    let mut l = left;
    let mut r = right;
    while l < r && !nums[l].is_none()&&!nums[r].is_none(){
        while l < r && nums[r].unwrap().size <= nums[left].unwrap().size {
            r -= 1;
        }
        while l < r && nums[l].unwrap().size >= nums[left].unwrap().size {
            l += 1;
        }
        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort_for_worst(nums, left, l - 1);
    }
 
    quick_sort_for_worst(nums, r + 1, right);
}

unsafe impl Alloc for Heap {
    unsafe fn alloc(&mut self,layout:Layout)->Result<NonNull<u8>,AllocErr>{
        self.allocate(layout)
    }

    unsafe fn dealloc(&mut self,ptr:NonNull<u8>,layout:Layout) {
        self.deallocate(ptr, layout )
    }
}

#[cfg(feature = "use_spin")]  // ?
pub struct LockedHeap(Mutex<Heap>);

#[cfg(feature = "use_spin")] // ?
impl LockedHeap {
    pub const fn empty() -> LockedHeap{
        LockedHeap(Mutex::new(Heap::empty()))
    }
}

#[cfg(feature = "use_spin")]
impl Deref for LockedHeap {

    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

#[cfg(feature = "use_spin")]
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self,layout:Layout) -> *mut u8 {
        self.0
            .lock()
            .allocate(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}

pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}