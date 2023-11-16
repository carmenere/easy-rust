# Structs
`Struct` type is container for values of different types.

<br>

## Syntax
### *Type declaration* syntax
```Rust
struct <Name> {
    f1: T1,
    f2: T2,
    ...
}
```
where: `fi` - name of the `field` in the `struct` of type `Ti`.

#### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}
```

<br>

### *Initialization* syntax
#### `Struct` constructor 
```Rust
let v: <Name> = <Name> {
    f1: val_1,
    f2: val_2,
    ...
}
```

##### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}

fn main() {
    let v = Foo { bar: 1, baz: 2 };
    println!("Fields of 'v': 'bar' = {}, 'baz' = {}.", v.bar, v.baz);
}
```

<br>

#### Method `new()`
```Rust
let v: <Name> = <Name>::new(a=val_1, b=val_2, ... );
```

<br>

## `..` operator
A `struct` **constructor** can include `..` operator to **copy** some values from another variable of **the same** `struct` type. 

#### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}

fn main() {
    let v1 = Foo { bar: 1, baz: 2 };
    let v2 = Foo { bar: 77, .. v1};
    println!("Fields of 'v1': 'bar' = {}, 'baz' = {}.", v1.bar, v1.baz);
    println!("Fields of 'v2': 'bar' = {}, 'baz' = {}.", v2.bar, v2.baz);
}

Output:
Fields of 'v1': 'bar' = 1, 'baz' = 2.
Fields of 'v2': 'bar' = 77, 'baz' = 2.
```
