# Slice type
**Slices** are used to **reference** to **contiguous sequence of elements** *in collection*.<br>
Internally, the **slice** is **fat pointer** that stores the **starting index** and the **length** of the *slice*.<br>

<br>

## Slice notation
Slice notation: ``& <collection>[starting_index..ending_index]``, where:
- ``..`` is **range operator**;
- ``<collection>`` name of some collection;
- ``starting_index`` is the **index** of the **first element** in the slice;
- ``ending_index`` is the **index** of the **last element** in the slice **+ 1**;

Note that **length of the slice** =  ``ending_index`` – ``starting_index``.<br>

Notes:
- if *slice* **includes** the **first element**, you can omit the ``starting_index``:
```Rust
let slice = &s[..2];
let slice = &s[0..2];
```
- if *slice* **includes** the **last element**, you can omit ``ending_index``:
```Rust
let slice = &s[3..];
let len = s.len();
let slice = &s[3..len];
```

<br>

## Slice rules
Slices **don’t** allow to **change state** of **collection** as they are **references**.<br>

How slices prevent from bugs, consider following example:
```Rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // word is now totally invalid!
}
```

This code compiles without any errors.<br>
But, because ``word`` **isn’t connected** to the **state** of ``s`` at all, ``word`` still contains the value 5 after calling ``s.clear()``.<br>
We could use that value 5 with the variable ``s`` to try to extract the first word out, but this would be a bug because the contents of ``s`` have changed since we saved 5 in ``word``.<br>

<br>

## String slice
**String slice** is a reference to part of a ``String``.<br>

Example:
```Rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

Range indices for ``String`` slice must occur at valid UTF-8 character boundaries.<br>
If you attempt to create a ``String`` slice in the middle of a multibyte character, your program will exit with an error.

## String slice as type of function parameter
Consider signature: ``fn f1(s: &String) -> &str { }``. This function can only receive ``&String`` type.<br>
Let’s rewrite it to: ``fn f2(s: &str) -> &str { }``. This function can receive both ``&String`` and ``&str`` types.<br>

Because **string literals** are **string slices** already, it is possible to pass value of ``&str`` (**string literal**) to ``f2`` directly.

<br>

## Get last element of dynamically growing collection
There are 2 ways to access to the last lement of collection:
1. Calling ``last`` on **slice**:
```Rust
fn main() {
    let mut v = vec![1,2,3,4,5];
    v.push(6);
    println!("{:?}", &v[..].last());
    v.push(7);
    println!("{:?}", &v[..].last());
}
```
2. Calling ``last`` or ``last_mut`` on ``Vec`` directly:
```Rust
fn main() {
    let v = &mut vec![0, 1, 2];

    if let Some(last) = v.last_mut() {
        *last = 10;
    }
    println!("{:?}", v.last_mut());
}
```

<br>

Notes:
 - ``last_mut`` method returns a **mutable pointer** to the **last item** in the collection;
 - signature of ``last_mut`` method: ``pub fn last_mut(&mut self) -> Option<&mut T>``.
