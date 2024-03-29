# Table of contents
- [Table of contents](#table-of-contents)
- [Modules](#modules)
  - [Crate’s module tree](#crates-module-tree)
  - [`mod` *keyword*:](#mod-keyword)
    - [Rust 2018 changes](#rust-2018-changes)
    - [Equavelent](#equavelent)
- [Items paths](#items-paths)
- [Types of items paths](#types-of-items-paths)
- [``use`` keyword](#use-keyword)
    - [Notations](#notations)
    - [``as`` keyword](#as-keyword)
    - [Globally available crates](#globally-available-crates)
- [Items visibility](#items-visibility)
    - [Example](#example)
      - [Output](#output)

<br>

# Modules
## Crate’s module tree
A **crate** consists of a **hierarchy of modules**, called **crate’s module tree**.

Every **module** corresponds to:
- `.rs` **file**;
- **module item**: `mod <name> { … }`.

A **module** is a **collection of items**. **Module** acts as **namespace for items**.

Every *module tree* has a **root module**. 

- The *root module* is an **entry point** into *crate*. 
- The *root module* **always** corresponds to some `.rs` file in *package*.

For **auto discovered crates**:
- the **root module** of a **library crate** corresponds to `src/lib.rs`;
- the **root module** of a **binary crate** corresponds to `src/ main.rs`.

The **module tree** *must be built manually*. It means **every** `.rs` file in *package* is included to *module tree* **explicitly** by `mod` *keyword*.<br>

<br>

## `mod` *keyword*:
`mod` *keyword*:
- is used to **add** *module* to *module tree*. 
- can be used **once** for particular *module*. 

<br>

Lookup rules:
- If you write `mod bar;` inside `foo.rs` file, then compiler will search for `bar.rs` and then (if it doesn’t exists) for `bar/mod.rs` (in both cases compiler will search file `bar.rs` or dir `bar/` inside the directory where `foo.rs` is).
- If you write `mod bar;` inside `mod.rs` file, then compiler will search for `bar.rs` inside the directory where `mod.rs` is.
- If you write `mod bar;` inside `main.rs` or `lib.rs` files, then compiler will search for `bar.rs` inside `src` directory.

<br>

### Rust 2018 changes
Consider hieararchy:
```bash
src
|- lib.rs
|- venti.rs
|- venti
    |- grande.rs
```

Content of files:
- `lib.rs`:
```rust
    pub mod venti;
```
- `venti.rs`:
```rust
    pub mod grande;
```
- `venti/grande.rs`:
```rust
    pub mod tall {
        pub fn coffee () { }
    }
```

<br>

In `lib.rs` we define **module** `venti`.<br>
In `venti.rs` we define **child module** `grande`.<br>

<br>

> **Note**:<br>
> The correspondig **file** (`grande.rs`) of **child module** (`grande`) must be placed into **directory** with the same name as its **parent module** (`venti`).

<br>

### Equavelent
It is also equalent to
- `lib.rs`:
```rust
    mod venti {
        mod grande {
            mod tall {
                fn coffee () { }
            }
        }
    }
```

<br>

# Items paths
In Rust items are
- Functions
- Types (structs, enums, unions, type aliases)
- Traits
- Impl blocks
- Macros
- Constants and statics
- Extern blocks
- Extern crates
- Imports
- Modules

**Every item** has **unique path** in module tree. 

Only **pub items** can be accessed **from any point** of crate by its path.

For example, the path ``foo::bar::Baz`` refers to the ``Baz`` *item* within the ``bar`` *item* within the ``foo`` *item*.

<br>

# Types of items paths
|Path|Description|
|:---|:----------|
|**Relative path**|Using ``foo::bar`` you can access **any pub** item inside ``bar`` item.<br>**Relative path** requires ``foo`` item to be available in the *current* scope.|
|**Relative path: super**|Using ``super::`` you can access **any pub** item in *parent* item of *current* item.<br>``super::`` is like ``../`` in file system.|
|**Absolute path**|Using ``crate::`` you can access **any pub** item **from any point** of crate.<br>**Absolute path** is prefixed with ``crate::``.<br>Prefix ``crate::`` refers to **root module**.|

<br>

# ``use`` keyword
``use`` keyword is used **to bring names into scope**, to make item’s path shorter.

For example, to make ``foo`` accessible in current scope directly include ``use path::to::foo;`` in current module.

<br>

### Notations
|Notation|Description|
|:-------|:----------|
|``use rand::*;``|**All pub names** inside ``rand`` can be used directly in current scope.|
|``use rand::random;``|Only name ``random`` from item ``rand`` can be used directly in current scope. Name ``random`` must be **pub**.|
|``use abc::{x, y, z};``|Only names ``x``, ``y``, ``z`` from item ``abc`` can be used directly in current scope. Names ``x``, ``y``, ``z`` must be **pub**.|
|``use std::fmt::{self, Display, Formatter};``|Here ``self`` is equal to separate line ``use std::fmt``.|

<br>

### ``as`` keyword
``as`` keyword allows create **aliases** for identifiers.<br>
``use futures as f;`` means ``futures`` can be used directly in current scope by ``f`` name, not by ``futures``.

<br>

### Globally available crates
If crate is defined in ``[dependencies]`` section it becomes **globally available**. It means we don't have to write ``use`` keyword to access the crate and it is **available in any point** of current package.

<br>

# Items visibility
By default, **everything in Rust is private**, with two **exceptions**: 
- **items** inside a ``Trait`` are **public** by default; 
- **variants** inside ``enum`` are **public** by default. 

Access rules:
- If an item is **public** (declared **with** ``pub`` keyword), it can be accessed from **external crate** and from **any point of current crate**.
- If an item is **private** (declared **without** ``pub`` keyword), it can be accessed **only** from the **current module and its descendants**.

In addition to **public** and **private**, Rust provides ``pub`` **keyword restrictions**.

The rules for ``pub`` **keyword restrictions**:
|Restriction|Description|
|:----------|:----------|
|``pub(crate) item``|Makes an item *visible* only within the **current crate**.|
|``pub(super) item``|Makes an item *visible* within the **parent module** and **all parent module’s descendants**.|
|``pub(self) item``|Makes an item **private**.|

<br>

### Example
In the code below ``E`` is **accessible in** ``B`` and ``C``, but **not in** ``A``:
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

mod A {
    fn f1() -> B::E {
        B::E::t
    }

    mod B {
        pub(self) enum E {
            t
        }

        fn f1() -> E {
            E::t
        }

        mod C {
            fn f1() -> super::E {
                super::E::t
            }
        }
    }
}

fn main() {

}
```

#### Output
```bash
cargo run                                                                                                     
   Compiling foo v0.1.0 (/tmp/some/project)                                                                                      
error[E0603]: enum `E` is private
  --> src/main.rs:7:19
   |
7  |     fn f1() -> B::E {
   |                   ^ private enum
   |
note: the enum `E` is defined here
  --> src/main.rs:12:9
   |
12 |         pub(self) enum E {
   |         ^^^^^^^^^^^^^^^^

error[E0603]: enum `E` is private
  --> src/main.rs:8:12
   |
8  |         B::E::t
   |            ^ private enum
   |
note: the enum `E` is defined here
  --> src/main.rs:12:9
   |
12 |         pub(self) enum E {
   |         ^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `foo` due to 2 previous errors                                                                                                      
```
