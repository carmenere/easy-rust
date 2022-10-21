# Combinators for ``Option`` type
- https://doc.rust-lang.org/std/option/

<br>

# Methods for checking the contained value
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

If the ``self`` is ``None`` it returns ``false``.<br>If the ``self`` is ``Some(t)`` it returns ``true``.

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

If the ``self`` is ``None`` it returns ``true``. <br>If the ``self`` is ``Some(t)`` it returns ``false``.

</td>
</tr>
</table>

<br>

# Methods for working with references
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

Converts from ``&Option<T>`` to ``Option<&T>``.

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

Converts from ``&mut Option<T>`` to ``Option<&mut T>``.

</td>


</table>

<br>

# Methods for extracting the contained value
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

- If the result is ``Some(v)`` returns **inner value** of type ``T``;
- If the result is ``None`` **panics** with a **generic message**.

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

- If the result is ``Some(v)`` returns **inner value** of type ``T``.
- If the result is ``None`` **panics** with a **custom message** provided by ``msg``.

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

- If the result is ``Some(v)`` returns **inner value** of type ``T``.
- If the result is ``None`` returns the **default value** provided by ``default``.

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

- If the result is ``Some(v)`` returns **inner value** of type ``T``.
- If the result is ``None`` calls **closure** ``f()`` and returns **its result** of type ``T``.

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

- If the result is ``Some(v)`` returns **inner value** of type ``T``;
- If the result is ``None`` returns the **default value** tor type ``T``. Type ``T`` must implement ``Default`` trait.

</td>
</tr>

</table>

<br>

# Methods for transforming the contained value
## Transform ``Option<>`` to ``Result<>``
- ``ok_or(err)`` transforms ``Option<T>`` to ``Result<T, E>``:
    - ``Some(v)`` to ``Ok(v)``;
    - ``None`` to ``Err(err)``, where err of type ``E``.

- ``ok_or_else(f)`` transforms ``Option<T>`` to ``Result<T, E>``:
    - ``Some(v)`` => ``Ok(v)``
    - ``None`` => ``f()``, where ``f()`` returns value of type ``E``.

- ``transpose()`` transposes ``Option<Result<i32, E>>`` => ``Result<Option<i32>, E>``
    - ``None`` => ``Ok(None)``
    - ``Some(Ok(v))`` => ``Ok(Some(v)) ``
    - ``Some(Err(e))`` => ``Err(e)``

<br>

## Transform ``Option<>`` to ``Option<>``:
- ``map(f)``
    - if the ``self`` is ``None`` it returns ``None``;
    - if the ``self`` is ``Some(t)`` it transforms ``T`` into ``U`` by applying the provided function ``f`` to the value ``t`` of the ``Some`` variant.

- ``filter(f)``
    - if the ``self`` is ``None`` it returns ``None``.
    - if the ``self`` is ``Some(t)`` it applies the provided function ``f`` to the value ``t`` of the ``Some`` variant and returns:
        - ``Some(t)`` if ``f(t)`` returns ``true``;
        - ``None`` if ``f(t)`` returns ``false``.

- ``flatten()`` converts from ``Option<Option<T>>`` to ``Option<T>``

<br>

## Transform an ``Option<T>`` into a value of a **possibly** different type ``U``:
- ``map_or(default, f)``
    - if the ``self`` is ``Some(v)`` it applies the provided function ``f`` to the value ``t`` of the ``Some`` variant, where ``f(t)`` returns ``U``;
    - if the ``self`` is ``None`` it returns the provided **default value** by default.

- ``map_or_else(d, f) ``
    - if the ``self`` is ``Some(v)`` it applies the provided function ``f`` to the value ``t`` of the ``Some`` variant;
    - if the ``self`` is ``None`` it returns ``d()``, where ``d()`` returns value of type ``U``.

<br>

# Methods acting as ``boolean`` operators
These methods treat the ``Option`` as a ``boolean`` value.<br>
The ``and()`` and ``or()`` methods take another ``Option`` as **input**, and produce an ``Option`` as **output**.<br>
The ``and()`` method can produce an ``Option<U>`` value having a **different** *inner type* ``U``.
The ``and_then()`` and ``or_else()`` methods take a function ``f`` as input, and produce an ``Option`` as output.

- ``and(o)``
    - If the ``self`` is ``None`` it returns ``None``.
    - If the ``self`` is ``Some(t)`` it returns ``o``, where ``o`` is of type ``Option<U>``.

- ``and_then(f)``
    - If the ``self`` is ``None`` it returns ``None``.
    - If the ``self`` is ``Some(t)`` it calls ``f(t)`` where t is of type T and ``f(t)`` returns ``Option<U>``.

- ``or(o)``
    - If the ``self`` is ``None`` it returns o, where o is of type ``Option<T>``.
    - If the ``self`` is ``Some(t)`` it returns ``Some(t)``.

- ``or_else(f)``:
    - If the ``self`` is ``None`` it calls ``f()`` and ``f()`` returns value of type ``Option<T>``.
    - If the ``self`` is ``Some(t)`` it returns ``Some(t)``.

<br>

### Declarations
```Rust
fn and<U>(self, optb: Option<U>) -> Option<U>;

fn or(self, optb: Option<T>) -> Option<T>;

fn and_then<U, F>(self, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>;

fn or_else<F>(self, f: F) -> Option<T>
where
    F: FnOnce() -> Option<T>;

```
