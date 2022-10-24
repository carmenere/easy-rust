# Iterator Adapters
``Iterator`` trait provides **adapter methods**, or simply **adapters**.<br>

Calling **adapter** on an **iterator** returns a **new iterator** that **yields its own items** from the first iterator.<br>

Every **adapter** takes **iterator** **implicitly**, because of ``self`` argument.<br>

In a **chain of adapters**, the only way to get a result is to call ``next()`` or ``collect()`` on the **final iterator**.<br>

<br>

## ``map()``
```Rust
fn map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}
```

The ``map()`` method takes **closure** ``f`` and returns an **iterator** ``Map``.<br>

``f`` properties:
- it takes an ``Self::Item`` and returns the type ``B``.

``Map`` properties:
- it **yields** items of the type ``B``;
- it also implements ``DoubleEndedIterator``, meaning that you can also **traverse** returned ``Map`` **backwards**.<br>

<br>

#### Example
```Rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

<br>

## ``rev()``
```Rust
fn rev(self) -> Rev<Self>
where
    Self: Sized + DoubleEndedIterator,
{
    Rev::new(self)
}
```

The ``rev()`` method returns an **iterator** ``Rev``.<br>

``Rev`` properties:
- it **yields** items of the type ``Self::Item``, 
- it **reverses** an iterator’s direction. 

<br>

## ``filter()``
```Rust
fn filter<P>(self, predicate: P) -> Filter<Self, P>
where
    Self: Sized,
    P: FnMut(&Self::Item) -> bool,
{
    Filter::new(self, predicate)
}
```

The method ``filter()`` takes a closure ``predicate`` and returns **iterator** ``Filter``.<br>

``predicate`` properties:
- it takes an ``&Self::Item`` and returns a ``bool``.

``Filter`` properties:
- it **yields** only items of type ``Self::Item`` for which predicate predicate returns ``true``.

<br>

## ``flatten()``
```Rust
fn flatten(self) -> Flatten<Self>
where
    Self: Sized,
    Self::Item: IntoIterator,
{
    Flatten::new(self)
}
```

The ``flatten()`` method requires ``Self::Item`` to be **iterable**.<br>

The ``flatten()`` method concatenates **iterator of iterables** into a **single collection of elements** and returns an **iterator** ``Flatten`` **over** the concatenated single collection of elements.<br>

``Flatten`` properties:
- it yields ``Self::Item::Item``, where ``Self::Item`` is some collection of elements of type ``Item``.

#### Example
```Rust
let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
```

<br>

## ``filter_map()``
```Rust
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> Option<B>,
{
    FilterMap::new(self, f)
}
```

Semantic of ``filter_map(f)`` is equivalent to: ``map(f).filter()``.<br>

The ``filter_map()`` method takes a closure ``f`` and returns iterator ``FilterMap``.<br>

``f`` properties:
- it takes an ``Self::Item`` and returns a ``Option<B>``.

``FilterMap`` properties:
- it **yields** only items of type ``B`` for which closure ``f`` returns ``Some(B)``.

<br>

## ``flat_map()``
```Rust
fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
where
    Self: Sized,
    U: IntoIterator,
    F: FnMut(Self::Item) -> U,
{
    FlatMap::new(self, f)
}
```

Semantic of ``flat_map(f)`` is equivalent to: ``map(f).flatten()``.<br>

The ``flat_map()`` method takes a **closure** ``f`` and returns **iterator** ``FlatMap``.<br>

``f`` properties:
- it takes an ``Self::Item`` and returns a ``U``.
- it requires ``U`` to be **iterable**.

``FlatMap`` properties:
- it **yields** only items of type ``U::Item``.

<br>

#### Example
```Rust
let words = ["alpha", "beta", "gamma"];

// chars() returns an iterator
let merged: String = words.iter()
                          .flat_map(|s| s.chars())
                          .collect();

assert_eq!(merged, "alphabetagamma");
```

<br>

## ``fuse()``
Once an ``Iterator`` has returned ``None``, the **trait doesn’t specify** how it ought to behave if you call its ``next()`` method again.<br>
Most **iterators** just return ``None`` **again**, but not all. If your code counts on that behavior, you may be in for a surprise.<br>

The ``fuse()`` adapter takes any ``Iterator`` and produces new ``Iterator`` that will definitely continue to return ``None`` once it has done so the first time.<br>

```Rust
// an iterator which alternates between Some and None
struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let val = self.state;
        self.state = self.state + 1;

        // if it's even, Some(i32), else None
        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}

let mut iter = Alternate { state: 0 };

// we can see our iterator going back and forth
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);

// however, once we fuse it...
let mut iter = iter.fuse();

assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), None);

// it will always return `None` after the first time.
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
```

<br>

## ``chain()``
```Rust
fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
where
    Self: Sized,
    U: IntoIterator<Item = Self::Item>,
{
    Chain::new(self, other.into_iter())
}
```

This adapter appends **iterable** (``other``) to **iterator** (``self``) and concatenates it to single **iterator** ``Self::Item``.<br>

The method ``chain()`` takes a **iterable** ``other`` and returns **iterator** ``Chain``.<br>

``other`` properties:
- it must be **iterable**, i.e., it must implement ``IntoIterator<Item = Self::Item>``.

``Chain`` properties:
- it **yields** items of type ``Self::Item``.

#### Example
```Rust
let a1 = [1, 2, 3];
let a2 = [4, 5, 6];

let mut iter = a1.iter().chain(a2.iter());

// OR

