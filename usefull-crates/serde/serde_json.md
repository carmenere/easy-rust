# ``serde_json``
#### Example 1
```Rust
use serde::{Serialize, Deserialize};
use serde_json; // 1.0.67
#[derive(Debug, Deserialize,Serialize)]
struct Test {
    m: u16,
    o: Option<u16>
}

fn main() {
let d1 = r#"{"m": 100, "o": 20}"#;
let t: Test = serde_json::from_str(&d1).unwrap();
println!("{}", serde_json::to_string_pretty(&t).unwrap());

let d2 = r#"{"m": 100}"#;
let t: Test = serde_json::from_str(&d2).unwrap();
println!("{}", serde_json::to_string_pretty(&t).unwrap());
}
```

<br>

#### Example 2
```Rust
use serde::{Serialize, Deserialize};
use serde_json;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize,Serialize, Validate)]
#[validate(schema(function = "check_range"))]
struct a {
    x: std::ops::Range<u16>
}

fn check_range(range: &a) -> Result<(), ValidationError> {
    if range.x.is_empty() {
        Err(ValidationError::new(
            "'end' can not be less then 'start'.",
        ))
    }
    else {
        Ok(())
    }
}

fn main() {
    let data = r#"{"x": {"start": 100, "end": 20}}"#;
    println!("data: {}", data);
    let t: a = serde_json::from_str(&data).unwrap();
    match t.validate() {
        Ok(p) => Ok(()),
        Err(p) => {println! ("{:?}", p); Err(ValidationError::new("abc"))},
    };
    println!("serde_json::to_string_pretty: \n{}", serde_json::to_string_pretty(&t).unwrap());
}
```
