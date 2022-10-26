# Examples
## 1. Function that adds two numbers of any integer type
### Over trait ``Into``
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

fn main() {
    let a: u16 = 5;
    let b: u8 = 10;
    
    let s = add(a, b);
    println!("Sum of 'a' and 'b' = {}", s);
}

fn add<Tx, Ty>(x: Tx, y: Ty) -> u32
where
    Tx: Into<u32>,
    Ty: Into<u32>
{
    x.into() + y.into()
}
```

<br>

### Over trait ``From``
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

fn main() {
    let a: u16 = 5;
    let b: u8 = 10;
    
    let s = add(a, b);
    println!("Sum of 'a' and 'b' = {}", s);
}

fn add<Tx, Ty>(x: Tx, y: Ty) -> u32
where
    u32: From<Tx> + From<Ty>
{
    u32::from(x) + u32::from(y)
}
```