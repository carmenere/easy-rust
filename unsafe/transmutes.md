# Transmutes
The `std::mem::transmute<S,D>(_src: S) -> D` takes a value of type `T` and casts it into another type `U`.<br>

<br>

**Requirements**:
- transmuting **requires** `T` and `U` to have **the same size**, compilation will **fail** if this is **not guaranteed**;
- transmuting different compound types **requires** them to have **the same layout**;
  - for `repr(C)` and `repr(transparent)` the **layout** is **defined**;

<br>

**UB**:
1. Transmuting `&` to `&mut` is a **UB**.

<br>

The `std::mem::transmute_copy<S,D>(_src: &S) -> D` interprets `src: &S` as having `&D` type, and then reads `src` without moving the contained value.<br>

<br>

**Example**:
```rust
use std::mem::transmute;

fn main() {
    let x = -127;
    let y = -128;
    let z = 200;
    let tx = unsafe { transmute::<i8, u8>(x) };
    let ty = unsafe { transmute::<i8, u8>(y) };
    let tz = unsafe { transmute::<u8, i8>(z) };
    println!("'{x}' transmuted to u8 = '{tx}'");
    println!("'{y}' transmuted to u8 = '{ty}'");
    println!("'{z}' transmuted to i8 = '{tz}'");

    println!("'{x}' as u8 = '{}'", x as u8);
    println!("'{y}' as u8 = '{}'", y as u8);
    println!("'{z}' as i8 = '{}'", z as i8);

    println!("{:b}\n{:b}", -19, 4294967277u32);
}
```

**Output**:
```rust
'-127' transmuted to u8 = '129'
'-128' transmuted to u8 = '128'
'200' transmuted to i8 = '-56'
'-127' as u8 = '129'
'-128' as u8 = '128
```

If `transmute()` is just reinterpreting the same bytes, then `200` and `-56` should look the **same as bytes**. Let’s give it a try:
```rust
fn main() {
    println!("{:b}\n{:b}", -56_i8, 200_u32);
}
```

**Output**:
```rust
11001000
11001000
```

<br>

Indeed, that `transmute()` **takes the same bytes** and **treats them differently**.<br>

**Another example**:
```rust
struct User {
    name: String,
    number: u32,
}

fn main() {
    let arr: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("size_of::<User> = {} bytes", std::mem::size_of::<User>());
    println!("size_of::<[i32; 8]> = {} bytes", std::mem::size_of::<[i32; 8]>());
    
    arr.iter().for_each(|i| {
        print!("{:08b} ", i)
    });
}
```

**Output**:
```rust
size_of::<User> = 32 bytes
size_of::<[i32; 8]> = 32 bytes
00000001 00000010 00000011 00000100 00000101 00000110 00000111 00001000
```

Size of type `User` is 32 bytes. So what happens if we give Rust an array of **eight** `i32`s and tell it to make a `User`?
```rust
use std::mem::transmute;

struct User {
    name: String,
    number: u32,
}

fn main() {
    let arr: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    let user = unsafe { transmute::<[i32; 8], User>(arr) };
}
```

**Output** - **segmentation fault**:
```rust
example(48738,0x1f4e9f080) malloc: *** error for object 0x400000003: pointer being freed was not allocated
example(48738,0x1f4e9f080) malloc: *** set a breakpoint in malloc_error_break to debug
zsh: abort      cargo run --bin example
```

<br>

The `transmute()` documentation says: "This makes `transmute()` **incredibly unsafe**. `transmute()` should be the absolute last resort."<br>
