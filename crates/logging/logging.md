<!-- TOC -->
* [``log`` crate](#log-crate)
  * [Usage](#usage)
  * [Log request](#log-request)
  * [Loggers](#loggers)
* [`env_logger` crate](#env_logger-crate)
  * [`RUST_LOG` environment](#rust_log-environment)
  * [Struct `Env`](#struct-env)
  * [Struct `Builder`](#struct-builder)
    * [Examples](#examples)
* [Other crates](#other-crates)
<!-- TOC -->

<br>

# ``log`` crate
[**Doc**](https://docs.rs/log/latest/log/).<br>

The `log` crate provides a single **logging API** that abstracts over the actual **logger**.<br>
A **logger** (aka **logging implementation**) is an anything that implements the `Log` trait provided by `log` crate.<br>

<br>

## Usage
**In libraries**:<br>
Libraries should link only to the `log` crate (`use log::{info, warn};`), and use the provided macros.<br>

**In executables**:<br>
Executables should choose a **logger** and initialize it early in the runtime of the program. The logging system may only be initialized **once**. Any log messages generated **before** the _logger_ is initialized will be **ignored**.<br>
If **no** **logger** is selected, the _logging API_ falls back to a **noop logger** that **ignores all log messages**. The overhead in this case is very small - **just an integer load**, comparison and jump.<br>

<br>

_Logging API_ provides 5 **marcos**:
- `error!`;
- `warn!`;
- `info!`;
- `debug!`;
- `trace!`;

These marcos behave like `println!` and supports the syntax of `format!`.<br>

That is:
- `{}` calls `display()` on an object;
- `{:?}` calls `debug()` on an object;
- `{:#?}` **pretty-print** for `debug()`;

<br>

## Log request
A **log request** is an invocation any of these macros. A **log request** consists of:
- a _target_: the **module path** of the location of the log request;
- a _level_: **log level** (aka **severity of message**);
- a _body_: actual **log message**;

<br>

**Every** _log request_ gets **appropriate** _log level_: the `error!` macros sets its log level to `Error`, and so on.<br>

The `log` crate provides **five** _log levels_:
```rust
pub enum Level {
  Error = 1,
  Warn,
  Info,
  Debug,
  Trace,
}
```

<br>

The `Error` represents the **minimal** _log level_, **1** and the `Trace` represents the **maximal** _log level_, **5**, because it provides the maximum details in the emitted logs.<br>
The `Error` has the **highest priority** and the `Trace` has the **lowest priority**.<br>
The **log messages** are filtered by configuring the _log level_ to **exclude** messages with a **lower priority**.<br>

<br>

## Loggers
A **logger** (aka **logging implementation**) is an anything that implements the `Log` trait.<br>

Custom `impl` for `Log` trait:
```rust
use log::{error, info, warn, Record, Level, Metadata, LevelFilter, SetLoggerError};
use env_logger;

struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
  log::set_logger(&LOGGER)?;
  log::set_max_level(LevelFilter::Warn); // sets MAX_LOG_LEVEL_FILTER

  // if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level()
  // max_level() extract MAX_LOG_LEVEL_FILTER and cpmapre with lvl
  info!("hello log");
  warn!("warning");
  error!("oops");
  Ok(())
}
```

<br>

- `metadata.level()`: contains the **log level** of current **log request**;
- `set_logger`: sets appropriate logger;
- `set_max_level`: sets the variable `MAX_LOG_LEVEL_FILTER`, aka global maximum log level:
  - messages with **log level** > **MAX_LOG_LEVEL_FILTER** are **filtered out**;
  - only messages with **log level** <= **MAX_LOG_LEVEL_FILTER** are printed out;
  - it’s important to set it, as it defaults to `Off`, so **no** log messages will ever be captured;
  - for example, `log::set_max_level(LevelFilter::Info)` will **ignore** `debug!()` and `trace!()` log requests;

<br>

The `set_logger` requires you to provide a `&'static Log`, which can be hard to obtain if your logger depends on some runtime configuration.<br>
The `set_boxed_logger` function is available with the `std` **feature**:
```toml
log = { version = "0.4", features = ["std"] }
```

<br>

The `set_boxed_logger` is identical to `set_logger` except that it takes a `Box<Log>` rather than a `&'static Log`:
```rust
fn main() {
    log::set_boxed_logger(Box::new(SimpleLogger));
}
```

<br>

A **logger** should provide a function that wraps a call to `set_logger` and `set_max_level`, handling **initialization** of the logger:
```rust
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

<br>

# `env_logger` crate
[**Doc**](https://docs.rs/env_logger/latest/env_logger/).<br>

The `env_logger` crate is a simple **logger** over `log` crate. It can be configured via environment variables.<br>
By default:
- `env_logger` writes logs to **stderr**;
- `env_logger` sets the **global maximum log level** to `Error`;

<br>

The `env_logger::init()` initializes the global logger from `Env::default()`:
```Rust
use log::debug;
use log::error;
use log::info;
use log::warn;

fn main() {
    env_logger::init();

    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}
```

<br>

## `RUST_LOG` environment
The `RUST_LOG` environment variable contains comma-separated list of filters:
```shell
RUST_LOG=[target][=][level][,...]
```

Filter consists of _only_ **target**, _only_ **level** or **both** (using `=`).<br>

The **target** is typically the **path** of the module the _log request_ is **originated** from. The path is **rooted** in the **name of the crate** it was compiled for.<br>

The **level** is the maximum `log::Level` to be shown and includes:
- `error`
- `warn`
- `info`
- `debug`
- `trace`
- `off` (pseudo level to disable all logging for the target)

The **level** names are **case-insensitive**; e.g., `debug`, `DEBUG`, and `dEbuG` all represent the same logging level.<br>

<br>

Some examples of valid values of `RUST_LOG` are:
- `RUST_LOG=hello` turns on **error** (default) logging for the **hello** module only;
- `RUST_LOG=trace` turns on **global trace** logging for the entire application;
- `RUST_LOG=hello=debug` turns on **debug** logging for **hello** only;
- `RUST_LOG=error,hello=warn` turn on **global error** logging and also **warn** logging for **hello**;
- `RUST_LOG=error,hello=off` turn on **global error** logging, but **turn off** logging for **hello**;

<br>

## Struct `Env`
The `Env` is a set of environment variables to configure from. By default, the `Env` will read the following environment variables:
- `RUST_LOG`: defines **set of filters**;
- `RUST_LOG_STYLE`: defines **style preferences**;

<br>

```rust
pub const DEFAULT_FILTER_ENV: &str = "RUST_LOG";
pub const DEFAULT_WRITE_STYLE_ENV: &str = "RUST_LOG_STYLE";

impl Default for Env<'_> {
    fn default() -> Self {
        Env {
            filter: Var::new(DEFAULT_FILTER_ENV),
            write_style: Var::new(DEFAULT_WRITE_STYLE_ENV),
        }
    }
}
```

<br>

Example:
```rust
fn init_logger() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");
}
```

There are also methods with `_or` suffix: if the variable is **not** set, the **default** value will be used:
```rust
fn main() {
  let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "trace")
    .write_style_or("MY_LOG_STYLE", "always");
}
```

<br>

## Struct `Builder`
The `Builder` acts as builder for initializing a **logger**.<br>

Methods to configure builder:
- [**from_default_env**](https://docs.rs/env_logger/latest/env_logger/struct.Builder.html#method.from_default_env):
  - initializes the **log builder** from the environment using **default variable names**;
  - this method is a convenient way to call `from_env(Env::default())` without having to use the `Env` type explicitly;
- [**from_env**](https://docs.rs/env_logger/latest/env_logger/struct.Builder.html#method.from_env):
  - initializes the **log builder** from the environment;
  - it receives anything that implements `Into<Env>`;
  - it is possible to use **another names** for variables;
- [**init**](https://docs.rs/env_logger/latest/env_logger/struct.Builder.html#method.init):
  - initializes the **global logger** from the _log builder_;

<br>

### Examples
```rust
use env_logger::Builder;

fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.init();
}
```

<br>

```rust
use env_logger::Builder;

fn init_logger() {
    let mut builder = Builder::from_env("MY_LOG");
    builder.init();
}
```

<br>

```rust
use env_logger::{Builder, Env};

fn init_logger() {
  let env = Env::default()
          .filter("MY_LOG_LEVEL")
          .write_style("MY_LOG_STYLE");

  Builder::from_env(env)
          .format_level(false)
          .format_timestamp_nanos()
          .init();
}
```

<br>

# Other crates
- **Complex** configurable frameworks
  - `log4rs`
  - `fern`
- **Adaptors** for other facilities
  - `syslog`
