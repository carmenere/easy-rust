# Direct dereference
```rust
use std::arch::asm;

fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}

fn main() {
    let ptr = 999999999000 as *const usize;
    println!("value {}", dereference(ptr));
}
```

<br>

The address `999999999000` has **3** zeros at the end `000` because it is aligned on **8** byte boundary for **64-bit** arch.<br>
The code above will cause to **segmentation fault** (**segfault**) error.<br>