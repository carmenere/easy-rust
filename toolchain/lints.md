# Lints
**Linter**, is a static code *analysis tool* used to flag programming **errors**, **bugs**, **stylistic errors** and **suspicious constructs**.<br>
The term originates from a Unix utility that examined C language source code.

The Rust compiler has **builtin linter** that **runs lints** at compile time.<br>

**Lint** is a **piece of check**.

<br>

## Lint levels
In ``rustc``, all lints are divided into **5 levels**:
- **allow**
- **warn**
- **force-warn**
- **deny**
- **forbid**

**Each lint** has a **default level**. The **level** of any lint **can be changed**.<br>
The command ``rustc -W help`` will print **all lints** and its **default levels** and **all lint groups**.<br>

When **linter** finds **lint violation** it produces a **message of particular type** depending on **lint level**.<br>

Maping between **lint level** and **type of message** that is produced by linter:
- If lint of **allow** level is violated then linter produces a **nothing**;
- If lint of **warn** level is violated then linter produces a **warning**;
- If lint of **deny** level is violated then linter produces a **error**.

<br>

**Force-warn** is the same as **warn**, but **unlike** the **warn** level:
- the **force-warn** level **cannot** be capped via ``--cap-lints LEVEL`` **flag**.
- the **force-warn** level **cannot** be capped via **compiler lint flags**.

<br>

**Forbid** is the same as **deny**, but **unlike** the **deny** level:
- the **forbid** level **cannot** be capped via **compiler lint flags**. 
- however, the **forbid** level **can be** capped with ``--cap-lints LEVEL`` **flag**.

<br>

## Lint groups
All lints are divided into **lint groups**.

**Lint groups** are:
- **warnings**;
- **nonstandard-style** (for instance, non-camel-case-type);
- **unused** (for instance, unused-variables).
- **future-incompatible**;
- **rust-2018-compatibility**;
- **rust-2018-idioms**;
- **rust-2021-compatibility**;

<br>

## Configuring lint levels
The level of any lint or whole lint group can be changed:
- via compiler's cli **lint flags**;
- via **attributes** in the source code.

<br>

## Priorities
Priorities:<br>
`lint flags (-A, -W, -D)` **<** `lint attributes (allow, warn, deny, forbid)` **<** `lint flag -F` **<** `--cap-lints` **<** `--force-warn`

<br>

> **Notes**:<br>
> Between `-A`, `-W`, `-D` the **last** *wins*.<br>
> Between `--force-warn` and `-F` lint flags the **first** *wins*.<br>
> *Order* **doesn't matter** for `--cap-lints`.<br>

<br>

### Examples
|Command|Description|
|:------|:----------|
|`RUSTFLAGS="-F dead_code --force-warn dead_code" cargo build`|Here `-F` wins.|
|`RUSTFLAGS="--force-warn dead_code -F dead_code" cargo build`|Here `--force-warn` wins.|
|`RUSTFLAGS="-F dead_code --cap-lints warn --force-warn dead_code" cargo build`|Here `--cap-lints` wins: `-F` **first**, so it rewrites `--force-warn`, but then `--cap-lints` rewrites `-F`.|
|`RUSTFLAGS="--cap-lints allow --force-warn dead_code -F dead_code" cargo build`|Here `--force-warn` wins: `--force-warn` **first**, so it suppresses `-F` and then rewrites `--cap-lints`.|

<br>

### Via lint flags
|Flag|Lint level|
|:---|:---------|
|``-A <lint> \| <lint-group>``|Sets lint ``<lint>`` or lint-group ``<lint-group>`` into **allow** level.|
|``-W <lint> \| <lint-group>``|Sets lint ``<lint>`` or lint-group ``<lint-group>`` into **warn** level.|
|``--force-warn <lint> \| <lint-group>``|Sets lint ``<lint>`` or lint-group ``<lint-group>`` into **force-warn** level.|
|``-D <lint> \| <lint-group>``|Sets lint ``<lint>`` or lint-group ``<lint-group>`` into **deny** level.|
|``-F <lint> \| <lint-group>``|Sets lint ``<lint>`` or lint-group ``<lint-group>`` into **forbid** level.|

<br>

Notes:
- it is possible to pass each **lint flag** more than once for changing **multiple lints**.
- the **order** of **lint flags** is **taken into account**: **last wins**.

#### Example
The following commands **allows** the ``unused-variables`` lint, because it is the last:
- ``rustc lib.rs --crate-type=lib -D unused -A unused-variables``
- ``rustc lib.rs --crate-type=lib -D unused-variables -A unused-variables``, here `-A` wins.


<br>

If ``cargo`` is used, then **env** ``RUSTFLAGS`` is used to pass **lint flags**, e.g.,<br>``RUSTFLAGS="-D unused" cargo run``.

<br>

### Via attribute in the source code
|Attribute|Lint level|
|:--------|:---------|
|``#![allow(<lint>)]``|Sets lint ``<lint>`` into **allowed** level.|
|``#![warn(<lint>)]``|Sets lint ``<lint>`` into **warn** level.|
|``#![deny(<lint>)]``|Sets lint ``<lint>`` into **deny** level.|
|``#![forbid(<lint>)]``|Sets lint ``<lint>`` into **forbid** level.|

<br>

> **Note**:
> There is **no way** to set a lint to **force-warn** using an **attribute**.

<br>

## Capping lints
``rustc`` supports a flag ``--cap-lints LEVEL`` that sets the **lint cap level**.

The **lint cap level** *sets* **global level** for all lints.

Examples:
1.	Set all lints to **warn** level: ``rustc lib.rs --cap-lints warn``;
2.	Set all lints to **allow** level: ``rustc lib.rs --cap-lints allow``.

<br>

# Example: ways to disable some compiler warnings
#### Using *outer* **allow attribute** above item
```Rust
#[allow(dead_code)]
struct SemanticDirection;
```

<br>

#### Using *inner* **allow attribute** inside block
```Rust
#![allow(dead_code)]
```

<br>

#### Using ``rustc`` **lint flags**
```Rust
rustc -A unused_variables main.rs
```

<br>

#### Via ``cargo`` ``RUSTFLAGS`` env
```Rust
RUSTFLAGS="$RUSTFLAGS -A unused_variables" cargo build
```

<br>

# Some useful warnings
Some usefull warnings:
- ``unused_variables``
- ``unused_assignments``
- ``unused_macros``
- ``non_snake_case``
- ``dead_code``
- ``unused_mut``
- ``non_camel_case_types``
