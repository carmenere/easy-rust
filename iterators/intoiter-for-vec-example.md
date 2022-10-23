# Struct ``std::vec::IntoIter``
The ``into_iter()`` method on ``Vec`` returns **struct** ``std::vec::IntoIter``:
```Rust
pub struct IntoIter<T, A = Global> where
    A: Allocator,  
    { /* private fields */ }
```

<br>

### Example
```Rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

<br>

**Struct** ``std::vec::IntoIter`` is **iterator** because it implements ``Iterator``:
```Rust
impl<T, A> Iterator for IntoIter<T, A> where
    A: Allocator, 
type Item = T
```
