# Result
``Result`` express **ability** of **error** through ``enum``.<br>
``Result`` is **more general** than ``Option``. <br>

Path to ``Result`` in **std**: ``std::result::Result``.<br>

```Rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

``T`` – type of **value**. ``T`` is wrapped in ``Ok`` variant.<br>
``E`` – type of **error**. ``E`` is wrapped in ``Err`` variant.<br>

Value of type ``T`` or ``E`` can only be obtained via ``match``:
```Rust
fn example(s: Option<i32>) -> Result<i32, &'static str> {
    match s {
        None => Err("invalid header length"),
        Some(val) => Ok(val) 
    }
}

let r1 = example(Some(1));
let r2 = example(None);
```
