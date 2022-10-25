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
``FromIterator::from_iter()`` is rarely called explicitly, and ``FromIterator::from_iter()`` is usually used through ``Iterator::collect()`` method.

<br>

# Method ``Iterator::collect()``
``collect()`` transforms an **iterator** into a **collection**.<br>
``collect()`` can also create instances of types that are not typical collections, e.g., ``collect()`` can return ``Result<SomeCollection<T>, E>``.

<br>

#### Using ``Iterator::collect()`` to implicitly use ``FromIterator``:
```Rust
let five_fives = std::iter::repeat(5).take(5);

let v: Vec<i32> = five_fives.collect();

assert_eq!(v, vec![5, 5, 5, 5, 5]);
```

<br>

**Declaration**:
```Rust
fn collect<B: FromIterator<Self::Item>>(self) -> B
where
    Self: Sized,
{
    FromIterator::from_iter(self)
}
```

<br>

# ``Turbofish``
Because ``collect()`` is so general, it can cause **problems** with **type inference**.<br>
Internally, ``collect()`` just uses ``FromIterator``, but it also **infers** the **type** of the **output**.<br>
Sometimes there **isn't enough** information to infer the type, so you may need to **explicitly** specify the type you want.<br>
There is *special syntax* in Rust called **turbofish**: ``::<SomeType>``.<br>
Example: ``let all_scores = score_table.values().cloned().collect::<Vec<Score>>();``.<br>
**Turbofish** helps the **inference** algorithm to understand type of item of **resulting collection**.<br>

### Example
```Rust
fn main() {
    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);
}
```

**Output**:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0282]: type annotations needed
 --> src/main.rs:6:9
  |
6 |     let even_numbers = numbers
  |         ^^^^^^^^^^^^ consider giving `even_numbers` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playrs` due to previous error
```

<br>

This is because the compiler **doesn’t know** what type you’re trying to collect your **iterator** into.<br>

This can be fixed in two different ways:
- by **declaring** the **type** of variable in ``let`` **binding**: 
```Rust
let even_numbers: Vec<i32> = ...
```
- by using a **turbofish**:
```Rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<i32>>();
```

The ``::<Vec<i32>>`` part is the **turbofish** and means collect this **iterator** into a ``Vec<i32>``.<br>

You can actually replace ``i32`` with ``_`` in **turbofish** and let the compiler infer it because it knows the **iterator** yields ``i32``:
```Rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<_>>();
```
