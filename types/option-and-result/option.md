# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Combinators](#combinators)
  - [Predicates for checking the contained value](#predicates-for-checking-the-contained-value)
  - [Methods for working with references](#methods-for-working-with-references)
  - [Methods for extracting the contained value](#methods-for-extracting-the-contained-value)
    - [Diagram](#diagram)
    - [Examples](#examples)
      - [`unwrap()`](#unwrap)
      - [`expect()`](#expect)
  - [Methods for transforming the contained value](#methods-for-transforming-the-contained-value)
    - [Diagram](#diagram-1)
    - [Transform `Option<T>` to `Result<T,E>`](#transform-optiont-to-resultte)
    - [Transform `Option<T>` to `Option<U>`:](#transform-optiont-to-optionu)
    - [Transform an `Option<T>` into a value of a **possibly** different type `U`:](#transform-an-optiont-into-a-value-of-a-possibly-different-type-u)
  - [Methods acting as `boolean` operators](#methods-acting-as-boolean-operators)
    - [Diagram](#diagram-2)

<br>

# URLs
|URL|
|:------------|
|[std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html)|
|[Combintaors](https://doc.rust-lang.org/std/option/#method-overview)|

<br>

# Declaration
```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

`T` is a type of some **wrapped value**.

<br>

# In a nutshell
`Option` express **optionality** through `enum`.<br>

Value of type `T` can only be obtained via `match`:
```Rust
match val { 
    Some(val) => ...,
    None => ...
}
```

Here `val` is of type `Option<T>`, **after deconstructing**, `val` becomes of type `T`.

<br>

# Combinators
## Predicates for checking the contained value
- [**is_some()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some):
  - if the `self` is `None` it returns `false`;
  - if the `self` is `Some(t)` it returns `true`;
- [**is_none()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none):
  - if the `self` is `None` it returns `true`;
  - if the `self` is `Some(t)` it returns `false`;

<br>

## Methods for working with references
- [**as_ref()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref):
  - converts from `&Option<T>` to `Option<&T>`;
- [**as_mut()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut):
  - converts from `&mut Option<T>` to `Option<&mut T>`;

<br>

## Methods for extracting the contained value
### Diagram
![Option-unwrap](/img/Option-unwrap.png)

<br>

**Methods**:
- [**unwrap()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap);
- [**expect()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect);
- [**unwrap_or()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or);
- [**unwrap_or_else()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else);
- [**unwrap_or_default()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default);

<br>

### Examples
#### `unwrap()`
```rust
None::<i32>.unwrap();
```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
called `Option::unwrap()` on a `None` value
```

<br>

#### `expect()`
```rust
None::<i32>.expect("some_message");
```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
some_message
```

<br>

## Methods for transforming the contained value
### Diagram
![Option-or-and-map](/img/Option-or-and-map.png)

<br>

### Transform `Option<T>` to `Result<T,E>`
- [**ok_or(err: E)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or):
  - **transforms** `Option<T>` to `Result<T, E>`:
- [**ok_or_else(f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else):
  - **transforms** `Option<T>` to `Result<T, E>`:
- [**transpose()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose):
  - **transposes** an `Option` of a `Result` into a `Result` of an `Option`: `Option<Result<T, E>>` to `Result<Option<T>, E>`
    - `None` => `Ok(None)`;
    - `Some(Ok(v))` => `Ok(Some(v)) `;
    - `Some(Err(e))` => `Err(e)`.

<br>

### Transform `Option<T>` to `Option<U>`:
- [**map(f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map):
  - it **returns** `Option`;
- [**filter(p)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.filter):
  - it **returns** `Option`;
    - if the `self` is `None` it **returns** `None`;
    - if the `self` is `Some(t)` it applies the provided predicate `p` to the value `t: T` of the `Some` variant and **returns**:
        - `Some(t)` if `p(t)` **returns** `true`;
        - `None` if `p(t)` **returns** `false`;
- [**flatten()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.flatten):
  - converts from `Option<Option<T>>` to `Option<T>`;

<br>

### Transform an `Option<T>` into a value of a **possibly** different type `U`:
- [**map_or(default: U, f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or):
  - it **returns** **value** of type `U`;
- [**map_or_else(d, f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else):
  - it **returns** **value** of type `U`;

<br>

## Methods acting as `boolean` operators
### Diagram
![Option-or-vs-and](/img/Option-or-vs-xor-vs-and.png)

<br>

**Example**:
```rust
let some_10: Option<i32> = Some(10);
let some_20: Option<i32> = Some(20);
let none_i32: Option<i32> = None;

let some_a: Option<&'static str> = Some("A");
let none_str: Option<&'static str> = None;

// or
assert_eq!(some_10.or(some_20), some_10);
assert_eq!(some_10.or(none_i32), some_10);

assert_eq!(none_i32.or(some_20), some_20);
assert_eq!(none_i32.or(none_i32), none_i32);

// xor
assert_eq!(some_10.xor(none_i32), some_10);
assert_eq!(none_i32.xor(some_20), some_20);

assert_eq!(some_10.xor(some_20), none_i32);
assert_eq!(none_i32.xor(none_i32), none_i32);

// and
assert_eq!(some_10.and(none_str), none_str);
assert_eq!(some_10.and(some_a), some_a);

assert_eq!(none_str.and(some_a), none_str);
assert_eq!(none_str.and(none_str), none_str);

// and: None cane be under different Option type
assert_eq!(none_i32.and(none_str), none_str);
assert_eq!(none_str.and(none_i32), none_i32);
```

<br>

[**Boolean operators doc**](https://doc.rust-lang.org/std/option/index.html#boolean-operators):
- [**and()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and);
- [**or()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.or);
- [**and_then()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then);
- [**or_else()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else);

<br>

All above methods treat the `Option` as a `boolean` value: `Some` is like **1** and `None` is like **0**:
- the `and()` and `or()` methods take another `Option` as **input**, and produce an `Option` as **output**l;
- the `and_then()` and `or_else()` methods take a function `f` as input, and only evaluate the function `f` when they need to produce a new value;
- `and()` and `and_then()` methods can produce an `Option<U>` value having a different inner type `U` than `Option<T>`;
