# ToOwned
Defenition of trait ``ToOwned``:
```Rust
trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}
```

Method ``to_owned`` called on reference returns owned type.<br>

Consider: ``let s1: &'static str = "hello";``. Then ``s1.to_owned()`` will return ``String``.
