# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declarations](#declarations)
  - [`ArcInner<T>`](#arcinnert)
  - [`Arc<T>`](#arct)
- [In a nutshell](#in-a-nutshell)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Arc`|[**std::sync::Arc**](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html)|

<br>

# Declarations
## `ArcInner<T>`
```rust
#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,
    weak: atomic::AtomicUsize,
    data: T,
}
```

<br>

## `Arc<T>`
```rust
pub struct Arc<T, A = Global>
where
    A: Allocator,
    T: ?Sized
{
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
    alloc: A,
}
```


<br>

# In a nutshell
`Arc` stands for **Atomic Reference Counted**.<br>
The `Arc<T>` type is **thread-safe reference-counting pointer**. It uses **atomic reference counting**.<br>
But, **Rc** is **faster** than **Arc**.<br>

The `Arc<T>` type **implements** `Send` and `Sync` if type `T` does:
```rust
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Send> Send for Arc<T, A> {}
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Sync> Sync for Arc<T, A> {}
```
