# AsRef and AsMut
``std::convert::AsRef`` and ``std::convert::AsMut`` are used for cheaply **converting types to references**. <br>

- If some type ``T`` implements ``AsRef<U>``, that means it possible to get **shared** *reference* ``&U`` from ``T``.
- If some type ``T`` implements ``AsMut<U>``, that means it possible to get **mutable** *reference* ``&mut U`` from ``T``.

<br>

In other words, for types ``A`` and ``B``:
- ``impl AsRef<B> for A`` indicates that a ``A`` can be converted to a ``&B``.
- ``impl AsMut<B> for A`` indicates that a ``A`` can be converted to a ``&mut B``.

This is useful for **performing** *type conversions* **without** **copying** or **moving** values.<br>

For instance, 
- ``Vec<T>`` implements ``AsRef<[T]>``;
- ``String`` implements ``AsRef<str>``.

<br>

Blanket implementation of ``AsRef`` in **std**:
```Rust
impl<a', T, U> AsRef<U> for &a' T
where T: AsRef<U> + ?Sized,
      U: ?Sized
{
    fn as_ref(&self) -> &U {
        (*self).as_ref()
    }
}
```

It means: for **any types** ``T`` and ``U``, if ``T: AsRef<U>``, then ``&T: AsRef<U>`` as well.

#### Explanation
Consider function ``std::fs::File.open()``:
```Rust
fn open<P: AsRef<Path>>(path: P) -> Result<File>
```

This allows ``File.open()`` to accept not only ``Path``, but also ``OsStr``, ``OsString``, ``&str``, ``String``, and ``PathBuf`` with implicit conversion because these types all implement ``AsRef<Path>``.
