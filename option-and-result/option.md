# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Combinators](#combinators)
  - [Methods for checking the contained value](#methods-for-checking-the-contained-value)
  - [Methods for working with references](#methods-for-working-with-references)
  - [Methods for extracting the contained value](#methods-for-extracting-the-contained-value)
  - [Methods for transforming the contained value](#methods-for-transforming-the-contained-value)
    - [Transform `Option<T>` to `Result<T,E>`](#transform-optiont-to-resultte)
    - [Transform `Option<T>` to `Option<U>`:](#transform-optiont-to-optionu)
    - [Transform an `Option<T>` into a value of a **possibly** different type `U`:](#transform-an-optiont-into-a-value-of-a-possibly-different-type-u)
  - [Methods acting as `boolean` operators](#methods-acting-as-boolean-operators)

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

`T` – type of **value**. `T` is wrapped in `Some` variant.<br>

<br>

# In a nutshell
`Option` express **optionality** through `enum`.<br>

Value of type `T` can only be obtained via `match`:
```Rust
match val { 
Some(val) => {any code using val},
None => expr
}
```

Here `val` is of type `Option<T>`, after deconstructing, `val` is of type `T`.

<br>

# Combinators
## Methods for checking the contained value
<table>
    <tr>
        <th>Method</th>
        <th>Description</th>
    </tr>
<tr></tr>
<tr>
<td>

```Rust
fn is_some(&self) -> bool
```

</td>


<td>

If the `self` is `None` it returns `false`.<br>If the `self` is `Some(t)` it returns `true`.

</td>
</tr>

<tr></tr>
<tr>
<td>

```Rust
fn is_none(&self) -> bool
```

</td>

<td>

If the `self` is `None` it returns `true`. <br>If the `self` is `Some(t)` it returns `false`.

</td>
</tr>
</table>

<br>

## Methods for working with references
<table>
    <tr>
        <th>Method</th>
        <th>Description</th>
    </tr>
<tr></tr>
<tr>
<td>

```Rust
fn as_ref(&self) -> Option<&T>
```

</td>

<td>

Converts from `&Option<T>` to `Option<&T>`.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn as_mut(&mut self) -> Option<&mut T>
```

</td>

<td>

Converts from `&mut Option<T>` to `Option<&mut T>`.

</td>


</table>

<br>

## Methods for extracting the contained value
<table>
    <tr>
        <th>Method</th>
        <th>Description</th>
    </tr>
<tr></tr>
<tr>
<td>

```Rust
fn unwrap(self) -> T
```

</td>
<td>

- If the result is `Some(v)` returns **inner value** of type `T`;
- If the result is `None` **panics** with a **generic message**.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn expect(self, msg: &str) -> T
```

</td>
<td>

- If the result is `Some(v)` returns **inner value** of type `T`.
- If the result is `None` **panics** with a **custom message** provided by `msg`.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn unwrap_or(self, default: T) -> T
```

</td>
<td>

- If the result is `Some(v)` returns **inner value** of type `T`.
- If the result is `None` returns the **default value** provided by `default`.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn unwrap_or_else<F>(self, f: F) -> T
where
    F: FnOnce() -> T

```

</td>
<td>

- If the result is `Some(v)` returns **inner value** of type `T`.
- If the result is `None` calls **closure** `f()` and returns **its result** of type `T`.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn unwrap_or_default(self) -> T
where
    T: Default
```

</td>
<td>

- If the result is `Some(v)` returns **inner value** of type `T`;
- If the result is `None` returns the **default value** tor type `T`. Type `T` must implement `Default` trait.

</td>
</tr>

</table>

<br>

## Methods for transforming the contained value
### Transform `Option<T>` to `Result<T,E>`
- [**ok_or(err: E)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or):
  - **transforms** `Option<T>` to `Result<T, E>`:
    - `Some(v)` => `Ok(v)`;
    - `None` => `Err(err)`, where `err` of type `E`.

- [**ok_or_else(f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else):
  - **transforms** `Option<T>` to `Result<T, E>`:
    - `Some(v)` => `Ok(v)`;
    - `None` => `f()`, where `f()` returns value of type `E`.

- [**transpose()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose):
  - **transposes** `Option<Result<T, E>>` to `Result<Option<T>, E>`
    - `None` => `Ok(None)`;
    - `Some(Ok(v))` => `Ok(Some(v)) `;
    - `Some(Err(e))` => `Err(e)`.

<br>

### Transform `Option<T>` to `Option<U>`:
- [**map(f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map):
  - it **returns** `Option`;
    - if the `self` is `None` it **returns** `None`;
    - if the `self` is `Some(t)` it transforms `T` into `U` by applying the provided function `f` to the value `t: T` of the `Some` variant and **returns** `Some(u)`;
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
    - if the `self` is `Some(t)` it applies the provided function `f` to the value `t` of the `Some` variant, where `f(t)` **returns** `U`;
    - if the `self` is `None` it returns the provided `default` value of type `U` by default;
- [**map_or_else(d, f)**](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else):
  - it **returns** **value** of type `U`;
    - if the `self` is `Some(t)` it applies the provided function `f` to the value `t` of the `Some` variant, where `f(t)` **returns** `U`;
    - if the `self` is `None` it computes a **default** function `d()` and **returns** its result, where `d()` returns `U`;;

<br>

## Methods acting as `boolean` operators
[**Boolean operators doc**](https://doc.rust-lang.org/std/option/index.html#boolean-operators):
- [**and()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and);
- [**or()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.or);
- [**and_then()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then);
- [**or_else()**](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else);

<br>

All above methods treat the `Option` as a `boolean` value.<br>
- The `and()` and `or()` methods take another `Option` as **input**, and produce an `Option` as **output**.<br>
- The `and_then()` and `or_else()` methods take a function `f` as input, and only evaluate the function `f` when they need to produce a new value.
  - Only the `and_then()` method can produce an `Option<U>` value having a different inner type `U` than `Option<T>`.

