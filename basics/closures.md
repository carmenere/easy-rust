# Closures
## Notation
`|| -> { ... }`

`||` defines **arguments**, **mandatory**.

`->` defines **returning type**, **optional**.

`{}` defines **body**, **optional**

<br>

## Various closures declarations
```Rust
let x: i32 = || -> i32 { … };
let x: ()  = || {};
let x: ()  = |a, b| { … };
let x: i32 = |a, b| a + b;
```