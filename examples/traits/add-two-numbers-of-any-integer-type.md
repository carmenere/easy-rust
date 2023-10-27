# Example: function that adds two numbers of any integer type
## Variant 1: using trait `Into`
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

## Variant 2: using trait `From`
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

<br>

## Variant 3: more generic version
```Rust
fn add<T1, T2, R>(a: T1, b: T2) -> R 
where T1: Into<R>,
      T2: Into<R>,
      R: std::ops::Add<Output = R>
{ a.into() + b.into() }

fn main() {
    let x: i8 = 10;
    let y: i64 = 100;
    println!("{}", {let x:i64 = add(x,y); x});
    
    let x: u8 = 10;
    let y: u64 = 100;
    println!("{}", {let x:u64 = add(x,y); x});
}
```