# Table of contents
- [Table of contents](#table-of-contents)
- [`std::mem`](#stdmem)
  - [Defenitions](#defenitions)
- [`mem::take`](#memtake)
- [`mem::replace`](#memreplace)

<br>

# `std::mem`
- [**`std::mem::drop`**](https://doc.rust-lang.org/std/mem/fn.drop.html)
  - disposes of a value;
- [**`std::mem::size_of`**](https://doc.rust-lang.org/std/mem/fn.size_of.html)
  - returns the **size of a type** in bytes;
- [**`std::mem::size_of_val`**](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)
  - returns the **size of the pointed-to value** in bytes;
  - this is usually the same as `size_of::<T>()`;
  - however, when `T` has no statically-known size, e.g., a slice `[T]` or a **trait object**, then `size_of_val` can be used to get the **dynamically-known size**;
- [**`std::mem::swap`**](https://doc.rust-lang.org/std/mem/fn.swap.html)
  - lets you **switch** the values between two variables;
    - if you want to **swap** with a **default** or dummy value, see `take`;
    - if you want to **swap** with a **passed value**, returning the old value, see `replace`;
- [**`std::mem::replace(dest: &mut T, src: T)`**](https://doc.rust-lang.org/std/mem/fn.replace.html)
  - **takes** value of `T` (`*dest`);
  - **replaces** the value with what you put in `src`;
  - **returns** the **old** value `*dest`;
- [**`std::mem::take(dest: &mut T)`**](https://doc.rust-lang.org/std/mem/fn.take.html)
  - **takes** value of `T` (`*dest`) and **replaces** it with the **default value** of `T`: that’s why this function requires the type `T` to implement `Default`;
  - **returns** the **taken** value `T`;
  - in practice, the `take()` function is often used as a convenience method to **turn** `Some` **into** `None` **without** having to do any *pattern matching*;

<br>

## Defenitions
```rust
pub const fn swap<T>(x: &mut T, y: &mut T) { }

pub const fn replace<T>(dest: &mut T, src: T) -> T {
    let old = *dest;
    swap(dest, &mut src);
    old
}

pub const fn take<T: [const] Default>(dest: &mut T) -> T {
    replace(dest, T::default())
}
```

Also `impl<T> Option<T> { }` provides method `take()`:
```rust
    pub const fn take(&mut self) -> Option<T> {
        mem::replace(self, None)
    }
```

<br>

# `mem::take`
```rust
use std::mem;

fn main() {
    let mut old_vec = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];
    
    old_vec.iter_mut().for_each(|number| {
        let old_value = mem::take(number);
        new_vec.push(old_value);
    });
    println!("old_vec={:?}\nnew_vec={:?}", old_vec, new_vec);
}
```

<br>

Using `take()` to **turn** `Some` **into** `None`:
```rust
struct Value {
    value: Option<u32>,
}

impl Value {
    fn has_value(&self) -> bool {
        self.value.is_some()
    }
    fn add_val(&mut self, v: u32) {
        self.value = Some(v);
    }
    fn rem_val(&mut self) {
        self.value.take();
    }
}
fn main() {
    let mut user_state = Value {
        value: None,
    };
    user_state.add_val(100);
    println!("self.value = {}", user_state.has_value());
    
    user_state.rem_val();
    println!("self.value = {}", user_state.has_value());
}
```

**Output**:
```bash
self.value = true
self.value = false
```

In the above example `self.value.take();` just takes the value and do nothing with it, leaving `None` in its place.<br>

<br>

# `mem::replace`
```rust
use std::mem;

struct City {
    name: String,
}

impl City {
    fn change_name(&mut self, name: &str) {
        let former = mem::replace(&mut self.name, name.to_string());
        println!("{former} is now called {new}.", new = self.name);
    }
}

fn main() {
    let mut city = City {
        name: "msk".to_string(),
    };

    city.change_name("spb");
}
```

<br>