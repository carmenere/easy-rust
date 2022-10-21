# Option
``Option`` express **optionality** through ``enum``.<br>
Path to ``Option`` in **std**: ``std::option::Option``: https://doc.rust-lang.org/std/option/enum.Option.html<br>

```Rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Value of type ``T`` can only be obtained via ``match``:
```Rust
match val { 
Some(val) => {any code using val},
None => expr
}
```

Here ``val`` is of type ``Option<T>``, after deconstructing, ``val`` is of type ``T``.