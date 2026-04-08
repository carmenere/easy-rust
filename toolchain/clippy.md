# Description
`clippy` is a **linter** for Rust.<br>
The **linter** (or **lint**), is a **static code analysis tool**.<br>

<br>

# `clippy` commands
`clippy` is distributed as a **rustup component**.<br>

|Example|Explanation|
|:------|:----------|
|rustup **component add** `clippy`|Adds `clippy` **component** to the **active toolchain**.|
|cargo `clippy`|Runs `clippy` for the **current crate**.|

<br>

# Example
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
		--fix --all-targets --all-features \
		-- -D warnings
```

**Options**:
- `--fix` in some cases, `clippy` can automatically fix code;
- `--all-features` enables **all crate features**;
- `--all-targets` also **checks tests**
  - **by default**, `clippy` **ignores** tests;
- `-D warnings` instructs `clippy` to **fail on warnings**;

<br>

# Clippy’s lints
[**Up-to-date list of the lints for stable Rust**](https://rust-lang.github.io/rust-clippy/stable/index.html).<br>

**Lints**:
- `blacklisted_name`
  - **disallows** the use of variable names such as `foo`, `bar`, or `quux`;
  - his lint **can be configured** to include a **custom list of variable names** you wish to disallow;
- `bool_comparison`
  - checks for **unnecessary comparisons** between expressions and booleans;
  - the following code is **considered invalid**: `if function_returning_boolean() == true {}`, because `== true` is **not necessary**;
  - on the other hand, the following code is **valid**: `if function_returning_boolean() {}`
- `redundant_clone`
  - can **find** situations where a variable is **unnecessarily cloned**;