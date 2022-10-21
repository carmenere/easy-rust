# Borrow and BorrowMut
The ``std::borrow::Borrow`` and ``std::borrow::BorrowMut`` traits are used to **treat borrowed types like owned types**.<br>
For types ``A`` and ``B`` ``impl Borrow<B> for A`` indicates that a borrowed ``A`` may be used where a ``B`` is desired.<br>

For instance, ``std::collections::HashMap.get()`` uses ``Borrow`` for its ``get()`` method, allowing a ``HashMap`` with keys of ``K`` to be indexed with a ``&Q``.<br>

## Defenitions
```Rust
trait Borrow:
trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

trait BorrowMut:
trait BorrowMut<Borrowed: ?Sized> {
    fn borrow_mut(&mut self) -> &mut Borrowed;
}
```

### Example
We can **insert** **key** of ``String`` type into ``HashMap`` and then use **key** of ``&strs`` type for **searching**:
```Rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("Foo".to_string(), 42);

assert_eq!(map.get("Foo"), Some(&42));
```

This is because the standard library has ``impl Borrow<&str> for String``.
