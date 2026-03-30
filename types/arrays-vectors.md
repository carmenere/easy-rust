# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Arrays](#arrays)
  * [*Initialization* syntax](#initialization-syntax)
    * [Syntax options for *pre initialized* arrays:](#syntax-options-for-pre-initialized-arrays)
    * [Syntax options for *empty* arrays:](#syntax-options-for-empty-arrays)
  * [*Type declaration* syntax](#type-declaration-syntax)
* [Vectors](#vectors)
  * [*Initialization* syntax](#initialization-syntax-1)
    * [Syntax options for *pre initialized* vectors:](#syntax-options-for-pre-initialized-vectors)
    * [Syntax options for *empty* vectors:](#syntax-options-for-empty-vectors)
  * [*Type declaration* syntax](#type-declaration-syntax-1)
<!-- TOC -->

<br>

# Arrays
An **array** is **fixed-size** *collection* of elements of **the same type**.<br>
Arrays are **allocated** on the **stack**.

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* arrays:
- **Comma-delimited**: explicit enumeration of values within square brackets \[\]:
```Rust
let arr = [0, 1, 2];
```

- **Repeat expression**: \[`V`; `N`\], where the **value** `V` is **repeated** `N`times:
```Rust
let arr = [100; 5];
```

### Syntax options for *empty* arrays:
- **Repeat expression** where `N` = 0:
```Rust
let a = [100; 0];
println!("len of 'a' is {}.", a.len());

Output:
len of a is 0.
```

<br>

## *Type declaration* syntax
- **Repeat expression**: \[`T`; `N`\], where the value of a **type** `T` is **repeated** `N` times:
```Rust
let arr1: [u64; 3] = [0, 1, 2];

let arr2: [u64; 3] = [100; 3];
```

<br>

# Vectors
A **vector** is *collection* of elements of **the same type** that is allowed to **grow** or **shrink** *in size* **at runtime**.<br>
Vectors are **allocated** on the **heap**.<br>
`Vec` is a type for **vector** provided by the **standard library**.<br>

`capacity` is the number of elements the `Vec` can hold without reallocating.

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* vectors:
- **Comma-delimited** by `vec!` macros: explicit enumeration of values within square brackets \[\]:
```Rust
let v = vec![0, 1, 2];
```

- **Repeat expression** by `vec!` macros: \[`V`; `N`\], where the **value** `V` is **repeated** `N` times:
```Rust
let v = vec![100; 5];
```

### Syntax options for *empty* vectors:
- **Vector type constructor**:
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```
- **Repeat expression** where `N` = 0:
```Rust
let v = vec![100; 0];
```

<br>

## *Type declaration* syntax
- `Vec<T>`
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```
