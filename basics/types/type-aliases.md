# Type aliases
```Rust
fn main () {
    type meters = u64;
    let a: meters = 10;
    let b: u64 = 20;
    assert_eq!(a+b, 30u64);
}
```

In the above example, `meters` is **not** a new type, it is **alias** (**synonym**) for `u64`, so `meters` is treated as `u64`.

<br>

Another example:
```Rust
type MySend = Box<dyn Fn() + Send + 'static>;
```

In the above example type alias `MySend` can be used instead `Box<dyn Fn() + Send + 'static>`.