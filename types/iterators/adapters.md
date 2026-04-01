# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Adapters](#adapters)
- [Method `collect()`](#method-collect)
  - [`Turbofish`](#turbofish)
- [Method `drain()`](#method-drain)
- [Iterator methods chaining](#iterator-methods-chaining)
- [`.for_each()`](#for_each)
- [`.skip()` and `.take()`](#skip-and-take)
- [`.enumerate()`](#enumerate)
- [`.filter_map()` and `.ok()`](#filter_map-and-ok)
- [`.ok_or(error)`](#ok_orerror)
- [`.ok_or_else(error)`](#ok_or_elseerror)
- [`.and_then()` and `.ok()`](#and_then-and-ok)
- [`.and()`](#and)
- [`.zip()`](#zip)
  - [`.zip()` and `.cycle()`](#zip-and-cycle)
  - [`.zip()` and `.collect()` into HashMap](#zip-and-collect-into-hashmap)
- [`.fold()` numbers](#fold-numbers)
- [`.fold()` events](#fold-events)
- [`.by_ref()`](#by_ref)
- [`.chunks()` and `.windows()`](#chunks-and-windows)
- [`.match_indices()`](#match_indices)
- [`.peekable()` and `.peek()`](#peekable-and-peek)
- [`flatten()`](#flatten)
  - [Flattening a single nested `Result` or `Option`](#flattening-a-single-nested-result-or-option)
  - [Flattening an `Iterator` that yields `Result` or `Option`](#flattening-an-iterator-that-yields-result-or-option)
  - [Collecting a `Vec<Result<T, E>>` into a `Result<Vec<T>, E>`](#collecting-a-vecresultt-e-into-a-resultvect-e)
- [`.inspect()`](#inspect)
<!-- TOC -->

<br>

# Adapters
The `Iterator` trait provides **adapter methods**, or simply **adapters**.<br>
Calling **adapter** on an **iterator** returns a **new iterator** that **yields its own items** from the first iterator.<br>
Every **adapter** takes **iterator** **implicitly**, because of `self` argument.<br>
In a **chain of adapters**, the only way to get a result is to call `next()` or `collect()` on the **final iterator**.<br>

<br>

**Adapters**:

| Adapter                                                                                    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|:-------------------------------------------------------------------------------------------|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [**all(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)            | Takes **predicate** `f` and returns `true` if `f` returns `true` for **all** elements. On **empty** returns `true`.                                                                                                                                                                                                                                                                                                                                                                  |
| [**any(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)            | Takes **predicate** `f` and returns `true` if `f` returns `true` **at least for 1** element. On **empty** returns `false`.                                                                                                                                                                                                                                                                                                                                                           |
| [**by_ref**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref)         | Returns a **mutable reference** to the **iterator**.<br>Calling **adapter** on an **iterator** *transfer ownership* of the **underlying iterator** (because of `self` argument).<br>When you call **adapter** on a **mutable reference** to an **iterator**, this **adapter** takes ownership of the reference, not the **iterator** itself.<br>It is just a **borrow** that ends when the adapter goes out of scope and **original collection is valid**.<br>                       |
| [**chain(o)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)        | **Connects 2 independent iterators** together, in a chain.<br>Argument `o` must implement `IntoIterator<Item = Self::Item>`.                                                                                                                                                                                                                                                                                                                                                         |
| [**cloned**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)         | Takes an **iterator** that produces **references** and returns an **iterator** that produces **values** cloned from those references.<br>Semantic of `some_iter.cloned()` is equivalent to: `some_iter.map(\|item\| item.clone())`.                                                                                                                                                                                                                                                  |
| [**collect**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)       |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**copied**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied)         | Takes an **iterator** that produces **references** and returns an **iterator** that produces **values** copied from dereferenced values.<br>Semantic of `some_iter.copied()` is equivalent to: `some_iter.map(\|item\| *item)`.                                                                                                                                                                                                                                                      |
| [**count**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**cycle**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle)           | Instead of stopping at `None`, the iterator will starting again from the beginig.                                                                                                                                                                                                                                                                                                                                                                                                    |
| [**filter_map**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) | Semantic of `filter_map(f)` is equivalent to: `map(f).filter()`.<br>It **yields** only items for which closure `f` returns `Some(B)`.                                                                                                                                                                                                                                                                                                                                                |
| [**filter(p)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)      | Applies **predicate** `p` to every element in collection and **yields** only the elements for which the **predicate** returns `true`.                                                                                                                                                                                                                                                                                                                                                |
| [**find**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)             |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**fold(init, f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)    | Recursively applies **closure** `f` to every element in collection and returns the **result value** of **accumulator**: `acc`. This operation is sometimes called **reduce**.<br>**Closure** `f` has 2 args: `acc` and element, result of closure is passed to `acc` in **next** *iteration*.<br>Argument `init` is the value that assigned to `acc` **before** *first call* of `f`.<br>Note, `init`, `acc` and **result value** must be of the **same type**.                       |
| [**for_each**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**last**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last)             |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**map(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)            | Applies **closure** `f` to every element in collection. **Closure** `f` takes an `Self::Item` as input and returns the result of another type `B`.                                                                                                                                                                                                                                                                                                                                   |
| [**max**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max)               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**min**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min)               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**nth**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**partition**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition)   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**product**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)       |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**reduce(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce)      | Similar to `fold()`, but instead explicit `init` value uses **first element** of collection as `init`.                                                                                                                                                                                                                                                                                                                                                                               |
| [**rev**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev)               | **Reverses** an iterator’s direction.                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [**scan**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan)             |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**skip**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip)             | Returns an **iterator** that **skips** the first `n` elements.<br>                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [**step_by**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by)       |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**sum**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**take**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take)             | Returns an **iterator** that **yields** the first `n` elements, or fewer if the underlying iterator ends sooner.                                                                                                                                                                                                                                                                                                                                                                     |
| [**unzip**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip)           |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [**zip(i)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)            | Takes **2 iterators**: `self` and `i` and **returns 1 iterator** *of 2-tuples* which yields tuple `(Self::Item, other::Item)`.<br>The `zip()` method uses **shortest semantics**: the **result** will have **length** of the **shortest iterable**.<br>For **longest semantics** use the `zip_longest()` in `itertools` crate.                                                                                                                                                       |
| [**flatten**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)       | Concatenates **iterator of iterables** into a **single collection of elements** and returns an **iterator** `Flatten` **over** the concatenated single collection of elements.<br>The `flatten()` method requires `Self::Item` to be **iterable**. It yields `Self::Item::Item`, where `Self::Item` is some collection of elements of type `Item`.                                                                                                                                   |
| [**fuse**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse)             | What returns `Iterator` after it has already returned `None`?.<br>Most **iterators** just return `None` **again**, but **not all**.<br>The `fuse()` takes any `Iterator` and produces new `Iterator` that will definitely continue to return `None` once it has done so the first time.                                                                                                                                                                                              |
| [**enumerate**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)   | **Yields** tuple `(usize, Self::Item)`, where element of `usize` type contains the **index** of the **value** of `Self::Item` type.                                                                                                                                                                                                                                                                                                                                                  |

<br>

# Method `collect()`
```rust
pub trait Iterator {
    type Item;
    // ...
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where Self: Sized,
    {
        <B as FromIterator>::from_iter(self)
    }
}
```

**Notes**:
- method `.collect::<U<_>>()` transforms an **iterator** into a **collection** of type `U`, type of _collection's_ **item** is taken from `Self::Item` of iterator;
- in method `.collect::<U<_>>()` the **collection** `U` **must implement** `FromIterator` trait;
- method `.collect()` can infer type of item of collection, so you can use `_`, e.g. instead `.collect::<U<u8>>()` use `.collect::<Vec<_>>()`;
- method `.collect()` can also create instances of types that are **not** typical collections, e.g., `.collect()` can return `Result<SomeCollection<U>, E>`;

<br>

Using `T::from_iter()` as a more readable alternative to `.collect::<T<_>>()`, because `.collect::<T<_>>()` is more general and it can return collection of **any** type, while `T::from_iter()` returns collection only of `T` type:
```rust
use std::collections::VecDeque;
let first = (0..10).collect::<VecDeque<i32>>();
let second = VecDeque::from_iter(0..10);

assert_eq!(first, second);
```

<br>

## `Turbofish`
Because `collect()` is so general, it can cause **problems** with **type inference**.<br>
Internally, `collect()` just uses `FromIterator`, but it also **infers** the **type** of the **output**.<br>
Sometimes there **isn't enough** information to infer the type, so you may need to **explicitly** specify the type you want.<br>
There is *special syntax* in Rust called **turbofish**: `::<SomeType>`.<br>
Example: `let all_scores = score_table.values().cloned().collect::<Vec<Score>>();`.<br>
**Turbofish** helps the **inference** algorithm to understand type of item of **resulting collection**.<br>

<br>

**Example**:
```rust
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
- by **declaring** the **type** of variable in `let` **binding**:
```rust
let even_numbers: Vec<i32> = ...
```
- by using a **turbofish**:
```rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<i32>>();
```

The `::<Vec<i32>>` part is the **turbofish** and means collect this **iterator** into a `Vec<i32>`.<br>

You can actually replace `i32` with `_` in **turbofish** and let the compiler infer it because it knows the **iterator** yields `i32`:
```rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<_>>();
```

<br>

# Method `drain()`
`into_iter()` **consumes** the **collection** **itself**, `drain()` only **consumes** the **values** in the collection.<br>

Therefore `drain()` allows draining of only a **part of the collection**.<br>

So,
- use `into_iter()` if you want to *consume* the **entire** collection;
- use `drain()` if you only want to *consume* **part** of the collection or if you want to *reuse* the **emptied collection** later;

<br>

# Iterator methods chaining
- `.filter()` it receives **clousre** that returns `bool` (aka **predicate**) and creates iterator that **yields only** the **items** for which the **predicate** returns `true`;
- `.filter_map()` it receives **closure** that returns `Option<T>` and creates iterator that **yields only** the `value`s for which the closure returns `Some(value)` and **filters out** everything that is `None`;
- `.and_then()` this method’s input is an `Option`, and its output is also an `Option`;
  - take **value** if `Some`, do something to the value, and wrap again to `Some`;
- `.and()` matches many `Option`s in the chain
  - if they are **all** `Some`, it will give the **last one**;
  - if **one** of them is a `None`, it will give `None`;
- the `.flatten()` method is a convenient way to **filter out** `None` or `Err` values in an iterator and only return the successful values;
  - `.flatten()` method of `Iterator` requires its items to implement `IntoIterator`;
- `.any(predicate)` returns a `true` if a **predicate** closure returns `true` for **any** of the item;
  - `.any()` only checks **until** it finds the **first** matching item, and then it **stops** — there’s no need to check the rest of the items;
- `.all(predicate)` returns a `true` if a **predicate** closure returns `true` for **all** of the item;
- `.rev()` you can use `.rev()` after `.iter()` to **reverse the iterator**;
- `.find()` returns an `Option` with the **item** inside or `None`;
- `.position()` finds an item that matches condition and returns an `Option` with the **index of item** inside or `None`;
- `.cycle()` creates an iterator that **loops forever**;
- `.zip()` joins two iterators and returns tuple of two values;
- `.fold()` is similar to `.for_each()` except that it returns a **final value** at the end;
  - when using `.fold()`, you first add a **starting value**, then a comma, and then the **closure**;
  - the *closure* has you **two inputs**: the **total so far** and the **next item**;
  - `.fold()` **not only **for adding numbers, it is possible to use **complex structs** in `.fold()`;
- `.cloned()` makes a clone inside the iterator; this **turns** a *reference* **into** a *value*;
- many other **_while** methods:
  - `.take_while(predicate)` takes item **as long as** predicate returns `true`;
  - `.skip_while()` skips item **as long as** predicate returns `true`;
  - `.map_while()` 
- `.sum()` adds everything together
- `.by_ref()` the most of methods take `self` and **consume the whole iterator**, however `.into_iter().by_ref()` takes `&self`;
  - `.by_ref()` allows to avoid error `error[E0382]: use of moved value: `foo``
- `.chunks(N)`
  - `.chunks(0)` will **panic** if you give it **0**;
  - let’s say you have a **vector** `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]`, then `.chunks(3)` will give you **4** slices:
    - `[0, 1, 2]`
    - `[3, 4, 5]`
    - `[6, 7, 8]`
    - `[9]`
- `.windows(N)` let’s say you have a **vector** `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]` then `.windows(3)` will give you **8** slices:
  - `[0, 1, 2]`
  - `[1, 2, 3]`
  - `[2, 3, 4]`
  - `[3, 4, 5]`
  - `[4, 5, 6]`
  - `[5, 6, 7]`
  - `[6, 7, 8]`
  - `[7, 8, 9]`
- `.match_indices()` it is similar to `.enumerate()` because it returns a **tuple** with two items **index** and **matched value**;
  - lets you pull out everything inside a  that **matches your input** and gives you the **index**, too;
- `.peekable()` and `.peek()`
  - `.peekable()` creates a type of iterator called **peekable**, which has the `.peek()` method (**regular** iterators **can’t** use `.peek()`);
  - `.peek()` allows you get the **next** item **without** moving iterator futher, i.e. it returns `Option` like `.next()` except that the iterator **doesn’t move**, so you can use it as many times as you want;
- `.inspect()` is similar to `dbg!`, but it is used in iterators in a similar fashion to `.map()`: it simply gives you the item to look at, which lets you print it or do whatever you want;
- `.collect()` transforms iterator to collection of some type;
- `.skip(N)` skips over `N` items;
- `.take(N)` takes the first `N` items;
- `.map()` lets you do something to every item (including turning it into a different type) and then pass it on to make a new iterator;
- `.for_each()` lets you do something with every item **without** creating a new iterator;
  - it allows **modify items** in the **original** `Vec` **without** nedding to make a **new** `Vec`;
- `.enumerate()` zips items with their indices, in other words it coverts original collection to collection of tuples `(usize, item)`;

<br>

The `.iter()`/`.iter_mut()` plus `.for_each()` is basically a `for` loop.<br>

<br>

# `.for_each()`
In the example below we don’t need to use `.collect()` to create a new `Vec`, because we change items directly in the original collection:
```rust
vector2.iter_mut().for_each(|x| *x +=100);
```
Thus, `vector2` is still there after the iterator is over.

<br>

# `.skip()` and `.take()`
```rust
fn main() {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec.into_iter().skip(2).take(2).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let v = (1..).skip(10).take(5).collect::<Vec<i32>>();
    println!("{v:?}");
}
```
**Output**:
```bash
[2, 3]
[11, 12, 13, 14, 15]
```

<br>

# `.enumerate()`
**Example**:
```rust
fn main() {
    let char_vec = vec!['z', 'y', 'x'];
    char_vec
    .iter()
    .enumerate()
    .for_each(|(index, c)| println!("Index {index} is: {c}"));
}
```
**Output**:
```bash
Index 0 is: z
Index 1 is: y
Index 2 is: x
```

<br>

# `.filter_map()` and `.ok()`
Anything that returns an `Err` becomes `None` after the `.ok()` method and then **gets filtered out** by `.filter_map()`:
```rust
fn main() {
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", successful_numbers);
}
```
**Output**:
```python
[8.9, 8.0, 7.6]
```

<br>

# `.ok_or(error)`
```rust
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Foo", "A"),
        Company::new("Bar", "B"),
        Company::new("Fizzbazz", ""),
    ];

    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();

    for item in results {
        println!("{:?}", item);
    }
}
```
**Output**:
```python
Ok("A")
Ok("B")
Err("No CEO found")
```

<br>

The `.map(|company| company.get_ceo().ok_or("No CEO found"))` means
- if `.get_ceo()` returns a `Some(value)`, put the `value` as is inside `Ok`;
- if `.get_ceo()` returns a `None`, put the `No CEO found` inside `Err`;

<br>

# `.ok_or_else(error)`
```rust
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Foo", "A"),
        Company::new("Bar", "B"),
        Company::new("Fizzbazz", ""),
    ];

    let results: Vec<_> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or_else(|| {
            let err_msg = format!("No CEO found for {}", company.name);
            println!("[[ERROR]]: {}", err_msg);
            err_msg
        }))
        .collect();

    results
        .iter()
        .filter(|i| i.is_ok())
        .for_each(|i| println!("{:?}", i));
}
```
**Output**:
```python
[[ERROR]]: No CEO found for Fizzbazz
Ok("A")
Ok("B")
```

<br>

# `.and_then()` and `.ok()`
```rust
fn main() {
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
        num_array
            .get(index)
            .and_then(|number| number.parse::<u32>().ok())
            .and_then(|number| char::try_from(number).ok())
        );
    }
    println!("{:?}", char_vec);
}
```
**Output**:
```python
[Some('\u{8}'), Some('\t'), None, None, None]
```

<br>

# `.and()`
```rust
fn main() {
    let try_1 = [1,2,3];
    let try_2 = ["a", "b", "c"];
    let try_3 = [111, 222];
    for i in 0..try_1.len() {
        println!("{:?}", try_1.get(i).and(try_2.get(i)).and(try_3.get(i)));
    }
}
```
**Output**:
```bash
Some(111)
Some(222)
None
```

<br>

# `.zip()`
## `.zip()` and `.cycle()`
```rust
fn main() {
    let inf_iter = ["even", "odd"].into_iter().cycle();
    let even_odd: Vec<(i32, &str)> = (0..=5)
        .zip(inf_iter)
        .collect();
    println!("{:?}", even_odd);
}
```
**Output**:
```python
[(0, "even"), (1, "odd"), (2, "even"), (3, "odd"), (4, "even"), (5, "odd")]
```

<br>

This iterator `["even", "odd"].into_iter().cycle()` will return `Some("even")` and `Some("odd")` **forever**. It will never return `None`.<br>
Even though `inf_iter` will never end, the other iterator only runs **six times** and thus the final `Vec` also only has six items.<br>

<br>

## `.zip()` and `.collect()` into HashMap
```rust
use std::collections::HashMap;
fn main() {
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values)
        .collect::<HashMap<_, _>>();
    println!("The value at key 2 is: {}", number_word_hashmap.get(&2).unwrap());
}
```

<br>

You can see that we wrote `<HashMap<_, _>>` because that is enough information for Rust to decide on the type `HashMap<i32, &str>`.<br>
You can write `.collect::<HashMap<i32, &str>>()` if you want.<br>

<br>

# `.fold()` numbers
```rust
fn main() {
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
}
```

<br>

# `.fold()` events
```rust
#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

fn main() {
    let events = [
    "Went to grocery store",
    "Came home",
    "Fed cat",
    "Fed cat again",
    ];

    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![]
    };
    let combined_events =
    events
        .iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
    });
    println!("{combined_events:#?}");
}
```
**Output**:
```python
CombinedEvents {
    num_of_events: 4,
    data: [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ],
}
```

<br>

# `.by_ref()`
```rust
fn main() {
    let mut number_iter = [7, 8, 9, 10].into_iter();
    let first = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let second = number_iter.take(2).collect::<Vec<_>>();
}
```

<br>

# `.chunks()` and `.windows()`
```rust
fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    println!("chunks:");
    for chunk in num_vec.chunks(3) {
        println!("  {:?}", chunk);
    }
    println!("windows:");
    for window in num_vec.windows(3) {
        println!("  {:?}", window);
    }
}
```
**Output**:
```python
chunks:
  [1, 2, 3]
  [4, 5, 6]
  [7]
windows:
  [1, 2, 3]
  [2, 3, 4]
  [3, 4, 5]
  [4, 5, 6]
  [5, 6, 7]
```

<br>

# `.match_indices()`
```rust
fn main() {
    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}' at index {index}");
    }
}
```
**Output**:
```python
ä at 22
ß at 53
'. ' at index 26
'. ' at index 46
```

<br>

# `.peekable()` and `.peek()`
```rust
fn main() {
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();
    for i in 0..3 {
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        number_iter.next();
    }
}
```

<br>

# `flatten()`
The `.flatten()` can be applied to `Result`/`Option` in different ways:
- for **single nested** `Result`/`Option` value;
- for **iterator** of `Result`/`Option` values;
- for **collecting** a `Vec<Result<T, E>>` into a `Result<Vec<T, E>>`;

<br>

## Flattening a single nested `Result` or `Option`
To **flatten** a **single instance** of a nested `Result` or `Option`, you can use the `.flatten()` method.<br>

The `Option<Option<T>>` implements `.flatten()` itself:
```rust
impl<T> Option<Option<T>> {
    pub const fn flatten(self) -> Option<T> {
        match self {
            Some(inner) => inner,
            None => None,
        }
    }
}
```

The `.flatten()` method called on the `Option<Option<T>>` **converts** from `Option<Option<T>>` to `Option<T>`:
```rust
let x: Option<Option<u32>> = Some(Some(6));
assert_eq!(Some(6), x.flatten());

let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
assert_eq!(Some(Some(6)), x.flatten());
assert_eq!(Some(6), x.flatten().flatten());
```

<br>

The `Result<Result<T, E>, E>` implements `.flatten()` itself:
```rust
impl<T, E> Result<Result<T, E>, E> {
    pub const fn flatten(self) -> Result<T, E> {
        match self {
            Ok(inner) => inner,
            Err(e) => Err(e),
        }
    }
}
```

The `.flatten()` method called on the `Result<Result<T, E>, E>` **converts** from `Result<Result<T, E>, E>` to `Result<T, E>`:
```rust
let x: Result<Result<&'static str, u32>, u32> = Ok(Ok("hello"));
assert_eq!(Ok("hello"), x.flatten());
```

<br>

## Flattening an `Iterator` that yields `Result` or `Option`
The `Option` and `Result` both implement `IntoIterator`.<br>

```rust
impl<T> IntoIterator for Option<T>
```

```rust
impl<T, E> IntoIterator for Result<T, E>
```

The `Result<T, E>` and `Option` both can be treated as an `Iterator` over **one** or **zero** items.<br>

`Option` implements `IntoIterator`, which yields:
- **one** item (the `T` **value**) in the case of `Some(T)`;
- **zero** items in the case of `None`;

`Result<T, E>` implements `IntoIterator`, which yields:
- **one** item (the `T` **value**) in the case of `Ok(T)`;
- **zero** items in the case of `Err(E)`;

<br>

**Examples**:
```rust
let x = Some("string");
let v: Vec<&str> = x.into_iter().collect();
assert_eq!(v, ["string"]);

let x = None;
let v: Vec<&str> = x.into_iter().collect();
assert!(v.is_empty());
```

<br>

**Examples**:
```rust
let x: Result<u32, &str> = Ok(5);
let v: Vec<u32> = x.into_iter().collect();
assert_eq!(v, [5]);

let x: Result<u32, &str> = Err("nothing!");
let v: Vec<u32> = x.into_iter().collect();
assert_eq!(v, []);
```

<br>

**Consider example**:
```rust
fn main() {
    let v = ["9", "nine", "ninety-nine", "9.9"];
    for num in v.into_iter().map(|num| num.parse::<f32>()) {
        println!("{num:?}");
    }
}
```
**Output**:
```python
Ok(9.0)
Err(ParseFloatError { kind: Invalid })
Err(ParseFloatError { kind: Invalid })
Ok(9.9)
```
<br>

But if we **don’t care** about the `Err` values, we can add `.flatten()`:
```rust
fn main() {
    let v = ["9", "nine", "ninety-nine", "9.9"];
    for num in v.into_iter().map(|num| num.parse::<f32>()).flatten() {
        println!("{num:?}");
    }
}
```
**Output**:
```python
9.0
9.9
```

<br>

## Collecting a `Vec<Result<T, E>>` into a `Result<Vec<T>, E>`
A common use case is to process an **iterator** of **fallible** operations and collect all **successful** results into a `Vec<T>` or **stops** on the **first** `Err` and **returns it**.<br>

Example:
```rust
fn main() {
    let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let transposed: Result<Vec<i32>, &str> = results.into_iter().collect();
    println!("{:?}", transposed); // Output: Ok([1, 2, 3])

    let results_err: Vec<Result<i32, &str>> = vec![Ok(1), Err("error encountered"), Ok(3)];
    let transposed_err: Result<Vec<i32>, &str> = results_err.into_iter().collect();
    println!("{:?}", transposed_err); // Output: Err("error encountered")
}
```
**Output**:
```python
Ok([1, 2, 3])
Err("error encountered")
```

<br>

# `.inspect()`
```rust
fn main() {
    let a = [1, 4, 2, 3];
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("current item: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("item after filter: {x}"))
        .fold(0, |sum, i| sum + i);
    println!("{sum}");
}
```
**Output**:
```python
current item: 1
current item: 4
item after filter: 4
current item: 2
item after filter: 2
current item: 3
6
```

