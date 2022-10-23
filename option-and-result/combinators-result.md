# Combinators for ``Option`` type
- https://doc.rust-lang.org/std/result/

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
fn is_ok(&self) -> bool
```

</td>


<td>

If the ``self`` is ``Err`` it returns ``false``.<br>If the ``self`` is ``Ok`` it returns ``true``.

</td>
</tr>

<tr></tr>
<tr>
<td>

```Rust
fn is_err(&self) -> bool
```

</td>

<td>

If the ``self`` is ``Err`` it returns ``true``. <br>If the ``self`` is ``Ok`` it returns ``false``.

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
fn as_ref(&self) -> Result<&T, &E>
```

</td>

<td>

Converts from ``&Result<T, E>`` to ``Result<&T, &E>``.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn as_mut(&mut  self) -> Result<&mut T, &mut E>
```

</td>

<td>

Converts from ``&mut Result<T, E>`` to ``Result<&mut T, &mut E>``.

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
where
    E: Debug
```

</td>
<td>

- If the result is ``Ok(v)`` returns **inner value** ``v`` of type ``T``;
- If the result is ``Err(e)`` **panics** with a **generic message**.

</td>
</tr>

<tr></tr>

<tr>
<td>

```Rust
fn expect(self, msg: &str) -> T
where
    E: Debug
```

</td>
<td>

- If the result is ``Ok(v)`` returns **inner value** ``v`` of type ``T``.
- If the result is ``Err(e)`` **panics** with a **custom message** provided by ``msg``.

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

- If the result is ``Ok(v)`` returns **inner value** ``v`` of type ``T``.
- If the result is ``Err(e)`` returns the **default value** provided by ``default``.

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

- If the result is ``Ok(v)`` returns **inner value** ``v`` of type ``T``.
- If the result is ``Err(e)`` calls **closure** ``f()`` and returns **its result** of type ``T``.

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

- If the result is ``Ok(v)`` returns **inner value** ``v`` of type ``T``.
- If the result is ``Err(e)`` returns the **default value** tor type ``T``. Type ``T`` must implement ``Default`` trait..

</td>
</tr>

</table>

<br>

# Methods for transforming the contained value
## Transform ``Result<>`` to ``Option<>``
- ``err()`` transforms ``Result<T, E>`` into ``Option<E>``
    - ``Err(e)`` => ``Some(e)``;
    - ``Ok(v)`` => ``None``;
- ``ok()`` transforms ``Result<T, E>`` into ``Option<T>``
    - ``Ok(v)`` => ``Some(v)``;
    - ``Err(e)`` => ``None``;
- ``transpose()`` transposes a ``Result`` of an ``Option`` into an ``Option`` of a ``Result``: ``Result<Option<i32>, E>`` => ``Option<Result<i32, E>>``:
    - ``Ok(None)`` => ``None``;
    - ``Ok(Some(v))`` => ``Some(Ok(v))``;
    - ``Err(e)`` => ``Some(Err(e))``.

<br>

## Transform ``Result<>`` to ``Result<>``:
- ``map(f)`` transforms ``Result<T, E>`` into ``Result<T2, E>``
    - if the result is ``Err(e)`` it leaves the value ``e`` of the ``Err`` variant unchanged;
    - if the result is ``Ok(v)`` it transforms ``T`` into ``U`` by applying the provided function ``f`` to the value ``v`` of the ``Ok`` variant.

- ``map_err(f)`` transforms ``Result<T, E>`` into ``Result<T, E2>``
    - if the result is ``Ok(v)`` it leaves the value ``v`` of the ``Ok`` variant unchanged;
    - if the result is ``Err(e)`` it transforms ``E`` into ``U`` by applying the provided function ``f`` to the value e of the ``Err`` variant.


<br>

## Transform an ``Result<T, E>`` into a value of a **possibly** different type ``U``:
- ``map_or(default, f)`` 
    - if the result is ``Ok(v)`` it applies the provided function ``f`` to the value ``v`` of the ``Ok`` variant;
    - if the result is ``Err(e)`` it returns the provided **default value** by default.

- ``map_or_else(d, f) ``
    - if the result is ``Ok(v)`` it applies the provided function ``f`` to the value ``v`` of the ``Ok`` variant;
    - if the result is ``Err(e)`` it applies the provided default fallback function ``d`` to the value ``e`` of the ``Err`` variant.


<br>

# Methods acting as ``boolean`` operators
These methods treat the ``Result`` as a ``boolean`` value.
The ``and()`` and ``or()`` methods take another ``Result`` as **input**, and produce a ``Result`` as **output**.
The ``and_then()`` and ``or_else()`` methods take a **function** ``f`` as **input**, and produce a **Result** as **output**.

The ``and()`` method can produce a ``Result<U, E>`` value having a **different** *inner type* ``U`` than ``Result<T, E>``.
The ``or()`` method can produce a ``Result<U, E2>`` value having a **different** *inner type* ``E2`` than ``Result<T, E>``.

- ``and(r)``
    - If the ``self`` is ``Err(e)`` it returns ``Err(e)``.
    - If the ``self`` is ``Ok(t)`` it returns ``r``, where ``r`` is of ``Result<U, E>``.

- ``and_then(f)``
    - If the ``self`` is ``Err(e)`` it returns ``Err(e)``.
    - If the ``self`` is ``Ok(t)`` it calls ``f(t)``, where ``t`` is of type ``T`` and returns ``Result<U, E>``.

- ``or(r)``
    - If the ``self`` is ``Err(e)`` it returns ``r``, where ``r`` is of ``Result<T, E2>``.
    - If the ``self`` is ``Ok(t)`` it returns ``Ok(t)``.

- ``or_else(f)``
    - If the ``self`` is ``Err(e)`` it calls ``f(e)`` where ``e`` is of type ``E`` and ``f(e)`` returns ``Result<T, E2>``.
    - If the ``self`` is ``Ok(t)`` it returns ``Ok(t)``.
