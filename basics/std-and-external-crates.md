# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Manage dependencies](#manage-dependencies)
- [`rand` crate](#rand-crate)
  - [Function `rand::rng()`](#function-randrng)
  - [Module `rand::rngs`](#module-randrngs)
  - [Example](#example)
- [`serde`](#serde)
- [Time](#time)
  - [`std::time`](#stdtime)
    - [Instant](#instant)
    - [`SystemTime`](#systemtime)
    - [`Duration`](#duration)
  - [`chrono`](#chrono)
    - [`Naive*`](#naive)
    - [`DateTime<Tz>`, `Utc`, `Local` and `FixedOffset`](#datetimetz-utc-local-and-fixedoffset)
    - [Time formatting syntax](#time-formatting-syntax)
  - [`chrono_tz`](#chrono_tz)
- [`rayon`](#rayon)
- [Error handling](#error-handling)
  - [`anyhow`](#anyhow)
  - [`thiserror`](#thiserror)
- [`lazy_static` and `once_cell`](#lazy_static-and-once_cell)
  - [`lazy_static`](#lazy_static)
  - [`OnceLock` and `OnceCell`](#oncelock-and-oncecell)
- [Working with user input](#working-with-user-input)
  - [stdin](#stdin)
  - [args](#args)
  - [envs](#envs)
- [reqwest](#reqwest)
  - [Blocking mode](#blocking-mode)
<!-- TOC -->

<br>

# Manage dependencies
There are 2 ways to manage external crates:
- **add**/**del** it to/from `Cargo.toml` file in `[dependencies]` section;
- using `cargo` command: `cargo add [options] dependency`

The `dependency` can be specified with:
- `crate`: fetches crate `crate` of **latest** version from a **registry**;
- `crate@version`: fetches crate `crate` of version `version` from a **registry**;
- `--path path`: fetches from the **specified path**;
- `--git url`: pulls from a **git repo** at url;

<br>

**Some options**:
- `--optional`: **marks** the `dependency` as **optional**;
- `--no-default-features`: **disables** the **default features**;
- `--features features`: space or comma separated list of **features to activate**;

<br>

Example of `cargo add rand` **output**:
```rust
cargo add rand
    Updating crates.io index
      Adding rand v0.10.0 to dependencies
             Features:
             + alloc
             + std
             + std_rng
             + sys_rng
             + thread_rng
             - chacha
             - log
             - serde
             - simd_support
             - unbiased
...
```

<br>

The `Cargo.toml` after `cargo add rand`:
```toml
[dependencies]
rand = "0.10.0"
```

<br>

The *features* that are **enabled** have a `+` to the **left**, and those that **aren’t** enabled have a `-` instead.<br>

<br>

A lot of crates use **features**, which let you compile just a part of the crate. *Some features* are **enabled by default**, but if you want to **add** *more functionality*, you have to **activate** *additional features*.<br>

The `cargo add <crate_name>` command **does not** have a dedicated flag to display **only** the default features of a crate, but it can list **all enabled features** including the default ones using the `--dry-run` (or shorthand `-n`) flag. If crate is already added to `Cargo.toml` and has some additional features, e.g. `reqwest = { version = "0.13.2", features = ["blocking"] }`, all these features will be displayed as `+`. To show **only default features** - **remove** crate by `cargo remove <crate_name>` and then call `cargo add --dry-run <crate_name>`.
```bash
cargo add --dry-run reqwest
    Updating crates.io index
      Adding reqwest v0.13.2 to dependencies
             Features:
             + __rustls
             + __rustls-aws-lc-rs
             + __tls
             + charset
             + default-tls
             + http2
             + rustls
             + system-proxy
             - __native-tls
             - __native-tls-alpn
             - blocking
             - brotli
             - cookies
             - deflate
             - form
             - gzip
             - hickory-dns
             - http3
             - json
             - multipart
             - native-tls
             - native-tls-no-alpn
             - native-tls-vendored
             - native-tls-vendored-no-alpn
             - query
             - rustls-no-provider
             - socks
             - stream
             - zstd
warning: aborting add due to dry run
```

<br>

You can also find out whether a feature is behind a feature flag by looking through the source code for the attribute `#[cfg(feature = "<feature_name>")]`.
Example: [`reqwest` crate](https://docs.rs/reqwest/latest/src/reqwest/lib.rs.html)
```rust
    #[cfg(feature = "blocking")]
    pub mod blocking;
```

<br>

Example of `cargo add  --features 'serde' --no-default-features rand` **output**:
```rust
cargo add  --features 'serde' --no-default-features rand 
    Updating crates.io index
      Adding rand v0.10.0 to dependencies
             Features:
             + serde
             - alloc
             - chacha
             - log
             - simd_support
             - std
             - std_rng
             - sys_rng
             - thread_rng
             - unbiased
```


<br>

The `Cargo.toml` after `cargo add rand --features 'serde'`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"] }
```

<br>

The `Cargo.toml` after `cargo add  --features 'serde' --no-default-features rand`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"], default-features = false }
```

<br>

The `Cargo.toml` after `cargo add  --features 'serde' --no-default-features --optional rand`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"], default-features = false, optional = true }

[features]
rand = ["dep:rand"]
```

<br>

# `rand` crate
**RNG** means **random number generator**.<br>

**Rng** traits:
- `rand::TryRng` is a **base trait** for for **RNGs**;
- `rand::Rng: TryRng` is the **dyn-safe** implementation for **RNG**;
- `rand::RngExt` is an extension trait over `Rng`, i.e. it is a **user-level interface** for **RNGs**;
  - this trait is **not** *dyn compatible*

<br>

Both traits `Rng` and `RngExt` can be brought into scope by `use rand::prelude::*;` or `use rand::{Rng, RngExt};`.<br>

`RngExt` methods:
- `random` returns a **random value** via the **standard uniform** distribution;
- `random_iter` returns an **iterator** over random variates
  - `rng.random_iter().take(5).collect();`
- `random_range` returns a **random value** in the **given range**;
- `random_bool(p: f64)` returns a **bool** with a **probability** `p`;
- `random_ratio(numerator: u32, denominator: u32)` returns a **bool** with a **probability** `numerator/denominator`;

<br>

**Any type** that implements `TryRng` also implements `Rng`:
```rust
impl<R> Rng for R
where
    R: TryRng<Error = Infallible> + ?Sized,
```

<br>

**Any type** that implements `Rng` also implements `RngExt`:
```rust
impl<R: Rng + ?Sized> RngExt for R
```

<br>

## Function `rand::rng()`
A `rand::rng()` returns a **thread-local generator**.<br>
The *thread-local generator* can be obtained via `rand::rng()` or via `ThreadRng::default()`.<br>
The *thread-local generator* **cannot** be passed between threads (is **not** `Send` and not `Sync`).<br>

For example, `ThreadRng` implements `TryRng`:
```rust
impl TryRng for ThreadRng {
    type Error = Infallible;
    ...
}
```

<br>

Thus `ThreadRng` also implements `Rng` and `RngExt` due to blanket implementation.<br>

<br>

## Module `rand::rngs`
[rand::rngs](https://docs.rs/rand/0.10.0/rand/rngs/index.html)

<br>

## Example
`rand::random()` is a convenient alternative to `rand::rng().random()`.<br>


```rust
use rand::{rng};
use rand::prelude::*;

fn main() {
    let mut my_rng = rng();
    
    let b = my_rng.random_ratio(1,2);
    let x: u64 = my_rng.random_range(0..100);
    let y: u16 = my_rng.random();

    let random_u64 = my_rng.next_u64();
    let random_u32 = my_rng.next_u32();

    println!("b={}", b);
    println!("x={}", x);
    println!("y={}", y);
    println!("random_u32={}", random_u32);
    println!("random_u64={}", random_u64);
    
    // rand::random()
    let z: f64 = rand::random();
    println!("z={}", z);    
}
```

**Output**:
```rust
b=false
x=70
y=5405
random_u32=153476317
random_u64=11800991613183827219
z=0.8727143056527192
```

<br>

# `serde`
**Example**:
```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
struct Person {
    id: u64,
    name: String,
    age: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonPost {
    name: String,
    age: u16,
}

impl From<PersonPost> for Person {
    fn from(p: PersonPost) -> Self {
        Self {
            id: 0,
            name: p.name,
            age: p.age
            
        }
    }
}

fn main() {
    //// Case 1 - corect json
    let p = r#"
    {
        "name": "Foo",
        "age": 32 
    }
    "#;
    let r = serde_json::from_str::<PersonPost>(p);
    let Ok(p) = r else {
        eprintln!("Cannot parse json: {:?}", r);
        std::process::exit(1)
    };
    let p: Person = p.into();
    println!("Json: {:?}", p);
    
    //// Case 2 - incorect json
    let p = r#"
    {
        "a": "Foo",
        "age": 32 
    }
    "#;

    let r = serde_json::from_str::<PersonPost>(p);
    let Ok(p) = r else {
        eprintln!("Cannot parse json: {:?}", r);
        std::process::exit(1)
    };
    let p: Person = p.into();
    println!("Json: {:?}", p);
}
```

**Output**:
```rust
Json: Person { id: 0, name: "Foo", age: 32 }
Cannot parse json: Err(Error("missing field `name`", line: 5, column: 5))
```

<br>

# Time
## `std::time`
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

### Instant
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

### `SystemTime`
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

### `Duration`
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


## `chrono`
### `Naive*`
There are some types inside chrono, which start with `Naive`:
- `NaiveTime` - ISO 8601 *time* without timezone;
- `NaiveDate` - ISO 8601 *calendar date* **without** timezone;
- `NaiveDateTime` - ISO 8601 **combined** *date and time* **without** timezone;

<br>

`NaiveTime` **methods**:
- `from_hms_opt(hour, min, sec) -> Option<NaiveTime>` makes a new `NaiveTime` from *hour*, *minute* and *second*;
- `from_hms_nano_opt(hour, min, sec, nano) -> Option<NaiveTime>` makes a new `NaiveTime` from *hour*, *minute*, *second* and *nanosecond*;
- `from_hms_micro_opt(hour, min, sec, micro) -> Option<NaiveTime>` makes a new `NaiveTime` from *hour*, *minute*, *second* and *microsecond*;
- `from_hms_milli_opt(hour, min, sec, milli) -> Option<NaiveTime>` makes a new `NaiveTime` from *hour*, *minute*, *second* and *millisecond*;
- `parse_from_str("23:56:04", "%H:%M:%S") -> ParseResult<NaiveTime>` **parses** a string with the specified *format string* and returns a new `NaiveTime`;
- `.format("%H:%M:%S") -> DelayedFormat<...>` **formats** the **time** with the specified *format string*;

<br>

The ISO 8601 **ordinal date** is a pair of **year number** and **day of the year**. The ordinal number ranges from **1** to **365** or **366** depending on the year.<br>

`NaiveDate` **methods**:
- `parse_from_str("2026-03-03", "%Y-%m-%d") -> ParseResult<NaiveDate>` **parses** a string with the specified *format string* and returns a new `NaiveDate`;
- `from_ymd_opt(year, month, day) -> Option<NaiveDate>` makes a new `NaiveDate` from *year*, *month* and *day*;
- `from_yo_opt(year, ordinal) -> Option<NaiveDate>` makes a new `NaiveDate` from the **ordinal date**;
- `.format("%Y-%m-%d") -> DelayedFormat<...>` **formats** the **date** with the specified *format string*;

To make `NaiveDateTime`:
- `and_time(t)` makes a new `NaiveDateTime` from the current instance of `NaiveDate` and given `t` of `NaiveTime`;
- `and_hms_opt(h, m, s) -> Option<NaiveDateTime>` makes a new `NaiveDateTime` from the current instance of `NaiveDate` and given *hour*, *minute* and *second*;
- `and_hms_nano_opt(h, m, s, ns) -> Option<NaiveDateTime>` makes a new `NaiveDateTime` from the current instance of `NaiveDate` and given *hour*, *minute*, *second* and *nanosecond*;
- `and_hms_micro_opt(h, m, s, us) -> Option<NaiveDateTime>` makes a new `NaiveDateTime` from the current instance of `NaiveDate` and given *hour*, *minute*, *second* and *microsecond*;
- `and_hms_milli_opt(h, m, s, ms) -> Option<NaiveDateTime>` makes a new `NaiveDateTime` from the current instance of `NaiveDate` and given *hour*, *minute*, *second* and *millisecond*;

<br>

`NaiveDateTime` **methods**:
- `new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime` makes a new `NaiveDateTime` from date and time components, **equivalent** to `date.and_time(time)`;
- `parse_from_str("2026-03-03", "%Y-%m-%d")` **parses** a string with the specified *format string* and returns a new `NaiveDateTime`;
- `.format("%Y-%m-%d %H:%M:%S") -> DelayedFormat<...>` **formats** the *combined date and time* with the specified *format string*;
- `.and_utc() -> DateTime<Utc>` converts the `NaiveDateTime` into the **timezone-aware** `DateTime<Utc>`;
- `.and_local_timezone<Tz: TimeZone>(tz: Tz) -> MappedLocalTime<DateTime<Tz>>` converts the `NaiveDateTime` into a **timezone-aware** `DateTime<Tz>` **with** the *provided time zone* `tz`;
- `date(&self) -> NaiveDate` retrieves a **date** component;
- `time(&self) -> NaiveTime` retrieves a **time** component;

<br>

Example:
```rust
use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};
use chrono::{DateTime, FixedOffset, Utc};
use std::time::{Duration, Instant, SystemTime};

fn main() {
    let start = SystemTime::now();

    let date = NaiveDate::from_ymd_opt(2023, 3, 25);
    let time = NaiveTime::from_hms_opt(4, 5, 25);
    let date_and_time = NaiveDate::from_ymd_opt(2023, 3, 25)
        .and_then(|d| d.and_hms_opt(3, 3, 4));

    let dt = NaiveDateTime::new(date.unwrap(), time.unwrap());
    println!("date = {:?}", date);
    println!("time = {:?}", time);
    println!("date_and_time = {:?}", date_and_time);
    println!("dt = {:?}", dt);
}
```

**Output**:
```rust
date = Some(2023-03-25)
time = Some(04:05:25)
date_and_time = Some(2023-03-25T03:03:04)
dt = 2023-03-25T04:05:25
```

<br>

```rust
use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};

fn main() {
    let d = NaiveDate::parse_from_str("2026-03-03", "%Y-%m-%d");
    let t = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S");
    let dt = NaiveDateTime::parse_from_str("2026-03-03 23:56:04", "%Y-%m-%d %H:%M:%S");
    println!("d = {:?}", d);
    println!("t = {:?}", t);
    println!("dt = {:?}", dt);
}
```

<br>

### `DateTime<Tz>`, `Utc`, `Local` and `FixedOffset`
`Utc` **methods**:
- `now() -> DateTime<Utc>` returns a `DateTime<Utc>` which corresponds to the **current date and time** in `UTC`;
```rust
let now_utc = Utc::now();
```

```rust
impl Utc {
    pub fn now() -> DateTime<Utc> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time before Unix epoch");
        DateTime::from_timestamp(now.as_secs() as i64, now.subsec_nanos()).unwrap()
    }
}
```

<br>

Using the `TimeZone` methods on the `Local` struct is the preferred way to construct `DateTime<Local>` instances.<br>
Using the `TimeZone` methods on a `FixedOffset` struct (`FixedOffset` implements `TimeZone`) is the preferred way to construct `DateTime<FixedOffset>` instances.<br>

<br>

`FixedOffset` **methods**:
- `east_opt(secs) -> Option<FixedOffset>` makes a new `FixedOffset` for the **Eastern Hemisphere** with given timezone in seconds `secs`;
  - the **negative** `secs` means the **Western Hemisphere**;
- `west_opt(secs) -> Option<FixedOffset>` makes a new `FixedOffset` for the **Western Hemisphere** with given timezone in seconds `secs`;
  - the **negative** `secs` means the **Eastern Hemisphere**;

<br>

`Local` **methods**:
- `Local::now()` returns a `DateTime<Local>` which corresponds to the **current date**, **time** and **offset** from UTC;

```rust
impl Local {
    pub fn now() -> DateTime<Local> {
        Utc::now().with_timezone(&Local)
    }
}

impl TimeZone for Local {
    type Offset = FixedOffset;
    fn from_offset(_offset: &FixedOffset) -> Local {
        Local
    }
}
```

```rust
// Current local time
let now = Local::now();

// Current local date
let today = now.date_naive();

// Current local time, converted to `DateTime<FixedOffset>`
let now_fixed_offset = Local::now().fixed_offset();
// or
let now_fixed_offset: DateTime<FixedOffset> = Local::now().into();

// Current time in some timezone (let's use +05:00)
// Note that it is usually more efficient to use `Utc::now` for this use case.
let offset = FixedOffset::east_opt(5 * 60 * 60).unwrap();
let now_with_offset = Local::now().with_timezone(&offset);
```

`TimeZone` methods:
- `with_ymd_and_hms(year, month, day, hour, min, sec)` make a new `DateTime` from year, month, day, time components and current time zone;
- `timestamp_opt(secs, nsecs)` makes a new `DateTime` from the **timestamp**: the number of seconds and nanoseconds since `January 1, 1970 0:00:00 UTC`;
- `from_utc_datetime(utc: &NaiveDateTime) -> DateTime<Self>` converts the **UTC** `NaiveDateTime` to the **local time**;
- `from_local_datetime(local: &NaiveDateTime) -> MappedLocalTime<DateTime<Self>>` converts the **local** `NaiveDateTime` to the **timezone-aware** `DateTime` if possible;

<br>

`DateTime<Tz>` internally stores the **date and time** in **UTC** with a `NaiveDateTime`.<br>

`DateTime<...>` methods:
- `date_naive()` retrieves the **date** component **with** an associated *timezone*;
- `time() -> NaiveTime` retrieves the **time** component;
- `.format("%Y-%m-%d %H:%M:%S") -> DelayedFormat<...>` **formats** the *combined date and time* with the specified *format string*;
- `from_naive_utc_and_offset(naive_utc, offset) -> DateTime<Tz>` makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`;
  - this is a **low-level** method;
  - use `TimeZone::from_local_datetime()` or `NaiveDateTime::and_local_timezone()` instead;
- `offset(&self) -> &Tz::Offset` retrieves an associated **offset** from UTC;
- `timezone(&self) -> Tz` retrieves an associated **time zone**;
- `with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> DateTime<Tz2>` **changes** the associated **time zone**, the returned `DateTime` references the same instant of time from the perspective of the provided time zone `tz`;
- `to_utc(&self) -> DateTime<Utc>` turn this `DateTime` into a `DateTime<Utc>`, **dropping** the offset and associated timezone information;
- `from_timestamp(secs: i64, nsecs: u32) -> Option<DateTime<Utc>>` makes a new `DateTime<Utc>` from the **timestamp**: the number of seconds and nanoseconds since `January 1, 1970 0:00:00 UTC`;
- `parse_from_str(s: &str, fmt: &str) -> ParseResult<DateTime<FixedOffset>>` **parses** a string from a user-specified format into a `DateTime<FixedOffset>` value;
  - that this method **requires** a timezone in the input string;
- `from_str(s: &str) -> ParseResult<DateTime<FixedOffset>>` requires `use std::str::FromStr` because `impl FromStr for DateTime<FixedOffset>`
  - accepts a **relaxed** form of **RFC3339**;

```rust
impl DateTime<Utc> {
    pub const fn from_timestamp(secs: i64, nsecs: u32) -> Option<Self> {
        let days = secs.div_euclid(86_400) + UNIX_EPOCH_DAY;
        let secs = secs.rem_euclid(86_400);
        if days < i32::MIN as i64 || days > i32::MAX as i64 {
            return None;
        }
        let date = try_opt!(NaiveDate::from_num_days_from_ce_opt(days as i32));
        let time = try_opt!(NaiveTime::from_num_seconds_from_midnight_opt(secs as u32, nsecs));
        Some(date.and_time(time).and_utc())
    }
}
```

<br>

**Example**:
```rust
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{naive::{NaiveDate, NaiveDateTime, NaiveTime}, DateTime, FixedOffset, Local, Utc};

fn main() {
    let tz: FixedOffset = FixedOffset::east_opt(3*60*60).unwrap();
    println!("tz = {:?}", tz);

    let ts: std::time::Duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time before Unix epoch");
    let ts_utc: DateTime<Utc> = DateTime::from_timestamp(ts.as_secs() as i64, ts.subsec_nanos()).unwrap();

    let dt_utc2: DateTime<Utc> = DateTime::from_naive_utc_and_offset(ts_utc.naive_local(), Utc);
    let dt_utc3: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(ts_utc.naive_local(), tz);

    println!("\nts_utc.naive_local() = {}", ts_utc.naive_local());
    println!("ts_utc.naive_utc() = {}", ts_utc.naive_utc());

    println!("\ndt_utc2 = {}", dt_utc2);
    println!("dt_utc3 = {}", dt_utc3);

    let utc_now: DateTime<Utc> = Utc::now();
    let dt_local: DateTime<Local> = Local::now();
    let dt_naive_utc: NaiveDateTime = dt_local.naive_utc();

    let dt_with_offset: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(dt_naive_utc, tz);
    let now_with_offset: DateTime<FixedOffset> = utc_now.with_timezone(&tz);

    println!("\nutc_now = {:?}", utc_now);
    println!("dt_local = {:?}", dt_local);
    println!("dt_naive_utc = dt_local.naive_utc() = {:?}", dt_naive_utc);
    println!("DateTime::from_naive_utc_and_offset(dt_naive_utc, tz) = {:?}", dt_with_offset);
    println!("utc_now.with_timezone(&tz) = {:?}", now_with_offset);
    

    println!("\nutc_now (%Y-%m-%d %H:%M:%S %z) = {}", utc_now.format("%Y-%m-%d %H:%M:%S %z"));
    println!("utc_now (%Y-%m-%d %H:%M:%S %Z) = {}", utc_now.format("%Y-%m-%d %H:%M:%S %Z"));

    println!("utc_now (%Y-%m-%d %H:%M:%S %z) = {}", dt_with_offset.format("%Y-%m-%d %H:%M:%S %z"));
    println!("utc_now (%Y-%m-%d %H:%M:%S %Z) = {}", dt_with_offset.format("%Y-%m-%d %H:%M:%S %Z"));

    println!("\nutc_now (in RFC 3339 format) = {}", utc_now.to_rfc3339());
    println!("dt_with_offset (in RFC 3339 format) = {}", dt_with_offset.to_rfc3339());
}
```

**Output**:
```rust
tz = +03:00

ts_utc.naive_local() = 2026-03-03 19:26:28.806244
ts_utc.naive_utc() = 2026-03-03 19:26:28.806244

dt_utc2 = 2026-03-03 19:26:28.806244 UTC
dt_utc3 = 2026-03-03 22:26:28.806244 +03:00

utc_now = 2026-03-03T19:26:28.806251Z
dt_local = 2026-03-03T22:26:28.806251+03:00
dt_naive_utc = dt_local.naive_utc() = 2026-03-03T19:26:28.806251
DateTime::from_naive_utc_and_offset(dt_naive_utc, tz) = 2026-03-03T22:26:28.806251+03:00
utc_now.with_timezone(&tz) = 2026-03-03T22:26:28.806251+03:00

utc_now (%Y-%m-%d %H:%M:%S %z) = 2026-03-03 19:26:28 +0000
utc_now (%Y-%m-%d %H:%M:%S %Z) = 2026-03-03 19:26:28 UTC
utc_now (%Y-%m-%d %H:%M:%S %z) = 2026-03-03 22:26:28 +0300
utc_now (%Y-%m-%d %H:%M:%S %Z) = 2026-03-03 22:26:28 +03:00

utc_now (in RFC 3339 format) = 2026-03-03T19:26:28.806251+00:00
dt_with_offset (in RFC 3339 format) = 2026-03-03T22:26:28.806251+03:00
```

<br>

### Time formatting syntax
A `chrono::format::DelayedFormat` is a temporary object which can be used as an argument to `format!`/`println!` and other **formatting macros**. In this way it **avoids** the redundant **memory allocation**. A `DelayedFormat` also has `to_string` method to get a `String`.<br>

[Time formatting syntax](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) for printing and parsing.<br>

The format specifier `%z` prints **offset** from the local time to UTC in the format `+0000`.<br>
The format specifier `%Z` prints **offset** from the local time to UTC in the format `+03:00`, but for **Utc** prints `UTC`.<br>

**Example**:
```rust
use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};

fn main() {
    let t = NaiveTime::from_hms_opt(23, 56, 4).unwrap();

    //// 
    let s1 = t.format("%H:%M:%S").to_string();
    let s2 = t.format("%H:%M:%S%.6f").to_string();
    let s3 = t.format("%-I:%M %p").to_string();
    println!("%H:%M:%S - {}", s1);
    println!("%H:%M:%S%.6f - {}", s2);
    println!("%-I:%M %p - {}", s3);

    //// 
    println!("%H:%M:%S - {}", t.format("%H:%M:%S"));
    println!("%H:%M:%S%.6f - {}", t.format("%H:%M:%S%.6f"));
    println!("%-I:%M %p - {}", t.format("%-I:%M %p"));
}
```

**Output**:
```bash
%H:%M:%S - 23:56:04
%H:%M:%S%.6f - 23:56:04.000000
%-I:%M %p - 11:56 PM
%H:%M:%S - 23:56:04
%H:%M:%S%.6f - 23:56:04.000000
%-I:%M %p - 11:56 PM
```

<br>

## `chrono_tz`
`chrono_tz` is a library that provides implementors of the `TimeZone` trait for `chrono`.<br>

```rust
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{TimeZone, naive::{NaiveDate, NaiveDateTime, NaiveTime}, DateTime, FixedOffset, Local, Utc};
use chrono_tz::Europe::Moscow;
use chrono_tz::America::New_York;

fn main() {
    let utc_now: DateTime<Utc> = Utc::now();

    let msk_now: DateTime<chrono_tz::Tz> = Moscow.from_utc_datetime(&utc_now.naive_utc());
    let nyc_time: DateTime<chrono_tz::Tz> = utc_now.with_timezone(&New_York);

    println!("utc_now = {}", utc_now.to_rfc3339());
    println!("msk_now = {}", msk_now.to_rfc3339());
    println!("nyc_time = {}", nyc_time.to_rfc3339());

    let msk_time: DateTime<chrono_tz::Tz> = Moscow.with_ymd_and_hms(2016, 3, 18, 3, 0, 0).unwrap();
    println!("\nmsk_time (custom with_ymd_and_hms()) = {}", msk_time.to_rfc3339());
}
```

**Output**:
```rust
utc_now = 2026-03-03T20:02:55.164497+00:00
msk_now = 2026-03-03T23:02:55.164497+03:00
nyc_time = 2026-03-03T15:02:55.164497-05:00

msk_time (custom with_ymd_and_hms()) = 2016-03-18T03:00:00+03:00
```

<br>

# `rayon`
Rayon is a crate that lets you **automatically** spawn multiple threads when working with iterators and related types. Instead of using `thread::spawn()` to spawn threads, you can just add `par_` to the **iterator methods** you already know (``).

The speedup that Rayon gives will depend a lot on your code and the number of threads on your computer.<br>

First, we will use a method `std::thread::available_parallelism()` to see how many threads are available.<br>

<br>

**Example**:
```rust
fn main() {
    let n = std::thread::available_parallelism();

    println!("Available threads: {:?}, the number of threads will be spawned: {:?}", n, rayon::current_num_threads());

    let my_vec = vec![0; 4_000_000_000];
    let start1 = std::time::Instant::now();
    let r= my_vec
        .iter()
        .enumerate()
        .fold(0, |acc, item| acc + item.0);
    println!("Without rayon result = {}, time = {:?}", r, start1.elapsed());

    let my_vec = vec![0; 4_000_000_000];
    let start2 = std::time::Instant::now();
    let r= my_vec
        .par_iter()
        .enumerate()
        .fold(|| 0, |acc, item| acc + item.0).sum::<usize>();
    println!("With rayon result = {}, time = {:?}", r, start2.elapsed());
}
```

**Output**:
```bash
cargo run --release
Available threads: Ok(10), the number of threads will be spawned: 10
Without rayon result = 7999999998000000000, time = 42ns
With rayon result = 7999999998000000000, time = 488.417µs
```

```bash
cargo run
Available threads: Ok(10), the number of threads will be spawned: 10
Without rayon result = 7999999998000000000, time = 1.282937166s
With rayon result = 7999999998000000000, time = 225.613292ms
```

<br>

# Error handling
## `anyhow`
It would be nice to have a **single error type** that’s easy to use. This is what `anyhow` is used for.<br>
Another common way to do this is to use `Box<dyn Error>`.<br>
Anyhow works with **any error type** that has an impl of `std::error::Error`.<br>

<br>

Use `fn foo () -> Result<T, anyhow::Error> {}` or **equivalently** `fn foo () -> anyhow::Result<T> {}`, as the return type of any **fallible function**:
```rust
pub fn parse_u16_1(input: &[u8]) -> anyhow::Result<u16> {
    Ok(std::str::from_utf8(input)?
        .parse::<u16>()?)
}

pub fn parse_u16_2(input: &[u8]) -> Result<u16, anyhow::Error> {
    Ok(std::str::from_utf8(input)?
        .parse::<u16>()?)
}

fn main() {
    println!("{:?}", parse_u16_1("abc".as_bytes()));
    println!("{:?}", parse_u16_1("444".as_bytes()));
    println!("{:?}", parse_u16_2("444444".as_bytes()));
}
```

**Output**:
```rust
Err(invalid digit found in string)
Ok(444)
Err(number too large to fit in target type)
```

<br>

We can also bring in the `anyhow!` **macro**, which makes a quick `anyhow::Error` from a **string** or an **error type**:
```rust
#[derive(Debug)]
pub enum MyError {
    A,
    B
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::A => write!(f, "MyError::A error"),
            MyError::B => write!(f, "MyError::B error"),
        }
    }
}

pub fn ret_myerr(err: MyError) -> anyhow::Result<u16> {
    Err(anyhow::anyhow!(err))
}

pub fn ret_str_err(msg: &str) -> anyhow::Result<u16> {
    Err(anyhow::anyhow!(msg.to_owned()))
}

fn main() {
    println!("{:?}", ret_myerr(MyError::A));
    println!("{:?}", ret_str_err("Some error string"));
}
```

<br>

## `thiserror`
You use `#[derive(Error)]` on top and then another `#[error]` attribute above each variant if we want a message. This will **automatically implement** `Display`.<br>

**Note**, the `error` attribute has the same format as when you use the `format!` macro.<br>

**Example**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum SystemError {
    #[error("Got error: {0}")]
    A(String),
    #[error("Wrong number: {0}")]
    B(u8),
}

fn main() {
    println!("{}", SystemError::B(8));
    println!("{}", SystemError::A("foo".to_owned()));
}
```

**Output**:
```bash
Wrong number: 8
Got error: foo
```

<br>

# `lazy_static` and `once_cell`
Since Rust 1.63 the following code became possible:
```rust
use std::sync::Mutex;

static GLOBAL_VEC: Mutex<Vec<u8>> = Mutex::new(Vec::new());
```

However, there are still a lot of other static variables you might want to have but **can’t** initialize with a `const fn`:
```rust
static v: Vec<u8> = vec![1,2,3]; // ❌ error[E0010]: allocations are not allowed in statics
```

But Rust provides means for that: `lazy_static` and `once_cell` both allow you to *initialize statics* **at runtime**.<br>

<br>

## `lazy_static`
The `lazy_static` crate provides `lazy_static!` which uses folloing syntax `static ref <NAME>: <TYPE> = <EXPR>;` to declare `static`.<br>

Consider, you want to declare **non-empty** vector `v` as `static` and then modify it, the syntax will:
```rust
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref v: Mutex<Vec<u8>> = Mutex::new(vec![1,2,3]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(v.lock()?.push(33))
}
```

<br>

## `OnceLock` and `OnceCell`
*Static variables* **must be thread-safe** (implement the `Sync` trait) to prevent data races, as the compiler assumes they might be accessed from multiple threads. So, it is **not possible** to use `std::cell::OnceCell` for a *static variable* in a multi-threaded context because `OnceCell` is **not thread-safe**. Instead, you should use the t**hread-safe counterpart**, `std::sync::OnceLock`.<br>

**Example**:
```rust
use std::error::Error;
use std::{cell::OnceCell, sync::Mutex};
use std::sync::OnceLock;

static V1: OnceLock<Vec<u8>> = OnceLock::new();
static V2: Mutex<OnceCell<Vec<u8>>> = Mutex::new(OnceCell::new());

fn main() -> Result<(), Box<dyn Error>> {
    V1.set(vec![1,2,3]).unwrap();
    println!("{:?}", V1);

    let r = V2.lock().unwrap().set(vec![1,2,3]).unwrap();
    println!("{:?}", V2);
    Ok(())
}
```

**Output**:
```bash
OnceLock([1, 2, 3])
Mutex { data: OnceCell([1, 2, 3]), poisoned: false, .. }
```

<br>

What makes a `OnceCell` **more flexible** than `lazy_static`:
- a `OnceCell` can hold a whole type, or it can be a parameter inside another type;
- a `OnceCell` can be used for values that unknown until runtime;
  - for example, we can start `main()`, get the **user input** and then pass it to `OnceCell` using `.set()`;
- for a `OnceCell`, you can choose a **sync** or **unsync** version:
  - `std::cell::OnceCell` is a **cell** which can be written to **only once**, but it is **thread-unsafe**;
  - `std::sync::OnceLock` a **thread-safe** version of `OnceCell`;

<br>

# Working with user input
## stdin
The function `std::io::stdin()` returns `std::io::Stdin` struct, which handles **user input** and has a various methods:
- `.read_line()` reads the input to a `&mut String`;

<br>

**Example**:
```rust
use std::io;

fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        ;
    }
    println!(r#"You wrote "{}", exit"#, input_string.trim());
}
```

**Output**:
```bash
Please type something, or x to escape:
aa
bb
x
You wrote "x", exit
```

## args
The function `std::env::args()` returns `std::env::Args` struct. This `Args` struct holds what the user types when starting the program, known as **command-line arguments**.<br>

**Example**:
```rust
use std::env::args;

fn main() {
    let input = args();
    for (i, entry) in input.enumerate() {
        println!("arg #{}: {}", i, entry);
    }
}
```

**Output**:
```bash
cargo run a b c
arg #0: target/debug/example
arg #1: a
arg #2: b
arg #3: c
```

The **first argument** (**zero index**) is always the **path** to the binary file.<br>
The main crate used by Rust users to work with command-line arguments is known as `clap` (**CLAP** = Command Line Argument Parser).<br>

<br>

## envs
The function `std::env::vars()` returns `std::env::Vars` struct. This `Vars` struct holds all environment variables.<br>

Example - list all environment variables:
```rust
fn main() {
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}
```

<br>

**Functions**:
- `std::env::set_var(env, value)` sets value `value` for environment variables `env`;
- `std::env::var(env)` reads value of environment variables `env`;

<br>

Most crates in Rust use the `RUST_LOG` environment variable to set **severity** for logging:

```rust
fn main() {
    match std::env::var("RUST_LOG") {
        Ok(severity) => println!("log severity level is {severity}"),
        Err(_) => {
            unsafe {std::env::set_var("RUST_LOG", "DEBUG");}
            println!("default log severity level is {}", std::env::var("RUST_LOG").unwrap());
        },
    }
}
```

<br>

# reqwest
## Blocking mode
To enable blocking client run `cargo add  reqwest --features blocking`.<br>

The instance of `Client` has **http methods**:
- `.post()`;
- `.put()`;
- `.get()`;
- `.delete()`;

<br>

The `.get()` method is pretty simple:
```rust
pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder
```

This `IntoUrl` trait is one that the reqwest crate made, not the standard library, so you don’t have to remember it. But you can guess from the name that `IntoUrl` means anything that can become a URL, and it’s implemented for both `&str` and `String`.

<br>

**Example**:
```rust
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let resp = client.get("https://www.rust-lang.org").send().unwrap();
    println!("{:#?}", resp);
    println!("{}", &resp.text().unwrap()[0..200]);
}
```

**Output**:
```bash
Response {
    url: "https://rust-lang.org/",
    status: 200,
    headers: {
        "server": "GitHub.com",
        "content-type": "text/html; charset=utf-8",
        "x-origin-cache": "HIT",
        "last-modified": "Wed, 11 Mar 2026 01:43:58 GMT",
        "access-control-allow-origin": "*",
        "etag": "\"69b0c8de-48a9\"",
        "expires": "Wed, 11 Mar 2026 13:48:29 GMT",
        "cache-control": "max-age=600",
        "x-proxy-cache": "MISS",
        "x-github-request-id": "ADFE:25B76B:168270:16C29C:69B17055",
        "accept-ranges": "bytes",
        "age": "0",
        "date": "Wed, 11 Mar 2026 14:29:28 GMT",
        "via": "1.1 varnish",
        "x-served-by": "cache-ams2100130-AMS",
        "x-cache": "HIT",
        "x-cache-hits": "0",
        "x-timer": "S1773239369.631582,VS0,VE122",
        "vary": "Accept-Encoding",
        "x-fastly-request-id": "71d736843c4c7171f06a73f48be8d9c7674958cc",
        "content-length": "18601",
    },
}
<!doctype html>
<html lang="en-US">
  <head>
    <meta charset="utf-8">
    <title>
            Rust Programming Language
        </title>
    <meta name="viewport" content="width=device-width,initial
```

<br>

The [`Response` struct](https://docs.rs/reqwest/latest/reqwest/struct.Response.html) has methods
- `.status()`
- `.content_length()`
- `.text()`: it gives a `Result<String>`, where `String` is a **body** of response;
