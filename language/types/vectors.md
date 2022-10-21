# Vectors
A **vector** is *collection* of elements of **the same type** that is allowed to **grow** or **shrink** *in size* **at runtime**.<br>
Vectors are **allocated** on the **heap**.<br>
``Vec`` is a type for **vector** provided by the **standard library**.<br>

``capacity`` is the number of elements the ``Vec`` can hold without reallocating.

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* vectors:
- **Comma-delimited** by ``vec!`` macros: explicit enumeration of values within square brackets \[\]:
```Rust
let v = vec![0, 1, 2];
```

- **Repeat expression** by ``vec!`` macros: \[``V``; ``N``\], where the **value** ``V`` is **repeated** ``N``times:
```Rust
let v = vec![100; 5];
```

### Syntax options for *empty* vectors:
- **Vector type constructor**: \[``V``; ``N``\], where the **value** ``V`` is **repeated** ``N``times:
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```

<br>

## *Type declaration* syntax
- ``Vec<T>``
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```
