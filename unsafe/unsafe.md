# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Unsafe Rust](#unsafe-rust)
* [`unsafe` keyword](#unsafe-keyword)
* [Dereference raw pointer](#dereference-raw-pointer)
* [Call unsafe function or method](#call-unsafe-function-or-method)
* [FFI](#ffi)
* [Take 2 mut pointers to different parts of the same vector](#take-2-mut-pointers-to-different-parts-of-the-same-vector)
  * [Example implementation of split_at_mut:](#example-implementation-of-split_at_mut)
* [r/w access to static variables](#rw-access-to-static-variables)
* [Implement an unsafe trait](#implement-an-unsafe-trait)
* [Bypassing borrow checker rules](#bypassing-borrow-checker-rules)
  * [Example 2: splitting borrows](#example-2-splitting-borrows)
  * [Example 2: aliasing mutable reference](#example-2-aliasing-mutable-reference)
<!-- TOC -->

<br>

# Unsafe Rust
Rust consists of 2 parts:
- **Safe Rust**;
- **Unsafe Rust**;

The main distinction between **Safe Rust** and **Unsafe rust** is **Safe Rust can't cause to UB**. It is also known as **soundness property**.<br>

**Sound vs. Unsound**:
- the code is **sound** if it **can't** cause to UB;
- the code is **unsound** if it **can** cause to UB;

<br>

For example, **all FFI functions are unsafe** because the other langs can do **arbitrary operations** that the **Rust can't check**.<br>

<br>

There is special lint `unsafe_code` to statically guarantee that **only Safe Rust is used**:
```rust
#![forbid(unsafe_code)]
```

<br>

# `unsafe` keyword
The separation between Safe Rust and Unsafe Rust is **controlled** by the `unsafe` keyword.<br>
The Rust language has a **set of rules** that need to be followed to **avoid UB**.<br>
The `unsafe` block **disables some of the compiler's safety checks**.<br>
**Unsafe** doesn't mean that the code is incorrect or never safe to use, but rather that the **compiler doesn't validate** for you that the code is safe.<br>
If the code **violates** *these rules*, it is called **unsound**.<br>

When calling any `unsafe` function, read its documentation carefully and make sure you fully understand its **safety requirements**: the assumptions you need to uphold, as the caller, to avoid UB.<br>

<br>

The `unsafe` keyword (**Unsafe Rust**) allows:
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

<br>

# Bypassing borrow checker rules
## Example 2: splitting borrows
Using _Unsafe Rust_ we can **bypass** mutual exclusion to borrow **disjoint fields** of a struct or **disjoint slices** of collection simultaneously.<br>

For example, there is `split_at_mut` method for `Vec` which divides one mutable slice into two at `mid`:
```rust
split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
```

<br>

## Example 2: aliasing mutable reference
Consider example:
```rust
fn example(x: &mut i32, y: &mut i32) -> i32 {
  *x = 20;
  *y = 10;
  *x
}
```

In **Safe Rust** it is **not possible** to have **2 mutable reference** to the **same value**.<br>
So, in Safe Rust `x` and `y` cannot alias, i.e. Safe Rust guarantees that they are point to different addresses.<br>
So, in Safe Rust the code above returns `20`.<br>

But, in **Unsafe Rust** the code above can return `10`. Why? Because we can use **raw pointers** in **Unsafe Rust** and they are **not** tracked by the type system.<br>

<br>

Consider example:
```rust
fn main() {
    let mut value = 10;
    let raw_ptr = &mut value as *mut i32;
    let result = unsafe { example(&mut *raw_ptr, &mut *raw_ptr) };
    println!("result is {}", result);
}

fn example(x: &mut i32, y: &mut i32) -> i32 {
    *x = 20;
    *y = 10;
    *x
}
```

<br>

In the code above we could pass **2 mutable references** to the **same value** and type system **didn't** stop us.<br>
And as a result we got `10`.<br>

<br>

However, **dereferencing raw pointers is only permitted** in `unsafe` block. The `unsafe` block serve as **syntactic marker** for potentially **UB**.<br>
