# Prelude
A **prelude** is a collection of names that are automatically brought into scope of every module in a crate by compiler. It means that compiler **implicitly** injects ``use std::prelude::*`` **in every module** in a crate.

There are several different preludes:
- **standard library** prelude
- **extern** prelude
- **language** prelude

<br>

## Standard library prelude
**Standard library prelude** *consists* of the *names from* **standard library crate**.<br>

There are 2 *standard library* **crates** in Rust: 
- ``std`` crate
- ``core`` crate

<br>

[Crate std](https://doc.rust-lang.org/std/index.html)

<br>

What *standard library* **crate** is chosen (``std`` or ``core``) depends on ``no_std`` **attribute**.<br>
What **standard library prelude** is chosen depends on **Rust edition**.<br>

``no_std`` attribute is declared in **global section** of **root module**, e.g.:
```Rust
#![no_std]
```

<br>

|**Edition**|``no_std`` **not** applied|``no_std`` applied|
|:------|:-----------------|:-------------|
|**2015**|**std**::prelude::rust_2015|**core**::prelude::rust_2015|
|**2018**|**std**::prelude::rust_2018|**core**::prelude::rust_2018|
|**2021**|**std**::prelude::rust_2021|**core**::prelude::rust_2021|

<br>

## Extern prelude
**Extern prelude** *consists* of the *names* **of external crates**.<br>

*External crate* **declaration** is ``extern crate <name> [as <alias>]``.<br>

If the *external crate* **declaration** ``extern crate <name> ...`` appears in the **root module**, then the ``<name>`` is also added to the **extern prelude**.<br>

Editions:
- **In 2015 edition** you needed both, a package in the Cargo.toml ``[dependencies]`` section and ``extern crate <name> ... `` to bring crate from some package into scope.
- **In 2018 edition** if crate is defined in ``[dependencies]`` section it becomes **globally available**, i.e., **such crate** is automatically added to the **extern prelude**. It means we don't have to write ``use`` keyword to access the crate and it is **available in any point** of current package.

**sysroot crates**: 
- ``core`` crate;
- ``std`` crate;
- ``alloc`` crate;
- ``test`` crate.

The ``core`` crate is always added to the **extern prelude**.<br>
The ``std`` crate is added to the **extern prelude** unless the ``#![no_std]`` is put in the **root module**.<br>
Crates ``alloc`` and `test` are **not** automatically added to the **extern prelude**. They must be brought into scope with an ``extern crate <name>``, e**ven in the 2018 edition**:

```Rust
extern crate alloc;
use alloc::rc::Rc;
```

In 2018 path starting with ``::`` must reference an ``external crate``.

<br>

## Language prelude
**Language prelude** *contains* **built-in to the language names**: **types** and **attributes** **names**.<br>

The **language prelude** is **always in scope**.<br>

E.g., ``bool``, ``char``, ``i32`` and so on.
