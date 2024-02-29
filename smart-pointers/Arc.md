# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Arc](#arc)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Arc`|[std::sync::Arc](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html)|

<br>

# Arc<T>
`Arc` stands for **Atomic Reference Counter**.<br>
It's **threadsafe** version of `Rc`.<br>
But, **Rc** is **faster** than **Arc**.

<br>

```Rust
use std::sync::Arc;
```

<br>
