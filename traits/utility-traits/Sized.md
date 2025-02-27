# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [`Sized`](#sized)
- [`?Sized`](#sized-1)
- [Examples](#examples)
  - [`Sized` trait](#sized-trait)
  - [`?Sized` generic](#sized-generic)

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
The `Sized` trait is the **marker trait** and it indicates that the **size** of type is **known** *at compile time*.<br>
The `Sized` trait is implemented **automatically** by the compiler for most types. In other words, most of types have **implicit** `Sized` bound **by default**.<br>

Also Rust adds the `Sized` bound to all **generics** (`T: Sized`). In other words, any **type parameter** (except `Self` in traits) or **associated type** has **implicit** `Sized` bound **by default**:
```rust
struct Foo<T>(T);
struct FooUse(Foo<[i32]>); // Error: the trait `Sized` is not implemented for `[i32]`
```

<br>

# `?Sized`
**DST** (**dynamically sized types**, aka **unsized types**) are types that **grow** or **shrink dynamically** *at runtime* and their **sizes** are **not known** *at compile time*.<br>
All **DST** don't implement `Sized`.<br>
The special syntax `?Sized` is used to **remove** default `Sized` bound. So, **DST** use `?Sized` bound.<br>

Examples of **DST**:
- slices `[T]`;
- `trait <name> {}`;
- `String`;

So, a trait **doesn't** have an implicit `Sized` bound as this is **incompatible** with **trait objects**.<br>
Although Rust **allows** define `Sized` for traits, but such `Sized` traits **cannot** be used as **trait objects** later. The `Sized` trait is **not** **dyn compatible**. In older versions of Rust, **dyn compatibility** was called **object safety**, so this trait is **not object safe**. <br>
Notation for defining `Sized` traits: `trait <name>: Sized { }`.<br>

So,
- `Self` type of any trait is `?Sized` **by default**;
- `Sized` trait (`trait <name>: Sized { }`) **requires** `Self: Sized`;

<br>

# Examples
## `Sized` trait
```rust
trait Foo { }
trait Bar: Sized { }

struct Impl;
impl Foo for Impl { }
impl Bar for Impl { }

let x: &dyn Foo = &Impl; // OK
let y: &dyn Bar = &Impl; // Error
```

**Error**:
```rust
20 |     trait Bar: Sized { }
   |           ---  ^^^^^ ...because it requires `Self: Sized`
   |           |
   |           this trait cannot be made into an object...
```

<br>

## `?Sized` generic
```rust
struct Bar<T>(T) where T: ?Sized;
struct BarUse(Bar<[i32]>); // OK
```