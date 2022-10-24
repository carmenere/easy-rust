# Trait ``FromIterator``
Trait ``FromIterator`` is used for conversion **from** an ``Iterator`` **to** **collection**.<br>
Path in **std**: ``std::iter::FromIterator``.<br>
**Defenition**:
```Rust
pub trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}
```

By implementing ``FromIterator`` for a **collection** type, you define how it will be created **from** an **iterator**.<br>
``FromIterator::from_iter()`` is rarely called explicitly, and is instead used through ``Iterator::collect()`` method.<br>

<br>

If you want to create a **collection** from the **content** of an **iterator**, the ``Iterator::collect()`` method is preferred.<br>
However, when you need to specify the **container type**, ``FromIterator::from_iter()`` can be more readable than using a **turbofish**, e.g., ``::<Vec<_>>()``.<br>

<br>

### Examples
#### Basic usage
```Rust
let five_fives = std::iter::repeat(5).take(5);

let v = Vec::from_iter(five_fives);

assert_eq!(v, vec![5, 5, 5, 5, 5]);
```

<br>

#### Using ``Iterator::collect()`` to implicitly use ``FromIterator``:
```Rust
let five_fives = std::iter::repeat(5).take(5);

let v: Vec<i32> = five_fives.collect();

assert_eq!(v, vec![5, 5, 5, 5, 5]);
```

<br>

# Trait ``IntoIterator``
Trait ``IntoIterator`` is used for conversion **from** **collection** **to** an ``Iterator``.<br>
Path in **std**: ``std::iter::IntoIterator``.<br>
**Defenition**:
```Rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator
    where
        <Self::IntoIter as Iterator>::Item == Self::Item;

    fn into_iter(self) -> Self::IntoIter;
}
```

One benefit of implementing ``IntoIterator`` is that your type will work with Rust’s for loop syntax.<br>
