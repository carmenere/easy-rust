# Table of contents
- [Table of contents](#table-of-contents)
- [Pinning](#pinning)

<br>

# Pinning
If **pointer** is wrapped into `Pin<P>`, it means the value pointer points to will **no longer move**.<br>
`Pin` allows to create **immovable** `Futures`.<br>
Also there is marker trait `Unpin` that **disable** such restirction.<br>

The `poll()` method requires the future be passed as `Pin<&mut Self>` value.<br>
So, you cannot poll future until you’ve constructed a `Pin` wrapper for it, and once you have done that, the future can’t be moved.<br>
This restrictions for `Pin` type are implemented in code-generated `Future` implementation.

Pin type:
```rust
pub struct Pin<P> {
    pointer: P,
}
```

<br>

There is `Box::pin(value: T)` constructor to **make reference pinned**: it takes ownership of value of type `T` and returns `Pin<Box<T>>`.<br>
`Pin<Box<T>>` implements `From<Box<T>>`, so `Pin::from(value: T)` takes ownership of value of type `T` and returns `Pin<Box<T>>`.
