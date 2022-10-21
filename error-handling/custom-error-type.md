# Custom error type
Rust allows to define **custom** *error type* ``E`` in ``Result<T, E>``.<br>

Custom error type ``E``:
- **must** implement ``std::fmt::Display`` trait;
- **must** implement ``std::error::Error`` trait;
- *may* implement ``std::fmt::Debug`` trait;
- *may* implement ``std::convert::From`` trait or ``std::convert::TryFrom`` trait.

Through the ``Display`` and ``Debug`` traits **errors** describe themselves.

### Example
```Rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// a test function that returns our error result
fn example(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("ABC"))
    } else {
        Ok(())
    }
}
```