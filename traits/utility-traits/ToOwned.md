# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [ToOwned](#toowned)
      - [Example](#example)
  - [Declaration](#declaration)
  - [Difference between .clone() and .to\_owned()](#difference-between-clone-and-to_owned)
    - [`.clone()`](#clone)
    - [`.to_owned()`](#to_owned)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`ToOwned`|[std::borrow::ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)|

<br>

# ToOwned
Some types make it possible to go from **borrowed** to **owned**, usually by implementing the `Clone` trait.<br>
But `Clone` works only for going from `&T` to `T`.<br>
The `ToOwned` trait is a **generalized** way to **convert** a **borrowed type** to an **owned type**.<br>
Method `to_owned` called on *reference* (*borrowed type*) returns **owned type**.

<br>

#### Example
```Rust
fn f<S>(p: S)
where S: AsRef<String> { 
    let x = p.as_ref().to_owned();
}
```

<br>

## Declaration
```Rust
pub trait ToOwned {
    type Owned: Borrow<Self>;

    fn to_owned(&self) -> Self::Owned;

    fn clone_into(&self, target: &mut Self::Owned) { ... }
}
```

<br>

## Difference between .clone() and .to_owned()
### `.clone()`
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

### `.to_owned()`
`.to_owned()` always generates **owned type** from `&T`, it is **not** *always* the same type, but it *always* **owned**.<br>
For instance, the `.to_owned()` method generates a `Vector` from a reference to arrays **without** a specified length: `&[i32]` -> `Vec<T>`.<br>

`.to_owned()` generates:
1. `String` from `&String`.
2. `T` from `&T` for scalar types and composite types if all their **constituent types** are *scalar types*.
3. `[T]` from `&[T]` for arrays **with** a specified length or without type definition:
   - `let a = &[1,2,3];`
   - `let a: &[i32; 3] = &[1,2,3];`
4. `Vec<T>` from `&[T]` for arrays **without** a specified length:
   - `let a: &[i32] = &[1,2,3];`
5. `String` from `&str`.
