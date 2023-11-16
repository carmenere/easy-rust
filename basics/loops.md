# Loops
There are 4 loop types in Rust:
- Iterator loops
- Iterator loops with enumeration
- Predicate loops
- Infinite loops

<br>

## Iterator loops
There is `for` loop in Rust when *number of iterations* in **known**.

### Syntax
```Rust
for var_name in expression {
    ...
}
```
where `expression` is an `iterator`.

Notes:
- The `iterator` allows to navigate through **collection**. 
- **Each element** of *collection* is **one** **iteration** of the loop. 
- **Each element** of *collection* is bound to the identifier **var_name**, which is **only valid inside** the loop.

<br>

### Examples
- Iterate over vector:
```Rust
let v = &["apples", "cake", "coffee"];

for item in v {
    println!("I like {}.", item);
}
```

- Iterate over range:
```Rust
for i in 1..6 {
    my_f();
}
```

<br>

## Iterator loops with `enumeration`
### Examples 
- Iterate over range with enumeration:
```Rust
for (i, j) in (5..10).enumerate() {
    println!("i = {}; j = {}.", i, j);
}

Output:
    i = 0; j = 5.
    i = 1; j = 6.
    i = 2; j = 7.
    i = 3; j = 8.
    i = 4; j = 9.
```

<br>

## Predicate loops
There is `while` loop in Rust when *number of iterations* in **unknown**.

<br>

### Syntax
```Rust
while expression {
    ...
}
```

where `expression` is `predicate`, i.e., returns `bool` type.

### Example
```Rust
let mut i = 0;

while i < 10 {
    println!("foo");
    i = i + 1;
}
```

<br>

# Infinite loops
### Syntax
```Rust
loop {
    ...
}
```

It is similar to `while true { ... }`. But from compiler point of view it is different cases and compiler uses **additional optimizations** for `loop {}` variant.

### Example
```Rust
loop {
    println!("hello");
}
```

<br>

# Loop labels
By default, statements `break` and `continue` **refer** to the **current** *loop*.<br>
**Labels** allow to **apply** statements `break` and `continue` to the **corresponding** *outer loop*.

## Example
```Rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; }
        if y % 2 == 0 { continue 'inner; }
        println!("x: {}, y: {}", x, y);
    }
}
```
