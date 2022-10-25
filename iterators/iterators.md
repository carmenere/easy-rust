# Iterators
An **iterator** is an object that enables to **traverse its elements**.<br>
**Iterators** are **stateful** because they **keep track** **of where they are** in the iteration process.<br>

The **iterator** supports at least the following features:
- **get** the current element;
- **advance** to the next element;
- **signal** when no more elements are available and return ``None``.

In Rust, **iterator** must implement ``Iterator`` trait.<br>

<br>

# Trait ``Iterator``
Path in **std**: ``std::iter::Iterator``.<br>
**Defenition**:
```Rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}
```
**Associated types**:
|**Type**|**Description**|
|:-------|:--------------|
|``Item``|The **type of the elements** ``Iterator`` **yields**.|

<br>

The ``next()`` method returns ``Some(Item)`` or ``None`` to **indicate** the **end of the sequence**.<br>

``Iterator`` can also be useful in generic code: 
- you can use a bound like ``T: Iterator`` to restrict the type variable to types that are iterators;
- you can write ``T: Iterator<Item=U>`` to restrict the type variable to types that are **iterators** **that yield a particular type** ``U``.

<br>

#### Example
```Rust
struct MyIterator;

impl Iterator for MyIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> { 
        Some("abc".to_string()) 
    }
}

fn main() {
    let mut iter = MyIterator;
    let value = iter.next();
}
```