let mut iter = a1.iter().chain(a2);

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), Some(&5));
assert_eq!(iter.next(), Some(&6));
assert_eq!(iter.next(), None);
```

<br>

## ``enumerate()``
```Rust
fn enumerate(self) -> Enumerate<Self>
where
    Self: Sized,
{
    Enumerate::new(self)
}
```

The method ``enumerate()`` returns **iterator** ``Enumerate``.<br>

``Enumerate`` properties:
- it **yields** tuple ``(usize, Self::Item)``, where element of ``usize`` type contains the **index** of the **value** of ``Self::Item`` type.

<br>

## ``zip()``
```Rust
fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
where
    Self: Sized,
    U: IntoIterator,
{
    Zip::new(self, other.into_iter())
}
```

The ``zip()`` method returns iterator ``Zip``.<br>

``Zip`` properties:
- it yields tuple ``(Self::Item, other::Item)``.

The ``zip()`` method uses **shortest semantics**: the **result** will have **length** of the **shortest iterable**.<br>
For **longest semantics** use the ``zip_longest()`` in ``itertools`` crate.<br>

#### Example for ``zip()``
```Rust
fn main() {
    let a1 = [1, 2, 3];
    let a2 = ['a'; 10];
    
    let mut iter = a1.iter().zip(a2.iter());

    println!("{:?}", iter.collect::<Vec<_>>());
}
````

**Output**:
```bash
[(1, 'a'), (2, 'a'), (3, 'a')]
```

<br>

#### Example for ``zip_longest()``:
```Rust
use itertools::{
    Itertools,
    EitherOrBoth::*,
};

fn main() {
    let num1 = vec![1, 2];
    let num2 = vec![3];

    for pair in num1.iter().zip_longest(num2.iter()) {
        match pair {
            Both(l, r) => println!("({}, {})", l, r),
            Left(l) => println!("({}, 0)", l),
            Right(r) => println!("(0, {})", r),
        }
    }
}
```

**Output**:
```Rust
(1, 3)
(2, 0)
```

<br>

## ``by_ref()``
Calling **adapter** on an **iterator** *transfer ownership* of the **underlying iterator** (because of ``self`` argument).<br>

The ``by_ref()`` adapter returns a **mutable reference** to the **iterator**:
```Rust
fn by_ref(&mut self) -> &mut Self
where
    Self: Sized,
{
    self
}
```

<br>

The **std** includes following implementation:
```Rust
impl<I: Iterator + ?Sized> Iterator for &mut I {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }

    ...

}
```
 
When you call **adapter** on a **mutable reference** to an **iterator**, this **adapter** takes ownership of the reference, not the **iterator** itself.<br>
It is just a **borrow** that ends when the adapter goes out of scope and **original collection is valid**.<br>

<br>

## ``cloned()``
The ``cloned()`` adapter takes an **iterator** that produces references and returns an **iterator** that produces values cloned from those references.<br>

Semantic of ``some_iter.cloned()`` is equivalent to: ``some_iter.map(|item| item.clone())``.<br>

<br>

## ``copied()``
The ``copied()`` adapter takes an iterator that produces references and returns an iterator that produces values copied from dereferenced values.<br>

The ``copied()`` method requires that the referent type must be **Copy type**.<br>

Semantic of ``some_iter.copied()`` is equivalent to: ``some_iter.map(|item| *item)``.<br>

<br>

## ``take()``
Creates an **iterator** that **yields** the first ``n`` elements, or fewer if the underlying iterator ends sooner.<br>

```Rust
let a = [1, 2, 3];

let mut iter = a.iter().take(2);

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);
```

<br>

## ``take_while()``
The ``take_while()`` method takes a **closure** as an argument, and **calls** this **closure** **on each element** of the **iterator**, and **yields** elements while closure returns ``true``.<br>

After ``false`` is returned, ``take_while()``’s job is over, and the rest of the elements are **ignored**.<br>

```Rust
let a = [-1, 0, 1];

let mut iter = a.iter().take_while(|x| **x < 0);

assert_eq!(iter.next(), Some(&-1));
assert_eq!(iter.next(), None);
```

<br>

## ``skip()``
Creates an **iterator** that **skips** the first ``n`` elements.<br>

```Rust
let a = [1, 2, 3];

let mut iter = a.iter().skip(2);

assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);
```

<br>

## ``skip_while()``
The ``skip_while()`` method takes a **closure** as an argument, and calls this **closure on each element** of the **iterator**, and **skip** elements while closure returns ``true``.<br>

After first ``false`` is returned it **yields** elements even ``true`` will be return **further**.<br>

```Rust
fn main() {
    let a = [-55, -1, 0, 1, -2];

    let r = a.iter().skip_while(|x| **x < 0).collect::<Vec<_>>();
    println!("{:?}", r);
}
```

Output:
```bash
[0, 1, -2]                                                                                                                                                                                      
```

<br>

## ``fold()``
```Rust
fn fold<B, F>(mut self, init: B, mut f: F) -> B
where
    Self: Sized,
    F: FnMut(B, Self::Item) -> B,
{
    let mut accum = init;
    while let Some(x) = self.next() {
        accum = f(accum, x);
    }
    accum
}
```

This operation is sometimes called **reduce**.<br>

The ``fold()`` method takes two arguments: an **initial value**, and a **closure** with **two arguments**: an **accumulator**, and an **element**.<br>

The **closure** returns the **value** that the **accumulator** should have for the **next iteration**.<br>

The **initial value** is the value the **accumulator** will have on the **first call**.<br>

After applying this closure to every element of the iterator, ``fold()`` returns the **accumulator**.<br>

Basic usage:
```Rust
let a = [1, 2, 3];

// the sum of all of the elements of the array
let sum = a.iter().fold(0, |acc, x| acc + x);

assert_eq!(sum, 6);
```

<br>

Let’s walk through **each step** of the iteration here:
```bash
Element	acc	x	Result
	0		
1	0	1	1
2	1	2	3
3	3	3	6
```
