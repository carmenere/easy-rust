# Macros try! and ? operator
The ``?`` operator is equivalent to ``try!``.<br>

``try!`` is **deprecated**.<br>

Syntax for ``try!``: ``let a = try!(expr);``.<br>

Syntax for ``?``: ``let a = expr?``.

Both ``try!`` and ``?`` **unwrap** ``Result`` OR perform **prematurely** /premətʃʊəʳli/ **return** from function.<br>

To use ``?``, **calling** and **called** functions must use ``Result`` as return type.

Definition of ``try!`` in **std**:
```Rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
```

``expr?`` unfolds to:
```Rust
match ::std::ops::Try::into_result(expr) {
    Ok(val) => val,
    Err(err) => return ::std::ops::Try::from_error(From::from(err)),
}
```