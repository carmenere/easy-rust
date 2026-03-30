# Table of contents
- [Table of contents](#table-of-contents)
- [`std::time`](#stdtime)
- [`Instant`](#instant)
- [`SystemTime`](#systemtime)
- [`Duration`](#duration)

<br>

# `std::time`
More about time [**here**](https://github.com/carmenere/easy-manuals/blob/main/time/time.md).<br>

The `std::time` module provides:
- `Instant` uses `clock_gettime` syscall with **clock source** `CLOCK_MONOTONIC`: it shows the time (**seconds** + **nanoseconds**) that have passed **since** the **system booted up**;
  - the `Instant` corresponds the **real** time of cli command `time`;
  - it is **monotonic** and **connot** be changed;
- `SystemTime` uses `clock_gettime` syscall with **clock source** `CLOCK_REALTIME`: it shows the time (**seconds** + **nanoseconds**) that have passed **since** the `UNIX_EPOCH`;
  - it can be used to get the **current date and time**;
  - unlike `Instant` the `SystemTime` is **not** *monotonic*, i.e. it **can** be changed by NTP or system administrator, and **can** move forwards and backwards;
- **const** `UNIX_EPOCH` represents the **Unix Epoch** `1970-01-01 00:00:00 UTC`;

<br>

Notation:
- a **millisecond**: symbol is **ms**;
- a **microsecond**: symbol is **μs**, sometimes simplified to **us** when Unicode is not available;
- a **nanosecond**: symbol is **ns**;

<br>

```rust
use std::time::{Instant, SystemTime};

fn main() {
    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("{instant:?}");
    println!("{system_time:?}");
}
```
**Output**:
```rust
Instant { tv_sec: 8804, tv_nsec: 118895000 }
SystemTime { tv_sec: 1772526300, tv_nsec: 299094000 }
```

<br>

# `Instant`
As documentation says `Instant` is **opaque** and useful only with `Duration` for **benchmarks** or *measuring* **how long an operation takes**.<br>
A `Duration` is a struct that is used to show how much time has passed.<br>

**Example**:
```rust
fn main() {
    let now = std::time::Instant::now();
    println!("{:?}", now);
}
```

**Output**:
```rust
Instant { tv_sec: 6639, tv_nsec: 372499916 }
```

<br>

The `Instant` implements `Sub` which lets us use the *minus symbol* to **subtract** one `Instant` from another:
```rust
impl Sub<Instant> for Instant {
  type Output = Duration;
  fn sub(self, other: Instant) -> Duration {
    self.duration_since(other)
  }
}
```

<br>

```rust
use std::time::Instant;

fn main() {
    let start_of_main = Instant::now();
    let before_operation = Instant::now();
    println!("before_operation - start_of_main = {:?}", before_operation - start_of_main);
    let after_operation = Instant::now();
    println!("after_operation - before_operation = {:?}", after_operation - before_operation);
}
```
**Output**:
```
before_operation - start_of_main = 41ns
after_operation - before_operation = 45.5µs
```

<br>

There is also a method called `.elapsed()` that lets us do the same thing **without creating** a new `Instant` every time:
```rust
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("hello");
    println!("Printing hello took {:?}", start.elapsed());
}
```
**Output**:
```rust
hello
Printing hello took 72.458µs
```

<br>

# `SystemTime`
```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let sys_time = SystemTime::now();
    println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    println!("{:?}", SystemTime::now().duration_since(sys_time).unwrap());
}
```
**Output**:
```rust
1772527114.102559s
71µs
```

<br>

# `Duration`
Inside a thread, you can use `std::thread::sleep()` to make the thread stop for a while, it receives `Duration`.<br>
Creating a `Duration` is fairly simple: pick the method that matches the **unit of time** you want to use and give it a number or use `Duration::new(secs, nanos);`:
```rust
    let s = Duration::from_secs(2);
    let ms = Duration::from_millis(300);
    let us = Duration::from_micros(100);
    let ns = Duration::from_nanos(100);
    let d = Duration::new(5, 000_023_852);
```

<br>

**Example**:
```rust
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let d = Duration::from_secs(3);
    println!("I must sleep now.");
    sleep(d);
    println!("Did I miss anything?");
}
```

<br>

**Methods**:
- `subsec_nanos(&self) -> u32` returns the **fractional part** of `Duration`, **in nanoseconds**;
- `subsec_micros(&self) -> u32` returns the **fractional part** of this `Duration`, **in microseconds**;
- `subsec_millis(&self) -> u32` returns the **fractional part** of this `Duration`, **in milliseconds**;
- `as_secs(&self) -> u64` returns the **number of whole seconds** contained by this `Duration`;
  - the returned value does **not** include the **fractional** (**nanosecond**) **part** of the duration;
- `as_millis(&self) -> u128` returns the **total number** of **milliseconds** contained by this `Duration`;
- `as_micros(&self) -> u128` returns the **total number** of **microseconds** contained by this `Duration`;
- `as_nanos(&self) -> u128` returns the **total number** of **nanoseconds** contained by this `Duration`;


```rust
use chrono::naive::{NaiveDate, NaiveTime};
use std::time::{Duration, Instant, SystemTime};

fn main() {
    let start = SystemTime::now();
    
    let duration = Duration::new(5, 000_033_852);
    println!("\n1) duration.as_nanos()  = {:?}", duration.as_nanos());
    println!("1) duration.subsec_nanos()  = {:?}", duration.subsec_nanos());

    let duration = Duration::new(5, 000_000_852);
    println!("\n2) _852 will be dropped");
    println!("2) duration.as_micros() = {:?}", duration.as_micros());
    println!("2) duration.subsec_micros() = {:?}", duration.subsec_micros());

    let duration = Duration::new(5, 000_033_852);
    println!("\n3) _852 will be dropped");
    println!("3) duration.as_micros() = {:?}", duration.as_micros());
    println!("3) duration.subsec_micros() = {:?}", duration.subsec_micros());

    let duration = Duration::new(5, 000_033_852);
    println!("\n4) _033_852 will be dropped");
    println!("4) duration.as_millis() = {:?}", duration.as_millis());
    println!("4) duration.subsec_millis() = {:?}", duration.subsec_millis());

    let duration = Duration::new(5, 044_033_852);
    println!("\n5) _033_852 will be dropped");
    println!("5) duration.as_millis() = {:?}", duration.as_millis());
    println!("5) duration.subsec_millis() = {:?}", duration.subsec_millis());

    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("\nduration = {:?}", duration);
}
```
**Output**:
```rust
1) duration.as_nanos()  = 5000033852
2) duration.subsec_nanos()  = 33852

3) _852 will be dropped
4) duration.as_micros() = 5000000
5) duration.subsec_micros() = 0

6) _852 will be dropped
7) duration.as_micros() = 5000033
8) duration.subsec_micros() = 33

9) _033_852 will be dropped
10) duration.as_millis() = 5000
11) duration.subsec_millis() = 0

12) _033_852 will be dropped
13) duration.as_millis() = 5044
14) duration.subsec_millis() = 44

duration = 55µs
```

<br>
