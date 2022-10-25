# ``log`` crate
The ``log`` crate is the de-facto **logging API** in Rust.<br>

One important note is that the log crate only provides the API, not implementations.<br>

There are **5 log levels**:
- **error** (**highest** priority);
- **warn**;
- **info**;
- **debug**;
- **trace** (**lowest** priority).

<br>

To log a message, there are corresponding **marcos**:
- ``error!()``;
- ``warn!()``;
- ``info!()``;
- ``debug!()``;
- ``trace!()``.

<br>

These marcos behave like ``println!`` and supports the syntax of ``format!``.<br>

That is:
- ``{}`` calls ``display()`` on an object;
- ``{:?}`` calls ``debug()`` on an object;
- ``{:#?}`` **pretty-print** the **debug** formatting.

<br>

# Other crates
There are many available implementations
- **Simple** minimal loggers:
    - ``env_logger``
- **Complex** configurable frameworks
    - ``log4rs``
    - ``fern``
- **Adaptors** for other facilities
    - ``syslog``

<br>

# ``env_logger`` crate
By far, the most commonly used logging library in Rust is the ``env_logger`` crate.<br>

``env_logger`` takes an environment variable ``RUST_LOG`` to configure its **log level**: ``std::env::set_var("RUST_LOG", "debug");``.<br>

Initialize logger: ``env_logger::init();``.

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

# ``log4rs`` crate
Another logging library in Rust is log4rs crate.<br>

```Rust
use log::error;
use log::info;
use log::warn;
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

fn main() {
    let stdout = ConsoleAppender::builder().build();    
    let config = Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace)).unwrap();    
    let _handle = log4rs::init_config(config).unwrap();    

    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}
```
