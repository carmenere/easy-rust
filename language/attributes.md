# Attributes
Declaration of **any item** in a Rust can be decorated (annotated) with one or more **attribute**. <br>
Each **attribute** contains **instructions for compiler**.<br>


In Rust items are
- Functions
- Types (structs, enums, unions, type aliases)
- Traits
- Impl blocks
- Macros
- Constants and statics
- Extern blocks
- Extern crates
- Imports
- Modules

<br>

There are 2 types of attributes:
- **inner** attributes;
- **outer** attributes.

<br>

To attach attribute **to whole crate** include **inner atribute** to the **root module**: ``main.rs`` or ``lib.rs``.

<br>

## Inner attributes
**Inner** attributes apply to **all items** within the scope where attribute is declared.

#### Example
```Rust
mod Bar {
    #![bar]
}
```

Here, the ``#![bar]`` attribute applies **to all items** inside module ``Bar``.

<br>

## Outer attributes
**Outer** attributes apply **only to 1 item** following the attribute.

#### Example
```Rust
#[foo]
struct Foo;
```
Here, the ``#[foo]`` attribute applies **only to the next item** ``Foo``.

<br>

## Attributes arguments
Some *attributes* **require arguments**, some *attributes* can be used **without arguments**.

#### Attributes without arguments
```Rust
// A unit test
#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}
```

<br>

#### Attributes with arguments
```Rust
// A conditional compilation
#[cfg(target_os = "linux")]
mod bar {
    /* ... */
}

// A lint attribute
#[allow(non_camel_case_types)]
type int8_t = i8;

```

<br>

## Kinds of attributes
- **built-in** attributes;
- **tool** attributes;
- **macro** attributes;
- **derive macro helper** attributes.

<br>

# Built-in attributes
**Built-in attributes** per categories:
- Conditional compilation
    - ``cfg``
- Testing
    - ``test``
- Derive
    - ``derive``
- Macros
    - ``proc_macro``
- Lint
    - ``allow``
    - ``warn``
    - ``deny``
    - ``forbid``
    - ``deprecated``
    - ``must_use``
- Code generation
    - ``inline``
- Documentation
    - ``doc``
- Preludes
    - ``no_std``
- Modules
    - ``path``
- Limits
    - ``recursion_limit``
    - ``type_length_limit``
- Runtime
    - ``panic_handler``
    - ``global_allocator``
- ABI, linking, symbols, and FFI

<br>

## Examples
### ``derive``
The ``derive`` attribute allows *certain* **traits** to be **automatically implemented** for data structures.
```Rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

<br>

### ``path``
The ``path`` attribute specifies the filename for a module.
```Rust
#[path = "foo.rs"]
mod c;
```

It means file ``foo.rs`` will be included into module tree as ``c`` module.
