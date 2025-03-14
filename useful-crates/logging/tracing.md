# Tracing stack
- [**tracing**](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html) is a **framework** for **instrumenting** Rust programs to collect traces.;
- [**tracing-subscriber**](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html): provides utilities for implementing and configuring subscribers;
- [**tracing-flame**](https://crates.io/crates/tracing-flame): for generating **flamegraphs** and **flamecharts** with [**inferno**](https://crates.io/crates/inferno);
- [**tracing-opentelemetry**](https://crates.io/crates/tracing-opentelemetry): provides compatibility with **OpenTelemetry**;
- [**tracing-futures**](https://crates.io/crates/tracing-futures): provides support for instrumenting **asynchronous code** written using **futures** and **async**/**await**;

<br>

# `tracing` crate
The core of **tracing’s API** is composed of:
- 5 macros for different levels of diagnostic information: `trace!`, `debug!`, `info!`, `warn!`, `error!`;
- ([**spans**](https://docs.rs/tracing/latest/tracing/span/index.html)): unlike a _log_ that represents a _moment in time_, a **span** represents a **period of time** with a beginning and an end;
- [**events**](https://docs.rs/tracing/latest/tracing/struct.Event.html): **like** a _log_ **event** represents **moment in time**, but **unlike** a _log_, an **event** exists within the **context** of a span;
- [**subscribers**](https://docs.rs/tracing/latest/tracing/trait.Subscriber.html): **subscriber** implement primitives for **collecting** trace events and spans;

<br>

## Spans and events
A **span** consists of fields, **set of fixed attributes** and **set of arbitrary user-defined key-value pairs**.
Attributes describing spans include:
- an **id** assigned by the subscriber that uniquely identifies it in relation to other spans;
- the **span’s parent** in the **trace tree**;
- [**metadata**](https://docs.rs/tracing/latest/tracing/struct.Metadata.html);

<br>

**Spans** are created using the `span!` macro. **Spans** form a **tree structure** — unless it is a root span, all spans have a parent, and may have one or more children. When a new span is created, the current span becomes the new span’s parent.<br>
Thus, a parent span always lasts for at least as long as the longest-executing span in its subtree.<br>
Execution may enter and exit a span multiple times before that span is closed.<br>

As **spans** and **events** occur, they are recorded or aggregated by implementations of the `Subscriber` trait.<br>
**Subscribers** are notified when an **event** takes place and when a **span** is **entered** or **exited**.<br>
It is up to the subscriber to determine whether and how span data should be stored.<br>

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

# `tracing-subscriber` crate
A layer is a composable handler for tracing events. A Layer implements a behavior for recording or collecting traces that can be composed together with other Layers to build a Subscriber.
The most important component of the tracing-subscriber API is the Layer trait, which provides a composable abstraction for building Subscribers.
Multiple layers can have their own, separate per-layer filters. A span or event will be recorded if it is enabled by any per-layer filter, but it will be skipped by the layers whose filters did not enable it.
A per-Layer filter that determines whether a span or event is enabled for an individual layer.
In addition, the Filter trait defines an interface for filtering what spans and events are recorded by a particular layer. This allows different Layers to handle separate subsets of the trace data emitted by a program.

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

In this example, we create a FmtSubscriber and set it as the global default. The with_max_level function is used to set the maximum level of events that the subscriber will record. In this case, we’re recording all events up to the TRACE level.


<br>

```rust
let subscriber = MySubscriber::new()
    .with(MyLayer::new())
    .with(MyOtherLayer::new())
    .with(MyThirdLayer::new());

tracing::subscriber::set_global_default(subscriber);
```

<br>

