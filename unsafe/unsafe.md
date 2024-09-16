# Table of contents
- [Table of contents](#table-of-contents)
- [`unsafe` keyword](#unsafe-keyword)
- [Dereference raw pointer](#dereference-raw-pointer)
- [Call unsafe function or method](#call-unsafe-function-or-method)
- [FFI](#ffi)
- [Take 2 mut pointers to different parts of the same vector](#take-2-mut-pointers-to-different-parts-of-the-same-vector)
  - [Example implementation of split\_at\_mut:](#example-implementation-of-split_at_mut)
- [r/w access to static variables](#rw-access-to-static-variables)
- [Implement an unsafe trait](#implement-an-unsafe-trait)

<br>

# `unsafe` keyword
The `unsafe` block **disables some of the compiler's safety checks**. The Rust language has a **set of rules** that need to be followed to avoid UB. It is possible to **bypass** some of *these rules* **inside** `unsafe` block.<br>
**Unsafe** doesn't mean that the code is incorrect or never safe to use, but rather that the **compiler doesn't validate** for you that the code is safe.<br>
If the code **violates** *these rules*, it is called **unsound**.<br>

When calling any `unsafe` function, read its documentation carefully and make sure you fully understand its **safety requirements**: the assumptions you need to uphold, as the caller, to avoid UB.<br>

The `unsafe` keyword gives 5 abilities:
1. **Dereference raw pointer**.
2. **Call unsafe function or method**.
3. **Modify a mutable static variable**.
4. **Implement an unsafe trait**.
5. **Access fields of unions**.

<br>

# Dereference raw pointer
Raw pointer can only be **dereferenced** in `unsafe` **block**:
```Rust
fn main() {
    let mut num = 10u64;

    let ptr1: *const u64 = &num as *const u64;
    let ptr2: *mut u64 = &mut num as *mut u64;

    unsafe {
        println!("ptr1 => {}", *ptr1);
        println!("ptr1 => {}", *ptr2);
    }
}
```

<br>

# Call unsafe function or method
```Rust
unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
}
```

<br>

The **body** of **unsafe function** is also **unsafe block**.<br>

<br>

# FFI
```Rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main () {
    let v = -12;
    unsafe {
        println!("Absolute value of '{}' is '{}'.", v, abs(v));
    }
}
```

<br>

# Take 2 mut pointers to different parts of the same vector
The borrow checker doesn't allow to have more then one mutable slices to the same collection.<br>
But we can add some checks into out code to guarantee its correctness.<br>

```Rust
fn main() {
    let mut v = vec![0,1,2,3,4,5];
    let r: (&mut [i32], &mut [i32]) = (&mut v[..]).split_at_mut(3);
    dbg!(&r.0);
    dbg!(&r.1);
    assert_eq!(r.0, &mut [0, 1,2]);
    assert_eq!(r.1, &mut [3,4,5]);
}
```

<br>

## Example implementation of split_at_mut:
`ptr.add(count)` calculates the **offset** from a pointer in units of `T`.<br>
For example, if `count==3` then offset will be `3 * size_of::<T>()` bytes.<br>
```Rust
core::ptr::mut_ptr
pub const unsafe fn add(self, count: usize) -> Self
where
    T: Sized,
```

<br>

```Rust
use std::slice;

fn main() {
    let mut v: Vec<u64> = vec![0u64,1,2,3,4,5];
    let r: (&mut [u64], &mut [u64]) = (&mut v[..]).split_at_mut(3);
    dbg!(&r.0);
    dbg!(&r.1);
    assert_eq!(r.0, &mut [0,1,2]);
    assert_eq!(r.1, &mut [3,4,5]);
    let r: (&mut [u64], &mut [u64]) = split_at_mut(&mut v[..], 3);
    dbg!(&r.0);
    dbg!(&r.1);
    assert_eq!(r.0, &mut [0,1,2]);
    assert_eq!(r.1, &mut [3,4,5]);
}

fn split_at_mut(slice: &mut[u64], mid: usize) -> (&mut [u64], &mut [u64]) {
    let len: usize = slice.len();
    let ptr: *mut u64 = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }
}
```

<br>

# r/w access to static variables
```Rust
static mut COUNTER: u64 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

fn main () {
    // access to modify static variable
    unsafe {
        increment();
    }

    // access to read static variable
    unsafe {
        println!("Counter is {}.", COUNTER);
    }
}
```

<br>

# Implement an unsafe trait
A `trait` is **unsafe** if at least **one** of it method is **unsafe**.<br>

```Rust
unsafe trait Foo {}

unsafe impl Foo for u64 {

}

fn main() {

}
```