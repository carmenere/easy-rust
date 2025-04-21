# Transmutes
The `std::mem::transmute<S,D>(_src: S) -> D` takes a value of type `T` and casts it into another type `U`.<br>

<br>

**Requirements**:
- transmuting **requires** `T` and `U` to have **the same size**, compilation will **fail** if this is **not guaranteed**;
- transmuting different compound types **requires** them to have **the same layout**;
  - for `repr(C)` and `repr(transparent)` the **layout** is **defined**;

<br>

**UB**:
1. Transmuting `&` to `&mut` is a **UB**.

<br>

The `std::mem::transmute_copy<S,D>(_src: &S) -> D` interprets `src: &S` as having `&D` type, and then reads `src` without moving the contained value.<br>