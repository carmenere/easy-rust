# `std::alloc::System`
**Defenition**:
```rust
pub struct System;
```

The `std::alloc::System` is a *default* **memory allocator** *provided* by the operating system. This is based on `malloc` on Unix platforms and `HeapAlloc` on Windows.<br>

This type implements the `GlobalAlloc` trait.<br>

In a given program, the **standard library** has one **global memory allocator** that is used for example by `Box<T>` and `Vec<T>`.<br>
Currently the *default* **global allocator** is **unspecified**. Libraries, however, like `cdylibs` and `staticlibs` are **guaranteed** to use the `System` by default.<br>

<br>

## The `#[global_allocator]` attribute
This attribute allows configuring the **choice** of **global allocator**.<br>
You can use this to implement a completely **custom global allocator** to route all default allocation requests to a custom object.<br>
This attribute is used on a `static` item whose type implements the `GlobalAlloc` trait.<br>

```rust
use std::alloc::{GlobalAlloc, System, Layout};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    // This `Vec` will allocate memory through `GLOBAL` above
    let mut v = Vec::new();
    v.push(1);
}
```
