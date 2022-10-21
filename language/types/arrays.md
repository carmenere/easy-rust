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

- **Repeat expression**: \[``V``; ``N``\], where the **value** ``V`` is **repeated** ``N``times:
```Rust
let arr = [100; 5];
```

### Syntax options for *empty* arrays:
- **Repeat expression** where ``N`` = 0:
```Rust
let a = [100; 0];
println!("len of 'a' is {}.", a.len());

Output:
len of a is 0.
```

<br>

## *Type declaration* syntax
- **Repeat expression**: \[``T``; ``N``\], where the value of a **type** ``T`` is **repeated** ``N`` times:
```Rust
let arr1: [u64; 3] = [0, 1, 2];

let arr2: [u64; 3] = [100; 3];
```
