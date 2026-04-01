# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [`std::collections`](#stdcollections)
  - [HashMap](#hashmap)
    - [The .entry() api](#the-entry-api)
  - [BTreeMap](#btreemap)
  - [HashSet and BTreeSet](#hashset-and-btreeset)
  - [BinaryHeap](#binaryheap)
  - [VecDeque](#vecdeque)
<!-- TOC -->

<br>

# `std::collections`
All **collections** are contained in the `std::collections` module in the *standard library*.<br>

- `HashMap`
  - it is a *collection* made out of *keys* and *values*;
  - `HashMap` is an **unordered** *collection*, i.e. it **doesn't** order its *keys*;
- `BTreeMap`
  - if you want the **keys to be ordered** use `BTreeMap`;
  - `BTreeMap` is like an **ordered** `HashMap`, i.e. it **orders** its *keys*;
- `HashSet`
  - it is like a `HashMap` but **without** *values*;
  - it is implemented as a `HashMap` where the *value* is `()`;
  - `HashSet` is an **unordered** *collection*, i.e. it **doesn't** order its *keys*;
- `BTreeSet`
  - if you want the **keys to be ordered** use `BTreeSet`;
  - `BTreeSet` is like an **ordered** `HashSet`, i.e. it **orders** its *keys*;
- `BinaryHeap`
  - it is a **priority queue**;
  - a `BinaryHeap` always has the **largest value** *at the front*, everything else is **unsorted**;
- `VecDeque` (pronounced /vec-deck/)
  - it is like a `Vec` that is **optimized for** *popping items both off* the **front** and the **back**;

<br>

## HashMap
```rust
use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

fn main() {
    let mut foo = City {
        name: "foo".to_string(),
        population: HashMap::new(),
    };
    foo.population.insert(2020, 437_619);
    foo.population.insert(1372, 3_250);
    foo.population.insert(1851, 24_000);
    for (year, population) in foo.population {
        println!("In {year}, Foo had a population of {population}.");
    }
}
```

<br>

The simplest way to get a value in a `HashMap` is by putting the key in `[]` square brackets, **similar** to a `Vec`.<br>
But be careful because the program will **crash** if there is **no** key, just like when indexing a `Vec`.<br>
If you are not sure there will be a key, you can use `.get()`, which returns a **reference** *to the value corresponding to the key* (`Option<&V>`):
- if *key* **exists**, it will be `Some(&value)`
- if **not**, you will get `None`;

<br>

If a `HashMap` already has a *key* when you try to put it in, using `.insert()` will **overwrite** its *value*.<br>
Also the `.insert()` returns an `Option` that holds the **old** value **if** the value was overwritten.<br>

```rust
use std::collections::HashMap;
fn main() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "foo");
    book_hashmap.insert(1, "bar");
    book_hashmap.insert(1, "hello");
    println!("{:?}", book_hashmap.get(&1));
}
```
**Output**:
```bash
Some("hello")
```

<br>

In the next sample, we will have a `Vec` that will **hold any old values** that have been returned by the `.insert()`:
```rust
use std::collections::HashMap;
fn main() {
    let mut book_hashmap = HashMap::new();
    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "foo"),
        (1, "bar"),
        (1, "hello"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting `{old_value}` with `{value}`!");
            old_hashmap_values.push(old_value);
        }
    }
    println!("All old values: {old_hashmap_values:?}");
}
```

<br>

To **prevent** *overwriting* we **must check** *whether an entry exists*:
```rust
use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();
    // book_hashmap.insert(1, "foo");
    let key = 1;

    // variant 1
    match book_hashmap.get(&key) {
        Some(val) => (),
        None => {book_hashmap.insert(1, "bar");},
    }

    // variant 2
    if let None = book_hashmap.get(&key) {
        book_hashmap.insert(1, "hello");
    }

    println!("{:?}", book_hashmap.get(&1));
}
```

<br>

### The .entry() api
With `.entry()`, you can try to make an entry and then another method like `.or_insert()` to insert a **default** *value* if there is **no** *key*.<br>
The `.or_insert()` method either returns a **mutable reference** to the *existing value*, or it *inserts the default value* and then returns a **mutable reference** to it.<br>
That means you can use `let` to **assign** the *mutable reference* to a *variable name* and **change** the *value* in the `HashMap`.<br>

<br>

**Count the words example**:
```rust
use std::collections::HashMap;

fn main() {
    let words = vec![
        "foo",
        "bar",
        "hello",
        "hello",
    ];
    let mut words_map = HashMap::new();
    for word in words {
        let counter = words_map.entry(word).or_insert(0_u32);
        *counter += 1;
    }
    for (word, counter) in words_map {
        println!("{word}: {counter}");
    }
}
```

<br>

**Join answers per gender and print all answers per gender**:
```rust
use std::collections::HashMap;

fn main() {
    let data = vec![
        ("male", 1),
        ("female", 2),
        ("male", 3),
        ("female", 3),
        ("male", 5),
        ("female", 8),
    ];
    let mut survey = HashMap::new();
    for item in data {
        let counter = survey.entry(item.0).or_insert(Vec::with_capacity(8)).push(item.1);
    }
    for (gender, answers) in survey {
        println!("{gender}: {answers:?}");
    }
}
```
**Output**:
```rust
female: [2, 3, 8]
male: [1, 3, 5]
```

<br>

The `.entry()` only takes a *key* and then returns an *enum* called `Entry`, `K` means *key*, and `V` means *value*:
```rust
pub fn entry(&mut self, key: K) -> Entry<K, V>

enum Entry<K, V> {
    Occupied(OccupiedEntry<K, V>),
    Vacant(VacantEntry<K, V>),
}
```

<br>

The *next method*, `.or_insert()`, is a method on the `Entry` *enum*: it looks at the *enum* and decides what to do:
```rust
fn or_insert(self, default: V) -> &mut V {
    match self {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => entry.insert(default),
    }
}
```

<br>

## BTreeMap
We can quickly change our `HashMap` to a `BTreeMap`, because their **methods** and **signatures** are very **similar**:
```rust
use std::collections::BTreeMap;

struct City {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut foo = City {
        name: "foo".to_string(),
        population: BTreeMap::new(),
    };
    foo.population.insert(2020, 437_619);
    foo.population.insert(1372, 3_250);
    foo.population.insert(1851, 24_000);
    for (year, population) in foo.population {
        println!("In {year}, Foo had a population of {population}.");
    }
}
```

<br>

## HashSet and BTreeSet
```rust
use std::collections::HashSet;

const TOTAL: usize = 50;

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21, 20, 38, 16, 48, 39,
        31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9, 46, 33,
    ];
    println!("How many numbers in the Vec? {}", many_numbers.len());
    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!("There are {hashset_length} unique numbers, so we are missing {}.", TOTAL - hashset_length);

    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            println!("{number} is missing");
        }
    }
}
```

<br>

## BinaryHeap
**Tuples** are compared **element by element**, **from left to right**. The comparison **stops** at the **first** *differing element*:
```rust
fn main() {
    let t1 = (1, "x");
    let t2 = (2, "a");
    let t3 = (2, "a");
    let t4 = (1, "z");

    assert_eq!(t2, t3);
    assert!(t4 < t2); // the first element in t4 is LESS THAN in t1: 1 < 2
    assert!(t4 > t1); // the first elements are equal: 1 = 1 and the second element in t4 is GREATER THAN in t1: 'z' > 'x' 
}
```

<br>

A good way to use a `BinaryHeap` is for jobs/tasks.<br>
By default, `BinaryHeap` is a **max-heap**, meaning the element with the **highest priority** (**greatest value**) is popped first.<br>
For **tuples**, *highest priority* means the one that is considered **greatest** based on sequential comparison of its elements.<br>

<br>

The `.pop()` removes item with the **greater** value and returns `Option<Item>`:
```rust
use std::{collections::BinaryHeap};

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "foo"));
    jobs.push((80, "bar"));
    jobs.push((5, "xyz"));
    jobs.push((70, "abc"));
    jobs.push((30, "qwerty"));

    while let Some((priority, payload)) = jobs.pop() {
        println!("Job with priority {}, payload: {}", priority, payload);
    }
}
```

<br>

The `.ppeekop()` gets item with the **greater** value and returns `Option<Item>`:
```rust
use std::{collections::BinaryHeap};

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "foo"));
    jobs.push((80, "bar"));
    jobs.push((5, "xyz"));
    jobs.push((70, "abc"));
    jobs.push((30, "qwerty"));

    if let Some((priority, payload)) = jobs.peek() {
        println!("Job with priority {}, payload: {}", priority, payload);
    }
}
```

<br>

## VecDeque
The `Vec::remove(index)` shifts over the remaining elements one step left and it has a worst-case performance of **O(n)**.<br>
The `VecDeque::remove(index)` is much **faster** than *Vec's* `.remove(index)` and it is safe, it returns `None` if `index` is **out of bounds**.<br>
The `VecDeque::pop_front()` is much **faster** than *Vec's* `.remove(0)`.<br>

**Example**:
```rust
use std::{collections::VecDeque, time::{Duration, Instant}};

fn main() {
    let mut my_vec = Vec::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.remove(0);
        }
    });
    println!("Overall time for Vec::remove(): {:#?}", r.0);
    let mut my_vec = Vec::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.pop();
        }
    });
    println!("Overall time for Vec::pop(): {:#?}", r.0);
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.remove(100000);
        }
    });
    println!("Overall time for VecDeque::remove(): {:#?}", r.0);
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.pop_front();
        }
    });
    println!("Overall time for VecDeque::pop_front(): {:#?}", r.0);
}

pub fn time<F, T>(f: F) -> (Duration, T)
where F: FnOnce() -> T 
{
  let now = Instant::now();
  let res = f();
  let elapsed = now.elapsed();
  (elapsed, res)
}
```
**Output**:
```bash
Vec::remove(0): 14.531206s
Vec::pop(): 288.75µs
VecDeque::remove(0): 937.875µs
VecDeque::pop_front(): 749.583µs
```
