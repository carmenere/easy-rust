# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Common crates](#common-crates)
* [`tracing` crate](#tracing-crate)
  * [Usage](#usage)
    * [In libraries](#in-libraries)
    * [In executables](#in-executables)
  * [Spans](#spans)
  * [Events](#events)
* [`tracing-subscriber` crate](#tracing-subscriber-crate)
<!-- TOC -->

<br>

# Common crates
- [**tracing**](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html) is a **framework** for **instrumenting** Rust programs to collect traces;
- [**tracing-subscriber**](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html): provides utilities for implementing and configuring **subscribers**;
- [**tracing-flame**](https://crates.io/crates/tracing-flame): for generating **flamegraphs** and **flamecharts** with [**inferno**](https://crates.io/crates/inferno);
- [**tracing-opentelemetry**](https://crates.io/crates/tracing-opentelemetry): provides compatibility with **OpenTelemetry**;
- [**tracing-futures**](https://crates.io/crates/tracing-futures): provides support for instrumenting **asynchronous code** written using **futures** and **async**/**await**;
- [**tracing-log**](https://crates.io/crates/tracing-log): provides a **compatibility** layer with the `log` crate, i.e. it allows a tracing `Subscriber` to consume **log records** of `log` crate as though they were `tracing` events;

<br>

# `tracing` crate
The core of **tracing’s API** is composed of:
- [**spans**](https://docs.rs/tracing/latest/tracing/span/index.html): unlike a _log_ that represents a _moment in time_, a **span** represents a **period of time** with a beginning and an end;
- [**events**](https://docs.rs/tracing/latest/tracing/struct.Event.html): **like** a _log_ **event** represents **moment in time**, but **unlike** a _log_, an **event** exists within the **context** of a span;
- [**subscribers**](https://docs.rs/tracing/latest/tracing/trait.Subscriber.html): **subscriber** implement trait `Subscriber` which provides methods for collecting or recording trace data;

<br>

## Usage
### In libraries
Libraries should link only to the `tracing` crate, and use the provided macros to record whatever information will be useful to downstream consumers.<br>

<br>

### In executables
In order to record trace events, executables have to use a `Subscriber` implementation compatible with `tracing`.<br>
A `Subscriber` implements a way of collecting **trace data**, such as by logging it to standard output.<br>

The simplest way to use a subscriber is to call the set_global_default function:
```rust
let my_subscriber = ... ;

tracing::subscriber::set_global_default(my_subscriber)
.expect("setting tracing default failed");
```

The `set_global_default` sets this subscriber as the global default for the duration of the entire program.<br>
**Warning**: libraries should not call set_global_default()!<br>

<br>

## Spans
A **span** consists of fields, **set of fixed attributes** and **set of arbitrary user-defined key-value pairs**.
Attributes describing spans include:
- an **id** assigned by the subscriber that uniquely identifies it in relation to other spans;
- the **span’s parent** in the **trace tree**;
- [**metadata**](https://docs.rs/tracing/latest/tracing/struct.Metadata.html);

<br>

**Spans** form a **tree structure** — unless it is a root span, all spans have a parent, and may have one or more children. When a **new** span is created, the **current** span **becomes** the new span’s **parent**.<br>
Thus, a parent span always lasts for at least as long as the longest-executing span in its subtree.<br>
Execution may **enter** and **exit** a span _multiple times_ before that span is **closed**.<br>

**Spans** are created using the `span!` macro. Parameters of `span!` macro:

| Parameter | Optional | Input type                        | Default                                      |
|:----------|:---------|:----------------------------------|:---------------------------------------------|
| Target    | Yes      | `&str`                            | The **module path** where the macro was invoked  |
| Parent    | Yes      | parent_span_id                    | The **current span** where the macro was invoked |
| **Level** | **No**   | Any `tracing::Level` enum variant |                                              |
| Span name | No       | `&str`                            |                                              |

A parameter `Level` specifies the **verbosity level** of the **span**.<br>

<br>

There are **set of macros** for creating `Span`: `trace_span!`, `debug_span!`, `info_span!`, `warn_span!`, `error_span!`.They behave similarly to the `span!` macro, but with the `Level` argument with appropriate value.<br>

<br>

**Example**:
```rust
span!(Level::TRACE, "my_span");
let span = span!(Level::TRACE, "my_span");
```

<br>

The **span** in which a **thread** is currently executing is referred to as that **thread’s current span**.
```rust
use tracing::{span, Level};

fn main() {
  let span = span!(Level::TRACE, "my_span");
  
  // `enter()` returns a RAII guard which, when dropped, exits the span
  let _enter = span.enter();
  // perform some work in the context of `my_span`...
}
```

<br>

The `#[instrument]` attribute provides an easy way to add tracing spans to functions.<br>
A function annotated with `#[instrument]` will create and enter a span with that function’s name every time the function is called, with arguments to that function will be recorded as fields using `fmt::Debug`.<br>

<br>

## Events

**Events** are created using the `event!` macro. Parameters of `event!` macro:

| Parameter       | Optional | Input type                        | Default                                          |
|:----------------|:---------|:----------------------------------|:-------------------------------------------------|
| Target          | Yes      | `&str`                            | The **module path** where the macro was invoked  |
| Parent          | Yes      | parent_span_id                    | The **current span** where the macro was invoked |
| **Level**       | **No**   | Any `tracing::Level` enum variant |                                                  |
| Key-value field | Yes      | `&str`                            |                                                  |

The `event!` macro suports up to **32** key-value fields.<br>

<br>

There are **set of macros** for creating `Events`: `trace!`, `debug!`, `info!`, `warn!`, `error!`. They behave similarly to the `event!` macro, but with the `Level` argument with appropriate value.<br>

<br>

**Example**:
```rust
span!(Level::TRACE, "my_span");
let span = span!(Level::TRACE, "my_span");

event!(parent: &span, Level::INFO, "something happened inside my_span");
// or
event!(Level::INFO, "something happened inside my_span");
```

<br>

# `tracing-subscriber` crate
As **spans** and **events** occur, they are recorded or aggregated by implementations of the `Subscriber` trait.<br>
**Subscribers** are notified when an **event** takes place and when a **span** is **entered** or **exited**.<br>
It is up to the subscriber to determine whether and how span data should be stored.<br>

<br>

**Example**:
```rust
use tracing::{info, trace, warn, error, debug};
use tracing_subscriber::{self, FmtSubscriber};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let number = 5;
    debug!("Some debug");
    info!("Some info");
    warn!("Some warn ");
    error!("Some error...");
}
```

In this example, we create a `FmtSubscriber` and set it as the global default. The `with_max_level` function is used to set the **maximum level** of events that the subscriber will record. In this case, we’re recording all events up to the `TRACE` level.<br>

<br>

The most important component of the `tracing-subscriber` API is the `Layer` trait.<br>
A **layer** is a composable handler for tracing events. A **layer** must implement `Layer` trait. _Layers_ can be **composed together** with other layers to build a `Subscriber`.<br>

Multiple layers can have their own, **separate per-layer filters**. A **per-layer filter** must implement `Filter` trait.<br>
A **per-layer filter** determines whether a span or event is **enabled** for an individual layer and if enabled it will be **recorded**.<br>
This allows different Layers to handle separate subsets of the trace data emitted by a program.<br>

<br>

**Example**:
```rust
let subscriber = MySubscriber::new()
    .with(MyLayer::new())
    .with(MyOtherLayer::new())
    .with(MyThirdLayer::new());

tracing::subscriber::set_global_default(subscriber);
```
