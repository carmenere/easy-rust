**By default**, Rust's standard library **sorts** strings in **lexicographical order** which is based on their ASCII/Unicode **code points**.<br>
The `'a' < 'z'` is **true** because *code point* of `'a'` is **less** than *code point* of `'z'`, i.e. they follow **lexicographical order**.<br>
**Lowercase** *letters* typically have **greater** *code points* than other characters, but for for *letters of the same case* `a-z` or `A-Z`, they are ordered **sequentially**.<br>
So, **uppercase** *letters* go **before lowercase** *letters*.<br>

<br>

**Example**:
```rust
fn main() {
    let mut v = vec![
        "1000  ",
        "1000  1",
        "1000  2",
        "1000  a",
        "1000  b",

        "  1.txt",
        "  10.txt",
        "  1000",
        "  1000-",
        "  1000-1",
        "  1000-2",
        "  1000-a",
        "  1000-b",
        "  1000.",
        "  1000.txt",
        "  10000",
        "  1000a",
        "  1000b",

        "7z",
        "Zw",
        "zw",
        "M",
        "F",
        "f",
        "m",
        "Abc",
        "abc",
        "1.txt",
        "2.txt",
        "1s.txt",
        "2s.txt",
        "10.txt",

        "1000",
        "1000-a",
        "1000-b",
        "1000a",
        "1000b",
        "1000.",
        "1000.txt",
        "1000-1",
        "1000-2",
        "1000-",
        
        "10000",
    ];

    println!("{:#?}", {v.sort(); v});
    assert!(' ' < '-'); // ' ' appears before '-'
    assert!('-' < '.'); // '-' appears before '.'
    assert!('.' < '0'); // '.' appears before '0'
    assert!('A' < 'a'); // 'A' appears before 'a'
    assert!('F' < 'M'); // 'F' appears before 'M'
    assert!('f' > 'M'); // 'f' appears after 'M'
    assert!('A' < 'Z'); // 'A' appears before 'Z'
    assert!('a' > 'Z'); // 'a' appears after 'Z'
}
```
**Output**:
```bash
[
    "  1.txt",
    "  10.txt",
    "  1000",
    "  1000-",
    "  1000-1",
    "  1000-2",
    "  1000-a",
    "  1000-b",
    "  1000.",
    "  1000.txt",
    "  10000",
    "  1000a",
    "  1000b",
    "1.txt",
    "10.txt",
    "1000",
    "1000  ",
    "1000  1",
    "1000  2",
    "1000  a",
    "1000  b",
    "1000-",
    "1000-1",
    "1000-2",
    "1000-a",
    "1000-b",
    "1000.",
    "1000.txt",
    "10000",
    "1000a",
    "1000b",
    "1s.txt",
    "2.txt",
    "2s.txt",
    "7z",
    "Abc",
    "F",
    "M",
    "Zw",
    "abc",
    "f",
    "m",
    "zw",
]
```