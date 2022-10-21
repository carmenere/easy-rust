# Enums
An ``enum`` in Rust is **tagged union** or **sum type**.<br>
``enum`` consists of different ``variants``.<br>
Each ``variant`` in the ``enum`` reperesents **some** ``type``.<br>
A value of an ``enum`` type matches to **one specific** ``variant``.

<br>

## Syntax
### *Type declaration* syntax
```Rust
enum <Name> {
    Variant_1,
    Variant_2,
    Variant_3,
}
```
where ``Variant_i`` is of some type.

#### Example
```Rust
enum MyEnum {
    Bar,
    Foo(i32, i32, i32),
    Baz { x: i32, y: i32 },
    FooBar(String),
}
```

<br>

### *Initialization* syntax
Variable of ``enum`` type can **only** be initialized with **specific** value of type ``Variant_i``:
```Rust
let v: <Name> = <Name>::Variant_1_constructor;
```

#### Example
```Rust
let v: MyEnum = MyEnum::Baz { x: 3, y: 4 };
```

<br>

## Access to ``enum`` variant
To access to **specific** ``variant`` of variable of ``enum`` type **pattern matching** is used.

#### Example
```Rust
enum MyEnum {
    Bar,
    Foo(i32, i32, i32),
    Baz {x: i32, y: i32},
    FooBar(String),
}

fn main() {
    let v: MyEnum = MyEnum::Baz { x: 3, y: 4 };
    match v {
        MyEnum::Bar => println!(""),
        MyEnum::Foo(x, y, z) => println!("MyEnum::Foo"),
        MyEnum::Baz{x, y} => println!("MyEnum::Baz"),
        MyEnum::FooBar(val) => println!("MyEnum::FooBar"),
    }
}
```
