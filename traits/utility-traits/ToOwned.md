# ToOwned
Method ``to_owned`` called on *reference* returns **owned type**.<br>
Some types make it possible to go from **borrowed** to **owned**, usually by implementing the `Clone` trait.<br>
But `Clone` works only for going from `&T` to `T`.<br>
The `ToOwned` trait generalizes `Clone` to construct *owned* data from **any** *borrow* of a given type.

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
