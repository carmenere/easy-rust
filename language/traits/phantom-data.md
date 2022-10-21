# ``PhantomData``
```Rust
struct Tagged<T>(usize);
```

**Output**:
```bash
error[E0392]: parameter `T` is never used
 --> src/main.rs:1:15
  |
1 | struct Tagged<T>(usize);
  |               ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`
```

That’s right, we’re **not** allowed to have a **type parameter** that goes **unused**.<br>

<br>

If we want to have an **unused type parameter** we **have to** add a ``PhantomData`` to it like so:
```Rust
use std::marker::PhantomData;
struct Tagged<T>(usize, PhantomData<T>);
```

``PhantomData`` is **marker type**. It is **ZST**.<br>

Declaration of ``PhantomData``:
```Rust
pub struct PhantomData<T>
where
    T: ?Sized;
```

Adding a ``PhantomData<T>`` field to your type tells the compiler that your type acts as though it stores a value of type ``T``, even though it doesn’t really.<br>
This information is used when computing certain safety properties.
