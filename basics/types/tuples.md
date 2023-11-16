# Tuples
A `tuple` is an **ordered** and **fixed size** *collection* of elements of *different types*. 

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* tuples:
**Comma separated** list of **values** *enclosed* in **parentheses**, e.g., `(val1, val2, val3)`.
- **2-length tuple**
```Rust
let tup = (1, "foo");
```
- **5-length tuple**
```Rust
let tup = (1, "foo", 2, "bar", 3);
```

<br>

## *Type declaration* syntax
- **Comma separated** list of **type declarations** *enclosed* in **parentheses**, e.g., `(i32, &str)`.
```Rust
let x: (i32, &str) = (1, "hello");
```
 
<br>

## Access to fields of a tuple
There is **dot notation** `.<index_of_field>` to access to field in a tuple with index of `<index_of_field>`.<br>

Example:
```Rust
let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);
```
