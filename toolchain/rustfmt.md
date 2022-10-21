# Description
``rustfmt`` is a tool that **checks** and **reformats** code in Rust according to the **community code style**.

<br>

# ``rustfmt`` commands
|Example|Explanation|
|:------|:----------|
|**rustup component add** ``rustfmt``|Adds ``rustfmt`` **component** to the **active toolchain**.|
|**cargo** ``fmt``|Runs ``rustfmt`` for the **current crate**.|

<br>

# Examples
```bash
PATH_TO_CARGO_TOML="./Cargo.toml"

cargo fmt \
		--all \
		--manifest-path $(PATH_TO_CARGO_TOML) \
		-- --check

```