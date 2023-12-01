# Rust toolchain 
**Toolchain** is set of **components**. Some of components are **required** and some are **optional**.

<br>

**Required** components:
|Components|Description|
|:---------|:----------|
|`rustc`|**Compiler** for Rust.|
|`rust-src`|This is a **local copy** of the **source code** of the Rust **standard library**.|
|`rust-std`|This is the Rust **standard library**.|
|`cargo`|Both the **package manager** and the **build system** for Rust.|

<br>

**Optional** components:
|Components|Description|
|:---------|:----------|
|`rls`|This is Rust **language server** that provides support for IDE|
|`rustfmt`|This is a **formatting tool** for Rust.|
|`clippy`|This is a **linter** for Rust.|

<br>

# Release channel
Every *toolchain* has a **toolchain specification** (aka r**elease channel**). 

A **toolchain specification** has the general format: `<channel-name>`\[-`<date>`\]\[-`<target-triple>`\], where:
- `<channel-name>` = `stable`|`beta`|`nightly`|`<major.minor>`|`<major.minor.patch>`
- `<date>` = `YYYY`-`MM`-`DD`
- `<target-triple>` = `<arch>`[`<sub>`]-`<vendor>`-`<sys>`[-`<abi>`], where:
    - `arch` = `x86_64`|`i386`|`arm`|`aarch64`|`thumb`|`mips`
    - `sub` (*sub arch*) = `v5`|`v6m`|`v7a`|`v7m`
    - `vendor` = `unknown`|`pc`|`apple`|`nvidia`|`ibm`
    - `sys` (*OS*) = `none`|`linux`|`win32`|`darwin`|`cuda`
    - `abi` = `gnu`|`android`|`elf`|`eabi`. If `abi` is omitted, then default set of **CPU**/**FPU**/**ABI** flags will be chosen.

**Target triple** (aka **host triple**) `<target-triple>` defines **target architecture** of host on which Rust toolchain will be run.

<br>

Notes:
- If `<date>` is omitted – **current date** is used.
- Channel names `stable`|`beta`|`nightly` are *synonyms* for **appropriate toolchain versions** that are **actual for some date**. So, channel names `stable`|`beta`|`nightly` can refer to different toolchain versions **at different time**.

<br>

Examples:
- `stable`-`aarch64`-`apple`-`darwin`
- `1.58.1`-`aarch64`-`apple`-`darwin`
