[**Configuration**](https://doc.rust-lang.org/cargo/reference/config.html).<br>

<br>

# `.cargo/config.toml`
Some optionts for cargo can be set in `.cargo/config.toml` file.<br>

Example:
```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-feature=-crt-static"]

[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=-crt-static"]
```

<br>

# Merge rules for `.cargo/config.toml` files
If, for example, `cargo` was invoked in `/projects/foo/bar/baz`, then it will read and merge `.cargo/config.toml` files in following order:<br>
```sh
/projects/foo/bar/baz/.cargo/config.toml
/projects/foo/bar/.cargo/config.toml
/projects/foo/.cargo/config.toml
/projects/.cargo/config.toml
/.cargo/config.toml
$HOME/.cargo/config.toml
```

<br>

If a **key** is specified in **multiple** `config.toml` files, the values will get **merged** together.<br>
- `numbers`, `strings`, and `booleans` will use the value in the **deeper** config directory taking **precedence over ancestor directories**, where the **home directory** is the **lowest priority**;
- `arrays` will be **joined together**;