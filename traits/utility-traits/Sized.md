# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declaration](#declaration)
* [`Sized`](#sized)
* [`?Sized`](#sized-1)
* [`?Sized` generic](#sized-generic)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Sized`|[std::marker::Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html)|

<br>

# Declaration
```rust
pub trait Sized { }
```

<br>

# `Sized`
The `Sized` trait is the **marker trait** and indicates that the **size** of type is **known** *at compile time*.<br>
The `Sized` trait is implemented **automatically** by the compiler for most types. In other words, most types have **implicit** `Sized` bound **by default**.<br>

Also Rust adds the `Sized` bound to all **generics** (`T: Sized`). In other words, any **type parameter** (except `Self` in traits) or **associated type** has **implicit** `Sized` bound **by default**:
```rust
struct Foo<T>(T);
struct FooUse(Foo<[i32]>); // Error: the trait `Sized` is not implemented for `[i32]`
```

<br>

# `?Sized`
Most types have a **fixed size** that is _known_ **at compile time** and implement the trait `Sized`.<br>
If **size** is _known_ **only at run-time** such types are called a **dynamically sized type** (**DST**), or, informally, **unsized types**.<br>
All **DST** types **don't** implement `Sized`.<br>
The special syntax `?Sized` is used to **remove** default `Sized` bound. So, **DST** use `?Sized` bound.<br>

<br>

> **Note**:<br>
> Variables, function parameters, const items, and static items must be `Sized`.

<br>

**Unsized types**:
- **slices**: `[T]`;
- **trait objects**: `dyn SomeTrait`;

<br>

Pointers to instances of DST types `&dyn SomeTrait` and `&[T]` are **fat pointers**.<br>
**Fat pointers** are **sized** but have **twice the size**:
- **fat pointer to slice** `&[T]` consists of **2 elements**:
  - **pointer** to **first element** of slice;
  - **number of elements** in slice (this is of `isize`/`usize`);
- **fat pointer** `&dyn SomeTrait` to **trait object** `dyn SomeTrait` consists of **2 pointers**:
  - **pointer** to **instance** of some type `T`;
  - **pointer** to a **vtable** for `<T as SomeTrait>` (for `Trait` that implemented for type `T`);

<br>

# `?Sized` generic
```rust
struct Bar<T>(T) where T: ?Sized;
struct BarUse(Bar<[i32]>); // OK
```