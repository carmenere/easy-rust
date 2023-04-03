# Tuple structs
``tuple struct`` is a hybrid between a ``tuple`` and a ``struct``.<br>
Unlike the ``struct`` type, the fields in the ``tuple struct`` don't have names.

<br>

## Syntax
### *Type declaration* syntax
```Rust
struct <Name> (T1, ... );
```
where ``Ti`` is of some type.

#### Examples
```Rust
struct Foo(i32, i32, i32);
struct Bar(i32, i32, i32);
```

<br>

### *Initialization* syntax
```Rust
let v = <Name>(val_1, ... );
```
where ``val_i`` - value of some type ``Ti``.

#### Examples
```Rust
let f = Foo(0, 0, 0);
let b = Bar(0, 0, 0);
```
