# Macros panic!
``panic!`` macro should be used when a **program reaches** an **unrecoverable state**.<br>
This allows a program to **terminate** immediately and provide feedback to the caller of the program.<br>

```Rust
if n < 1 || n > 100 {
    panic!("Incorrect number: {}", n);
}
```
