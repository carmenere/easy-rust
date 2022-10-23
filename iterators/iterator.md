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

<br>

# Iterable
**Collections** usually implement ``IntoIterator`` **to be able to iterate** *over its items*.<br>
**Iterable type** is a **any type** that implements ``IntoIterator`` trait.<br>
``IntoIterator`` returns ``Iterator`` over some type ``U``.<br>

<br>

# Trait ``IntoIterator``
Path in **std**: ``std::iter::IntoIterator``.<br>

**Defenition**:
```Rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

So, ``IntoIterator`` returns ``Iterator`` over ``Self::Item``.<br>

**Associated types**:
|**Type**|**Description**|
|:-------|:--------------|
|``Item``|The type of the elements in collection.|
|``IntoIter: Iterator``|The method ``into_iter()`` returns ``Iterator`` whose elements are ``Self::Item``.|

<br>


``IntoIterator`` for collection, e.g, ``SomeCollection``, can return ``Iterator`` **over**: 
- *values* of type ``T``, **move semantics**;
- *shared references*, **allow** to **reuse** *original collection* **after** iteration;
- *mutable references*, **allow** to **reuse** *original collection* **after** iteration.

<br>

<table>
    <tr>
        <th>Case</th>
        <th>Description</th>
    </tr>
<tr></tr>
<tr>
<td>

Over **values** of type **T**

</td>


<td>

```Rust
impl IntoIterator for SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=T> {
  }
}
```

</td>
</tr>

<tr></tr>
<tr>
<td>

Over **shared references**: ``&T``

</td>

<td>

```Rust
impl IntoIterator for &SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&T> {
  }
}
```

</td>
</tr>

<tr></tr>
<tr>
<td>

Over **mutable references**: ``&mut T``

</td>

<td>

```Rust
impl IntoIterator for &mut SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&mut T> {
  }
}
```

</td>
</tr>

</table>

<br>

### Examples
For example, ``Vec`` implements ``IntoIterator`` for **all** cases: ``T``, ``&T`` and ``&mut T``. But not every type provides all these 3 implementations:
- ``HashSet`` and ``BinaryHeap`` **don’t** implement on **mutable** references;
- ``Arrays`` (**until** Rust **1.53**) and ``Slices`` implement **only** ``&T`` and ``&mut T``: 
    - ``&[T]``
    - ``&mut [T]``

<br>

## Conventions
- If collection implements ``IntoIterator`` for ``&SomeCollection<T>`` it also **must** implement (by convention) method ``iter()``, it's shorthand for ``(&T).into_iter()``.
- If collection implements ``IntoIterator`` for ``&mut SomeCollection<T>`` it also **must** implement (by convention) method ``iter_mut()``, it's shorthand for ``(&mut T).into_iter()``.

<br>

# ``Iterator`` and ``IntoIterator`` relationships
You **don't need** to implement ``IntoIterator`` if you implement ``Iterator``. There is already trivial implementation for it if you implement ``Iterator``:
```Rust
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
```




# IntoIterator for arrays
**Until** Rust **1.53**, ``only`` **references to arrays** implement ``IntoIterator``.<br>

This means you **can** iterate **over** ``&[1, 2, 3]`` and ``&mut [1, 2, 3]``, but **not** **over** ``[1, 2, 3]`` **directly**.

```Rust
for &e in &[1, 2, 3] {} // Ok
for e in [1, 2, 3] {} // Error
```

**This feature** has been a **long-standing issue**, but the **solution is not as simple as it seems**.<br>
Just adding the trait implementation would break existing code.<br>
It has been suggested many times to "only implement ``IntoIterator`` for arrays in Rust 2021".<br>
However, this is simply **not possible**. You can't have a trait implementation exist in one edition and not in another, since editions can be mixed.<br>

Instead, the trait implementation was added in all editions (starting in Rust **1.53.0**) but with a small hack to avoid breakage **until** Rust **2021**:
- In Rust **2015** and **2018** code, the compiler will still resolve ``array.into_iter()`` to ``(&array).into_iter()`` like before, as if the trait implementation does not exist. 
- This only applies to the ``.into_iter()`` method call syntax. It **doesn't** affect **any other syntax** such as 
``for e in [1, 2, 3]``,  ``iter.zip([1, 2, 3])`` or ``IntoIterator::into_iter([1, 2, 3])``. Those will work in all editions.

<br>

# Loop syntax
``for ... in ...`` syntax is just a syntactic sugar for an ``IntoIterator::into_iter()`` invocation, followed by repeated calling of ``Iterator::next()``.<br>
Type of ``Iterator`` returned by ``into_iter`` is **determined by context**.<br>

Contexts:
- The call ``(T).into_iter()`` returns an ``Iterator`` over ``T``;
- The call ``(&T).into_iter()`` returns an ``Iterator`` over ``&T``;
- The call ``(&mut T).into_iter()`` returns an ``Iterator`` over ``&mut T``.

<br>

<table>
    <tr>
        <th>Context</th>
        <th></th>
        <th>Real call</th>
        <th></th>
        <th>Real loop</th>
    </tr>
<tr></tr>
<tr>
<td>

```Rust
for x in v {
  // body
}
```

</td>


<td>

**=>**

</td>
<td>

```Rust
let mut iter = (v).into_iter();
```

</td>
<td>

**=>**

</td>
<td rowspan="3">

```Rust
loop {
    match iter.next() {
        Some(x) => {
          // body
        },
        None => break,
    }
}
```

</td>
</tr>




<tr></tr>
<tr>
<td>

```Rust
for x in &v {
  // body
}
```

</td>


<td>

**=>**

</td>
<td>

```Rust
let mut iter = (&v).into_iter();
```

</td>
<td>

**=>**

</td>

</tr>


<tr></tr>
<tr>
<td>

```Rust
for x in &mut v {
  // body
}
```

</td>


<td>

**=>**

</td>
<td>

```Rust
let mut iter = (&mut v).into_iter();
```

</td>
<td>

**=>**

</td>

</tr>

</table>

<br>

# Example of implementing ``IntoIterator``
```Rust
// A sample collection, that's just a wrapper over Vec<T>
#[derive(Debug)]
struct MyCollection(Vec<i32>);

// Let's give it some methods so we can create one and add things
// to it.
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Now we can make a new collection...
let mut c = MyCollection::new();

// ... add some stuff to it ...
c.add(0);
c.add(1);
c.add(2);

// ... and then turn it into an Iterator:
for (i, n) in c.into_iter().enumerate() {
    assert_eq!(i as i32, n);
}
```

<br>

# Method drain()
``into_iter()`` **consumes** the **collection** **itself**, ``drain()`` only **consumes** the **values** in the collection.<br>

Therefore ``drain()`` allows draining of only a **part of the collection**.<br>

So use ``into_iter()`` if you want to **consume** the entire collection, and use ``drain()`` if you only want to consume part of the collection or if you want to reuse the emptied collection later.

