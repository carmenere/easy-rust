# Enums
An `enum` in Rust is **tagged union** or **sum type**.<br>
`enum` consists of different `variants`.<br>
Each `variant` in the `enum` reperesents **some** `type`.<br>
A value of an `enum` type matches to **one specific** `variant`.

<br>

## Syntax
### *Type declaration* syntax
```Rust
enum <Name> {
    Variant_1,
    Variant_2(SomeType_2),
    Variant_3(SomeType_3),
}
```
where `Variant_i` wraps type `SomeType_i` or without any type like `Variant_1`.

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
Variable of ``enum`` type can **only** be initialized with **specific** value of type ``SomeType_i``:
```Rust
let x: SomeType_i = SomeType_i::new(...);
let v: <Name> = <Name>::Variant_i(x);
```

<br>

## Access to ``enum`` variant
To access to **specific** ``variant`` of variable of ``enum`` type **pattern matching** is used.

#### Example
```Rust
#[derive(Debug)]
struct MyStruct { x: i32, y: i32 }

#[derive(Debug)]
enum MyEnum {
    Bar,
    Foo(i32, i32, i32),
    Baz (MyStruct),
    FooBar(String),
}

fn main() {
    let s = MyStruct { x: 3, y: 4 };

    let v = MyEnum::Baz(MyStruct { x: 3, y: 4 });

    match &v {
        MyEnum::Bar => println!(""),
        MyEnum::Foo(x, y, z) => println!("MyEnum::Foo"),
        MyEnum::Baz(MyStruct) => println!("MyEnum::Baz"),
        MyEnum::FooBar(val) => println!("MyEnum::FooBar"),
    }

    println!("{:X?}", v);
}
```
