# memory_allocator

um....no no no、no value.  
Learn from linked-list-allocator

## Usage

Create a static allocator in your root module:

```rust
use memory_allocator::{LockedHeap,AllocType};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
```

Before using this allocator, you need to init it:

```rust
pub fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        ALLOCATOR.lock().init(AllocType::First,HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
  //or  ALLOCATOR.lock().init(AllocType::Best,HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
  //or  ALLOCATOR.lock().init(AllocType::Worst,HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
```
