# Ways of errors handling
Generally speaking, there 2 ways of **error handling**: 
- **exceptions**; 
- **return values**. 

Rust uses **return values** approach. 

There are 2 kind of errors in Rust: 
- **recoverable**;
- **unrecoverable**. 

**Unrecoverable** errors are always symptoms of bugs.

Rust has the type ``Result<T, E>`` for **recoverable** errors and the ``panic!`` macro for **unrecoverable** error.
