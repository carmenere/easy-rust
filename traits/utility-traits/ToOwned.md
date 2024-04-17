# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)
  - [Examples](#examples)
    - [`.as_ref()` with `.to_owned()`](#as_ref-with-to_owned)
- [Declaration](#declaration)
- [Implementations in `std`](#implementations-in-std)
- [Blanket implementations](#blanket-implementations)
  - [`impl<T> ToOwned for T`](#implt-toowned-for-t)
  - [`impl<T> ToOwned for [T]`](#implt-toowned-for-t-1)
- [Difference between .clone() and .to\_owned()](#difference-between-clone-and-to_owned)
  - [`.clone()`](#clone)
  - [`.to_owned()`](#to_owned)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`ToOwned`|[std::borrow::ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)|

<br>

# In a nutshell
The `ToOwned` trait is a **generalized** way to **convert** a **borrowed type** to an **owned type**.<br>
Method `.to_owned()` always generates **owned type** `O` from `&T`, it is **not always the same type**, **but** it **always owned**.<br>
There is also `Clone` trait. Unlike `ToOwned` trait the `Clone` works **only** for going **from** `&T` **to** `T`.<br>

<br>

## Examples
```rust
let s: String = "hello".to_owned();

let s: String = "hello".to_string();
let s: String = "hello".into();

let s: String = String::from("hello");
```

<br>

### `.as_ref()` with `.to_owned()`
```Rust
fn f<S>(p: S)
where S: AsRef<String>
{ 
    let x = p.as_ref().to_owned();
}
```

<br>

# Declaration
```Rust
pub trait ToOwned {
    type Owned: Borrow<Self>;

    fn to_owned(&self) -> Self::Owned;

    fn clone_into(&self, target: &mut Self::Owned) { ... }
}
```

<br>

# Implementations in `std`
- `impl ToOwned for str` returns `String`:
```rust
impl ToOwned for str {
   type Owned = String;
   fn to_owned(&self) -> String {
      unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
   }
}
```
- d
```rust
```


# Blanket implementations
## `impl<T> ToOwned for T`
```rust
impl<T> ToOwned for T
where
    T: Clone,
{
    type Owned = T;
    fn to_owned(&self) -> T {
        self.clone()
    }

    fn clone_into(&self, target: &mut T) {
        target.clone_from(self);
    }
}
```

<br>

## `impl<T> ToOwned for [T]`
```rust
impl<T: Clone> ToOwned for [T] {
   type Owned = Vec<T>;
   #[cfg(not(test))]
   fn to_owned(&self) -> Vec<T> {
      self.to_vec()
   }

   #[cfg(test)]
   fn to_owned(&self) -> Vec<T> {
      hack::to_vec(self, Global)
   }

   fn clone_into(&self, target: &mut Vec<T>) {
      SpecCloneIntoVec::clone_into(self, target);
   }
}
```

<br>

# Difference between .clone() and .to_owned()
## `.clone()`
`.clone()` generates:
1. `String` from `&String`.
2. `T` from `&T` for **scalar types** and **composite types** if all their **constituent types** are *scalar types*.
3. `[T]` from `&[T]` for arrays **with** a specified length or without type definition:
   - `let a = &[1,2,3];`
   - `let a: &[i32; 3] = &[1,2,3];`
4. `&[T]` from `&[T]` for arrays **without** a specified length:
   - `let a: &[i32] = &[1,2,3];`
5. `&str` from `&str`.

<br>

## `.to_owned()`
`.to_owned()` generates:
1. `String` from `&String`.
2. `T` from `&T` for scalar types and composite types if all their **constituent types** are *scalar types*.
3. `[T]` from `&[T]` for arrays **with** a specified length or without type definition:
   - `let a = &[1,2,3];`
   - `let a: &[i32; 3] = &[1,2,3];`
4. `Vec<T>` from `&[T]` for arrays **without** a specified length:
   - `let a: &[i32] = &[1,2,3];`
5. `String` from `&str`.
