# Higher-Rank Trait Bounds (HRTBs)
**HRTBs** are most commonly used with the `Fn*` traits.<br>
This compiles:
```rust
pub fn foo(x: impl for<'a> Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = x(&y);
    *z
}

fn main() {
    foo(|_| &5);
}
```

<br>

But this code does **not** compile:
```rust
pub fn bar<'a>(x: impl Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = x(&y);
    *z
}

fn main() {

}
```
Error:
```bash
y does not live long enough
```

<br>

The **first** one says: the lifetime `'a` of the returned reference from `x` is the same as for the reference argument it's given.<br>
The **second** one says: there is a lifetime `'a` for which `x` takes a reference argument with that lifetime and also returns a reference with that lifetime.<br>
The lifetime of **local variable** `y` passed to `x` is **shorter** than `'a` in this case, but it must be at least as long as `'a`.<br>

`for<'a>` means that the **reference can be valid for any lifetime** (hence a smaller lifetime can be used).<br>