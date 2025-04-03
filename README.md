# About
`easy-rust` contains easy manual of Rust language.

<br>

# Benefits of Rust
The `Rust` language **gets rid** of *some issues* that `C` and `C++` languages have.<br>

The **UB** term is used to describe the behaviour of a program which is **completely unpredictable**.<br> 
In many languages accessing an index out of bound lead to **UB**, which can lead to **security vulnerabilities** and **bugs that are hard to track**.<br>
**Rust prevents you from accessing invalid memory locations**, that's why Rust is considered as a **safe** language.<br>

<br>

## Compile time memory safety
- no uninitialized variables;
- no **double-free**: because of **move semantics** and **scope based** resource management;
- no **use after free** (aka **dangling pointers**): because of *reference* **can’t outlive** its *owner*;
- no NULL pointers;
- no forgotten locked mutexes;
- no data races between threads;
- no iterator invalidation (iterator invalidation is a mutating collection during iteration);
- access to element of **array** by index is **defined** - when the value of index is **known** at compile time and is **out of** array - it is a **compile error**;

<br>

**Dangling pointer** is pointer that points to a memory location that has been **deallocated**.<br>

<br>

### Examples
**Compile error**:
```rust
fn main() {
    let arr = [0; 2];
    println!("{}", arr[5]);
}
```

<br>

**Compile error**:
```rust
fn main() {
    let arr = [0; 2];
    let idx = 7;
    println!("{}", arr[idx]);
}
```

<br>

## No UB at runtime
- **access beyond** ыуйгутсу (array/slice/vector) **boundary** is **defined** - **panic**;
- **integer overflow** is defined - **panic** or **wrap around**;

<br>

### Examples
Successfully compiled but panics at runtime:
```rust
fn main() {
    let vec = vec![0; 7];
    let arr = [0; 2];
    println!("{}", arr[vec.len()]);
}
```

<br>

Successfully compiled but panics at runtime:
```rust
fn main() {
    let vec = vec![0; 5];
    println!("{}", vec[7]);
}
```

<br>

Successfully compiled but panics at runtime:
```rust
fn main() {
    let vec = vec![0; 5];
    let arr = [0; 5];

    let s = &arr[0..10];
}
```

<br>

Successfully compiled but panics at runtime:
```rust
fn main() {
    let vec = vec![0; 5];
    let arr = [0; 5];

    let s = &arr[0..3];
    println!("{}", s[5]);
}
```

<br>

## Modern features
- Enums and pattern matching
- Generics
- Zero cost abstractions
- Built-in dependency manager
- Built-in support for testing
- Great compiler errors
