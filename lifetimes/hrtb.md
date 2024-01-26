# Higher-Rank Trait Bounds (HRTBs)
**HRTBs** are most commonly used with the `Fn*` traits.<br>
This compiles:
```rust
pub fn foo(f: impl for<'a> Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = f(&y);
    *z
}

fn main() {
    foo(|_| &5);
}
```

<br>

But this code does **not** compile:
```rust
pub fn bar<'a>(f: impl Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = f(&y);
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

- The **first** one says: the lifetime `'a` of the returned reference from `f` is the same as for the passed argument of `f` and it's **not** **directly related** to the **input** or **output** lifetimes of function `foo`.<br>
- The **second** one says: there is a lifetime `'a` for which `f` takes a reference with that lifetime and also returns a reference with that lifetime. The lifetime of **local variable** `y` passed to `f` is **shorter** than `'a` in this case, but it must be at least as long as `'a`.<br>

<br>

`for<'a>` means that the **reference can be valid for any lifetime** (hence a smaller lifetime can be used).<br>