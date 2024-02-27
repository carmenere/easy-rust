# Table of contents
- [Table of contents](#table-of-contents)
- [Description](#description)
- [`cargo` subcommands](#cargo-subcommands)
- [Examples](#examples)
		- [cargo tree](#cargo-tree)
		- [cargo build](#cargo-build)
		- [cargo test](#cargo-test)

<br>

# Description
`cargo` is both the **package manager** and the **build system** for Rust.

<br>

# `cargo` subcommands
|Subcommand|Explanation|
|:---------|:----------|
|cargo **new** `foo`|Creates **new** *directory* `foo` and fills it by: **<ul><li>src/main.rs</li><li>Cargo.toml</li><li>.git</li><li>.gitignore</li></ul>** Sets `foo` as name for **package**.|
|cargo **init**|Create files in **current working** *directory*: **<ul><li>src/main.rs</li><li>Cargo.toml</li><li>.git</li><li>.gitignore</li></ul>** Sets name of **current working** *directory* as name for **package**.|
|cargo **init** `foo`|Acts like the `cargo new foo` command, i.e., equal to the `cargo new foo` command.|
|cargo **run**|**Compiles** and **runs**. Creates `target` directory in `src` directory.|
|cargo **build**|Just **compiles**. Creates `target` directory in `src` directory.|
|cargo **build** `--release`|Just **compiles** for **production**. Creates `target` directory in `src` directory. Also creates `release` directory in `src/target` directory.|
|cargo **check**|Runs **unit tests**.|
|cargo **doc**|Generates **documentation**.|
|cargo **install** `bar`|Installs **binary** of **crate** with name `bar` from the default registry `crates.io`.|
|cargo **test** `bar`|Runs **unit tests** for **crate** with name `bar`.|
|cargo **tree**|Display a tree visualization of a **dependency graph**.|

<br>

# Examples
### cargo tree
- option `-i <package>` **invert** the tree direction and focus on the given package and shows all crates where specified package are used:
```
cargo tree -i libc
```

- option `-e <KIND>` specifies the kind of dependencies to display:
```
cargo tree -e features
```

<br>

### cargo build
```bash
PATH_TO_CARGO_TOML="./Cargo.toml"
CARGO_TARGET_DIR=".artefacts/build"
BUILD_OPTs="--release"
TARGET_ARCH="x86_64-unknown-linux-gnu"
BIN_NAME="foo"
CARGO_FEATURES="bar"

export RUSTFLAGS="-C target-feature=-crt-static"

cargo build $(BUILD_OPTs) \
		--bin $(BIN_NAME) \
		--features $(CARGO_FEATURES) \
		--manifest-path $(PATH_TO_CARGO_TOML) \
		--target-dir $(CARGO_TARGET_DIR) \
		--target $(TARGET_ARCH)

```

<br>

### cargo test
```bash
PATH_TO_CARGO_TOML="./Cargo.toml"
CARGO_TARGET_DIR=".artefacts/build"
BUILD_OPTs="--release"
TARGET_ARCH="x86_64-unknown-linux-gnu"
BIN_NAME="foo"
CARGO_FEATURES="bar"

export RUSTFLAGS="-C target-feature=-crt-static"

cargo test $(BUILD_OPTs) \
		--bin $(BIN_NAME) \
		--features $(CARGO_FEATURES) \
		--manifest-path $(PATH_TO_CARGO_TOML) \
		--target-dir $(CARGO_TARGET_DIR) \
		--target $(TARGET_ARCH)


```