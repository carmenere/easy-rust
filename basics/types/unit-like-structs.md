# Unit-like structs
`unit-like struct` type is a `struct` with no members.<br>
`unit-like struct` type is a **zero-sized type** (**ZST**).

## Syntax
### *Type declaration* syntax
```Rust
struct <Name>;
```

#### Examples
```Rust
struct Foo;
struct Bar;
```

<br>

### *Initialization* syntax
```Rust
let v = <Name>;
```

#### Examples
```Rust
let f = Foo;
let b = Bar;
```
