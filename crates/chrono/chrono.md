# Table of contents
- [Table of contents](#table-of-contents)
- [`std::time`](#stdtime)
  - [Instant](#instant)
  - [`SystemTime`](#systemtime)
  - [`Duration`](#duration)
- [`chrono`](#chrono)
  - [`Naive*`](#naive)
  - [`DateTime<Tz>`, `Utc`, `Local` and `FixedOffset`](#datetimetz-utc-local-and-fixedoffset)
  - [Time formatting syntax](#time-formatting-syntax)
  - [`chrono_tz`](#chrono_tz)

<br>


# `chrono`
## `Naive*`
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

## `DateTime<Tz>`, `Utc`, `Local` and `FixedOffset`
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

## Time formatting syntax
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