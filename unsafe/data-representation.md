# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Data representation](#data-representation)
* [`repr`](#repr)
  * [`#[repr(C)]`](#reprc)
  * [`#[repr(transparent)]`](#reprtransparent)
  * [`repr(u*)` and `repr(i*)`](#repru-and-repri)
  * [`repr(packed)` and `repr(packed(n))`](#reprpacked-and-reprpackedn)
  * [`repr(align(n))`](#repralignn)
<!-- TOC -->

<br>

# Data representation
By default, **composite types** (_structs_, _enums_, _arrays_, _tuples_, _unions_) have an **alignment equal to align of their maximal field**.<br>

Rust will **insert padding where necessary** to ensure that all fields are **properly aligned** and the **overall size** of composite type is a **multiple of its alignment**.<br>

Consider struct:
```rust
struct A {
    a: u8,
    b: u32,
    c: u16,
}
```

It will be **32-bit aligned** and the whole struct will therefore have a size that is a multiple of 32-bits.<br>

**It may become**:
```rust
struct A {
    a: u8,
    _pad1: [u8; 3], // to align `b`
    b: u32,
    c: u16,
    _pad2: [u8; 2], // to make overall size multiple of 4
}
```

**or maybe**:
```rust
struct A {
    b: u32,
    c: u16,
    a: u8,
    _pad: u8,
}
```

<br>

The **N-byte aligned address** means that (**N** is the size of CPU's **memory access granularity**) this **address** is a **multiple** of **N**.<br>

<br>

All types have an **alignment** specified in bytes. A **value** of type with **alignment N**:
- must have **size** that is **multiple of N**;
- must be stored at an **address** that is **multiple of N**;

All primitive types are aligned to their size.<br>

Rust **guarantees** that **two instances** of **type A** have **the same layout**: **the same ordering** and **the same padding**.<br>
But Rust **doesn't** guarantee that instance of **type A** has the **same field ordering**/**padding** as instance of **type B**.<br>

For instance, consider this struct:
```rust
struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}
```

Consider two monomorphizations of `Foo<u32, u16>` and `Foo<u16, u32>`.<br>

If Rust **didn't** reorder fields, we would expect it to produce the following:
```rust
struct Foo<u16, u32> {
    count: u16,
    data1: u16,
    data2: u32,
}
```

```rust
struct Foo<u32, u16> {
    count: u16,
    _pad1: u16,
    data1: u32,
    data2: u16,
    _pad2: u16,
}
```

The **latter** case quite **simply wastes space**.<br>
So, an **optimal use of space** requires **different monomorphizations** to have **different field orderings**.<br>

Consider enum:
```rust
enum Foo {
    A(u32),
    B(u64),
    C(u8),
}
```

In memory it can be represented as:
```rust
struct FooRepr {
    data: u64, // this is either a u64, u32, or u8 based on `tag`
    tag: u8,   // 0 = A, 1 = B, 2 = C
}
```

But for pointers such representation is inefficient.<br>
Consider, `Option<&T>`. A **null pointer** can be interpreted as `None` variant and **non-null pointer** can be interpreted as the `Some(&T)`.<br>
So, `Option<&T>` makes the tag in enum representation is unnecessary for pointers. This is called **null pointer optimization**.

The **null pointer optimization** means: `size_of::<Option<&T>>() == size_of::<&T>()`.<br>

<br>

# `repr`
**Native types** in Rust **don't** have a **stable ABI representation**.<br>
There are attribute repr to guarantee appropriate representation for various cases.<br>

<br>

## `#[repr(C)]`
Means the **order**, **size** and **alignment** of fields is exactly **C/C++** does. In other words, `#[repr(C)]` repr guarantees **defined layout** of the type.<br>
Any type you expect to **pass through** an **FFI boundary** should have `repr(C)`.<br>

<br>

## `#[repr(transparent)]`
Can only be used on a **struct** or **enum** that has a **single non-zero-sized field** (there may be additional **zero-sized** fields).<br>
The **layout** and **ABI** of the **whole struct/enum** is **guaranteed** to be **the same as that one field**.<br>

The goal is to make it possible to `transmute` between the **single field** and **the struct/enum**.<br>
An example of that is `UnsafeCell`, **which can be transmuted into the type it wraps**.<br>

<br>

## `repr(u*)` and `repr(i*)`
Both specify the **size** and **sign** to make a **fieldless enum**.<br>
The term **fieldless enum** only means that the **enum doesn't have data in any of its variants**.<br>
These reprs guarantees **defined layout** for **fieldless enums** and makes it possible to pass the enum to **C** code.<br>
A fieldless enum **without** a `repr(u*)` or `repr(C)` is still a **Rust native type**, and **does not** have a stable ABI representation.<br>

<br>

## `repr(packed)` and `repr(packed(n))`
Here `n` is a **power of two**.<br>
The `repr(packed(n))` **forces** the type to have an **alignment** of **at most** `n`.<br>
The `repr(packed)` is **equivalent** to `repr(packed(1))` which forces Rust to **strip any padding**, and only align the type to a **byte**.<br>

<br>

## `repr(align(n))`
Here `n` is a **power of two**.<br>
The `repr(align(n))` **forces** the type to have an **alignment** of **at least** `n`.<br>
This enables, for example, make sure neighboring elements of array never share the same cache line.<br>
