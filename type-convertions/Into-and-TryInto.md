# Trait ``Into``
Path in **std** is ``std::convert::Into``.<br>
Trait ``Into`` is used to convert value **from** type ``S`` **to** type ``D`` and **must** be implemented on ``S`` type.<br>
Trait ``Into`` **must** **not fail**. If the conversion **can fail**, use ``TryInto``.<br>

**Declaration** of ``Into``:
```Rust
pub trait Into<T> {
    fn into(self) -> T;
}
```

Method ``into()`` performs the conversion.<br>

It is important to understand that ``Into`` **doesn't** automatically implements a ``From`` (as ``From`` does with ``Into``).<br>
Therefore, you should always try to implement ``From`` and then fall back to ``Into`` if ``From`` can’t be implemented.<br>

<br>

### Example
Prior to Rust 1.41, if the **destination type** was not part of the current crate then you couldn’t implement `From` directly.
For example, the code below will fail  in older versions:
```Rust
struct Wrapper<T>(Vec<T>);
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}
```

<br>

To bypass this, you could implement Into directly:
```Rust
struct Wrapper<T>(Vec<T>);
impl<T> Into<Vec<T>> for Wrapper<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
```

<br>

# Trait ``TryInto``
Path in **std** is ``std::convert::TryInto``.<br>
``TryInto<T>`` returns ``Result<T, E>``.<br>

**Declaration** of ``TryInto``:
```Rust
pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```
