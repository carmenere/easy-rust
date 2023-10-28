# Raw pointers
- `*const T` **immutable raw pointer**, `&T` can be casted to `*const T`;
- `*mut T` **mutable raw pointer**, `&mut T` can be casted to `*mut T`;

<br>

**Raw pointers** are useful to build up safe abstractions that borrow checker doesn't anderstand or for FFI with C code.<br>
**Raw pointers** used to **bypass borrowing rules**. For example, **raw pointers** allow to have **multiple mutable raw pointers** to the same value.<br>
They have no **lifetime** or **ownership** attached to them.<br>
Also they **aren't guaranted** to point to valid memory.<br>
They **don't implement automatic dealocation**.<br>
Rust **allows** to create *raw pointers*, but it **doesn't allow** to **dereference** *raw pointers*.

<br>

# Example
## Declare raw pointers
```Rust
fn main() {
    let mut num = 10u64;

    let ptr1: *const u64 = &num as *const u64;
    let ptr2: *mut u64 = &mut num as *mut u64;
}
```

<br>

## Raw pointer to arbitrary memory
We can create **raw pointer** that will point to arbitrary memory:
```Rust
fn main() {
    let mut address: usize = 0x10abcdefusize;
    let ptr: *const u64 = address as *const u64;
}
```

<br>

Trying to use arbitrary memory is **UB**.