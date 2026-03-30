# Table of contents
- [Table of contents](#table-of-contents)
- [Leap seconds](#leap-seconds)
- [POSIX time](#posix-time)
- [`gettimeofday()` vs. `clock_gettime()`](#gettimeofday-vs-clock_gettime)
- [Differences between `ISO 8601` and `RFC 3339`](#differences-between-iso-8601-and-rfc-3339)

<br>

# Leap seconds
[Leap seconds](https://en.wikipedia.org/wiki/Leap_second)

<br>

There are 3 type of day:
- a **sidereal day** /saɪˈdɪə.ri.əl/ is the Earth's **true 360-degree rotation period** relative to fixed stars, lasting **23** hours, **56** minutes, and **4** seconds;
- a **solar day** is the time it takes for the Sun to return to the same meridian, lasting **24 hours**. The **4-minute difference** occurs because Earth moves along its orbit, requiring **extra rotation** to face the Sun again;
- the **SI day** (*SI* stands for the international system of units) is exactly **86,400** seconds;

<br>

There are 3 time scales:
- **UT1**
- **TAI**
- **UTC**

**UT1** (*Universal Time*) is a time based on the *Earth's rotation around itself* relative to the Sun, so the **UT1** is an astronomical time and it measures the **true length of the solar day**.<br>

The **UT1** time is **irregular** иecause the Earth's rotation speed around itself is variable and is generally slowing down. The **IERS** (*International Earth Rotation Service*) **monitors** the Earth's rotation parameters, **calculates** the value of **UT1** and **publishes** it.<br>

**TAI** (*International Atomic Time*, from French *Temps Atomique International*) is a high-precision, a pure linear time scale based on a weighted average of over 450 atomic clocks all over the world.<br>

**UTC** (*Coordinated Universal Time*) is a **civil time** based on **TAI**: since 1972, **UTC** has been defined as **offset** from **TAI** by an integer number of seconds. The **initial difference** between **TAI** and **UTC** was `10` seconds.<br>

**GMT** (*Greenwich Mean Time*) is another name for the **UTC time zone** `+00:00`.<br>

<br>

A *solar day* is **3 milliseconds longer** than the *SI day* that's why **UT1** lags behind **UTC**, in other words difference between **UT1** and **UTC** is accumulated continuously. **Leap seconds** were introduced to keep the difference between **UT1** and **UTC** within **±0.9s**, in other words **leap seconds** keep **UTC** aligned with the Earth's rotation.<br>

A **positive** *leap second* is *one-second adjustments added* to UTC.<br>
A **negative** *leap second* is *one-second adjustments subtracted* from UTC, **negative** *leap seconds* will probably never happen.<br>

The decision on when to insert a *leap second* is made by the **IERS**. The **IERS** **monitors** the earth rotation parameters, **calculates** the value of **UT1** and **publishes** it. A *leap second* has been inserted **about** every **15** months. The **first** *leap second* was inserted into the **UTC** on **June 30, 1972**. The **last** *leap second* was inserted into the **UTC** on **December 31, 2016**. There have been **27 positive** *leap seconds* added since 1972.  **No negative** *leap second* has ever been inserted.<br>

[All added leap seconds](https://data.iana.org/time-zones/data/leap-seconds.list).<br>

The **current difference** (2026) between **UTC** and **TAI**: `TAI = UTC + 10 + 27`, **TAI** is **ahead of** **UTC** by **37** seconds.<br>

<br>

So,
- **leap years** keep calendar synchronized with the **rotation of the Earth around the Sun**;
- **leap seconds** keep clocks synchronized with the **rotation of the Earth around itself relative to the Sun**;

<br>

# POSIX time
**Unix time** (aka **POSIX time**) is defined as the **number of seconds** since the **Unix epoch** `1970-01-01 00:00:00 UTC`, but **without** *leap seconds*.<br>
**UTC** **includes** *leap seconds*. However, in *POSIX time* *leap seconds* are **ignored** to provide an easy and compatible method of computing time differences.<br>

The fact that *Unix time* **ignores** *leap seconds* means that *Unix time* **does not** have any way to represent the *leap second* in the form of **23:59:60**.<br>
When a UTC leap second occurs (an extra second, e.g., 23:59:60), the Unix clock typically repeats the same second **59** **twice**, meaning the **timestamp remains the same for two seconds**.<br>

In practical terms, that means that *Unix time* **progressed like this**:
- `2016-12-31 23:59:59 UTC` -> `1483217999`
- `2016-12-31 23:59:60 UTC` -> `1483218000`
- `2017-01-01 00:00:00 UTC` -> `1483218000`
- `2017-01-01 00:00:01 UTC` -> `1483218001`

<br>

And that means that *Unix time* is **ambiguous** with respect to **leap seconds**. In the above example, the *unix timestamp* `1483218000` could correspond:
- to the **leap second itself** `2016-12-31 23:59:60`
- to the **next second** `2017-01-01 00:00:00`

Unix `date` util options:
- `-u` for UTC output;
- `-r` for input as seconds since epoch;

<br>

- from **broken-down** to **epoch**:
```bash
date -j -f "%Y-%m-%d %H:%M:%S" "2016-12-31 23:59:59" +%s
1483217999

date -j -f "%Y-%m-%d %H:%M:%S" "2016-12-31 23:59:60" +%s
1483218000

date -j -f "%Y-%m-%d %H:%M:%S" "2017-01-01 00:00:00" +%s
1483218000

date -j -f "%Y-%m-%d %H:%M:%S" "2017-01-01 00:00:01" +%s
1483218001
```
- from the **epoch** to **broken-down** via `mktime(3)`:
```bash
date -ur 1483217999 "+%Y-%m-%d %H:%M:%S"
2016-12-31 20:59:59

date -ur 1483218000 "+%Y-%m-%d %H:%M:%S"
2016-12-31 21:00:00

date -ur 1483218001 "+%Y-%m-%d %H:%M:%S"
2016-12-31 21:00:01
```

<br>

A *calendar date and time* expressed in *Unix time* as a **single elapsed value** like seconds since an epoch is often called a **Unix timestamp**, (aka **epoch time**, **timestamp**, **epoch**).<br>
**Broken-down time** refers to a **calendar date and time** separated into **human-readable** components (*year*, *month*, *day*, *hour*, *minute*, *second*, .etc), as opposed to a **Unix timestamp**.<br>

**Broken-down time** is represented by `struct tm`:
```C
struct tm {
  int  tm_sec;    /* Seconds          [0, 60] */
  int  tm_min;    /* Minutes          [0, 59] */
  int  tm_hour;   /* Hour             [0, 23] */
  int  tm_mday;   /* Day of the month [1, 31] */
  int  tm_mon;    /* Month            [0, 11]  (January = 0) */
  int  tm_year;   /* Year minus 1900 */
  int  tm_wday;   /* Day of the week  [0, 6]   (Sunday = 0) */
  int  tm_yday;   /* Day of the year  [0, 365] (Jan/01 = 0) */
  int  tm_isdst;  /* Daylight savings flag */
  long tm_gmtoff; /* Seconds East of UTC */
  const char *tm_zone;   /* Timezone abbreviation */
};
```

**Every day** in *unix time* consists of **exactly** *86400* seconds. But to convert *unix timestamp* to **calendar date and time** it is not enough to divide the timestamp by **86,400**, you must *take into consideration* **leap years**, **leap seconds**, **time zones** and **seasonal time adjustments**, e.g. **DST** (**Daylight Saving Time**).<br>

<br>

- **DST** is an adjustment of the timezone by usually **one hour**;
- **DST** rules are determined by local law and can change from year to year;

<br>

`libc` functions:
- the `gmtime()` function converts the **epoch** time to the **broken-down** time, expressed as **UTC**;
- the  `localtime()` function converts the **epoch** time to the **broken-down** time, expressed as a local time;
  - the function **corrects** for the **timezone** and **DST**;
- the `mktime()` function converts the **broken-down** time, expressed as **local time**, into the **epoch** time;

<br>

# `gettimeofday()` vs. `clock_gettime()`
In Linux, the `clock_gettime()` returns the current time of a **specified clock**. 

Syscalls to get time:
- `gettimeofday()`:
  - resolution: **microseconds**;
  - represents the amount of time (in seconds and microseconds) since the Unix epoch;
  - the **POSIX.1-2008** marks `gettimeofday()` as **obsolete**, recommending the use of `clock_gettime()` instead;
- `clock_gettime()`:
  - resolution: **nanoseconds**, depending on the clock source;
  - supports several clock sources:
    - `CLOCK_REALTIME`
      - represents the *unix time*: number of seconds seconds elapsed since the Unix epoch, **ignoring** *leap seconds*;
      - this clock is **settable**: it **can** be changed **after** system startup:
        - for this clock **both** `clock_gettime()` and `clock_settime()` are available;
        - it can be changed by NTP or system administrator, and **can** move **forward** or **backward**;
    - `CLOCK_MONOTONIC`
      - this clock is **nonsettable**: it **cannot** be changed **after** system startup:
        - for this clock **only** `clock_gettime()` is available;
        - it is **monotonically** increasing clock that measures time from system start-up time that **does not change** after system startup;
        - it never goes **forward** or **backward**;
      - when a process calls a `sleep()` function, it yields its CPU time to other processes, so the system as a whole remains active, and the `CLOCK_MONOTONIC` continues to increment normally, measuring the actual voluntarily that has passed;
      - it **does not** measure the time when the **entire system is suspended** (e.g., laptop lid closed, hibernation, etc.);
    - `CLOCK_BOOTTIME`
      - similar to `CLOCK_MONOTONIC`, but includes time spent while the **whole system is suspended**;
      - this is useful for applications that need to be **suspend-aware**;
    - `CLOCK_PROCESS_CPUTIME_ID`
      - per-process CPU-time clock;
    - `CLOCK_THREAD_CPUTIME_ID`
      - per-thread CPU-time clock;

<br>

# Differences between `ISO 8601` and `RFC 3339`
[`RFC 3339` vs. `ISO 8601`](https://ijmacd.github.io/rfc3339-iso8601/)

Key differences between `ISO 8601` and `RFC 3339`:
- `ISO 8601` is a **paid standard**, while `RFC 3339` is **freely available**;
- `ISO 8601` requires a `T` to separate date and time (e.g. `2026-03-04T12:00:00Z`)
- `RFC 3339` allows both a space and `T` (e.g. `2026-03-04 12:00:00Z` and `2026-03-04T12:00:00Z`);
- `ISO 8601` allows **2-digit** years; `RFC 3339` requires **4-digit** years;
- both support `Z` as aliase to **UTC timezone**: `+00:00`;
- `RFC 3339` is **case-insensitive **so every `T` or `Z` could also be `t` or `z` respectively; 
- `RFC 3339` explicitly allows `-00:00` to indicate **unknown timezone** which is not allowed in standard `ISO 8601`;
- `RFC 3339` requires the use of a **period** `.` for **fractional seconds**, while `ISO 8601` permits **comma** `,`;
