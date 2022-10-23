# Method ``Iterator::collect()``
``collect()`` transforms an **iterator** into a **collection**.<br>
``collect()`` can also create instances of types that are not typical collections.<br>

For example, ``collect()`` can return ``Result<SomeCollection<T>, E>``.

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
There is *special syntax* in Rust called **turbofish**: ``::<SomeType>``.<br>
**Turbofish** helps the **inference** algorithm to understand type of item of **resulting collection**.

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

This is because the compiler **doesn’t know** what type you’re trying to collect your iterator into.<br>

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

The ``::<Vec<i32>>`` part is the **turbofish** and means collect this iterator into a ``Vec<i32>``.<br>

You can actually replace ``i32`` with ``_`` in **turbofish** and let the compiler infer it because it knows the iterator yields ``i32``:
```Rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<_>>();
```
