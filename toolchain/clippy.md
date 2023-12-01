# Description
`clippy` is a **linter** for Rust. 
<br>
The **linter** (or **lint**), is a **static code analysis tool**. 

<br>

# `clippy` commands
|Example|Explanation|
|:------|:----------|
|rustup **component add** `clippy`|Adds `clippy` **component** to the **active toolchain**.|
|cargo `clippy`|Runs `clippy` for the **current crate**.|

<br>

# Examples
```bash
PATH_TO_CARGO_TOML="./Cargo.toml"
CARGO_TARGET_DIR=".artefacts/build"
BUILD_OPTs="--release"
TARGET_ARCH="x86_64-unknown-linux-gnu"
BIN_NAME="foo"

cargo clippy $(BUILD_OPTs) \
		--bin $(BIN_NAME) \
		--manifest-path $(PATH_TO_CARGO_TOML) \
		--target-dir $(CARGO_TARGET_DIR) \
		--target $(TARGET_ARCH) \
		-- -D warnings
```