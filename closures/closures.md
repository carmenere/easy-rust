# Ordinary closure
**Closure** aka **anonymous function** or **lambda**.<br>

We create a **closure** using the ``|...| {...}`` syntax, and then we create a ``let`` **binding** so we can use it later.<br>
Note that we call the closure using the **binding name** and **parentheses**, just like we would for a **named function**.<br>

**Closure has access to values in the scope where it defined**. In other words, **closure** **captures** any **values** it uses from **scope** where it is **defined**.<br>

**By default**, the *closure* **borrows** *captured values*.<br>

**Syntax** for **closure** (braces are optional) and **named function**:
<table>
<tr>
<td>

**Closure**

</td>


<td>

```Rust
let add_one = |x: i64| -> i64 { 1 + x };
let add_one = |x: i64|        { 1 + x };
let add_one = |x: i64|          1 + x  ;
```

</td>
</tr>

<tr></tr>
<tr>
<td>

**Closure without args**

</td>

<td>

```Rust
let x = 1;

|| -> i64 { 1 + x } 
||        { 1 + x }
||          1 + x
```

</td>
</tr>

<tr></tr>
<tr>
<td>

**Function**

</td>

<td>

```Rust
fn addone (x: i64) -> i64 { 1 + x }
```

</td>
</tr>

</table>

<br>

#### Example
```Rust
fn main() {
    let x: u64 = 1;
    let print = || { println!("x = {}", x); };
    print();
}
```

**Output**:
```bash
x = 1
```

<br>

# Moving closures
Rust has a second type of **closure**, called a **moving closure**.<br>

We create a **moving closure** using the ``move |...| {...}`` syntax.<br>

**Moving closure** always **takes ownership** of **all values** that it uses. **Moving closures** are most useful with **concurrency**.<br>

<br>

#### Example
```Rust
fn main() {        
    let mut v: Vec<u64> = vec![1; 3];
    let mut print_v = || { v.push(1); println!("v = {:?}", v); };

    print_v();
    print_v();

    v.push(1);

    println!("v = {:?}", v);
}

Output:
v = [1, 1, 1, 1]
v = [1, 1, 1, 1, 1]
v = [1, 1, 1, 1, 1, 1]
```

<br>

#### Example
```Rust
fn main() {        
    let mut v: Vec<u64> = vec![1; 3];
    let mut print_v = move || { v.push(1); println!("v = {:?}", v); };

    print_v();
    print_v();

    v.push(1);

    println!("v = {:?}", v);
}
```

**Output**:
```bash
Error:
error[E0382]: borrow of moved value: `v`
 --> src/main.rs:8:5
  |
2 |     let mut v: Vec<u64> = vec![1; 3];
  |         ----- move occurs because `v` has type `Vec<u64>`, which does not implement the `Copy` trait
3 |     let mut print_v = move || { v.push(1); println!("v = {:?}", v); };
  |                       -------   - variable moved due to use in closure
  |                       |
  |                       value moved into closure here
...
8 |     v.push(1);
  |     ^ value borrowed here after move
```

<br>

# Closure traits and closure capture modes
There are **3 capture modes**:
- by **Move**;
- by **Mutable borrow**;
- by **Immutable borrow**;

<br>

There are **3 closure traits**:
|**Trait**|**Declaration**|
|:--------|:--------------|
|``FnOnce``|``pub trait FnOnce<Args>``|
|``FnMut``|``pub trait FnMut<Args>: FnOnce<Args>``|
|``Fn``|``pub trait Fn<Args>: FnMut<Args>``|

These traits are implemented **automatically** for closures after Rust compiler choose **capture mode** for values that are used inside **closure**.<br>

<br>

So:
- ``FnOnce`` is **supertrait** for ``FnMut``.
- ``FnMut`` is **supertrait** for ``Fn``.

<br>

**Mapping** between **capture modes** and **traits**:
<table>
<tr>
<th>Trait</th>
<th>Capture mode</th>
<th>Description</th>
<th>Syntax</th>
</tr>

<tr></tr>
<tr>
<td>

``FnOnce``

</td>


<td>

**Move**

</td>
<td>

**Captures** *values* **from scope** with **move semantics**.

</td>
<td>

```Rust
let s3 = String::from("value");
let FnOnce_closure = move || {
        let a = s3;
};
```

</td>
</tr>

<tr></tr>
<tr>
<td>

``FnMut``

</td>


<td>

**Mutable borrow**

</td>
<td>

**Captures** *values* **from scope** by **mutable reference**.

</td>
<td>

```Rust
let mut s2 = String::from("abc");
let mut FnMut_closure = || {
    s2.push_str(" xyz");
};
```

</td>
</tr>

<tr></tr>
<tr>
<td rowspan="3">

``Fn``

</td>


<td rowspan="3">

**Immutable borrow** (*by default*).

</td>
<td>

**Captures** *values* **from scope** by **shared reference**.

</td>
<td>

```Rust
let s1 = String::from("abc");
let Fn_closure = || {
    println!("s1.len() = {}.", s1.len());
};
```

</td>
</tr>


<tr></tr>
<tr>
<td>

Closure doesn’t use any value from its scope.

</td>
<td>

```Rust
let Fn_closure = || {
    println!("Hello!");
};

```

</td>
</tr>

</table>

<br>

# Another examples
#### Function that accepts one closures
```Rust
fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}

fn square(x: i32) -> i32 { x * x }

fn main() {
    twice(5, square); // evaluates to 50
}
```

<br>

#### Function that accepts two closures
```Rust
fn compose<F, G>(x: i32, f: F, g: G) -> i32
    where F: Fn(i32) -> i32, 
          G: Fn(i32) -> i32 
{
    g(f(x))
}

fn main() {
    compose(5,
            |n: i32| { n + 42 },
            |n: i32| { n * 2 }); // evaluates to 94
}
```
