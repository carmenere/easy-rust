# Table of contents
- [Table of contents](#table-of-contents)
- [Registry](#registry)
- [Crate’s module tree](#crates-module-tree)
- [Crate auto-discovery](#crate-auto-discovery)
- [Packages and Crates](#packages-and-crates)
  - [The `Cargo.toml` layout](#the-cargotoml-layout)
  - [Platform-specific dependencies](#platform-specific-dependencies)
  - [The `[package]` section settings](#the-package-section-settings)
  - [The `include`/`exclude` keys](#the-includeexclude-keys)
  - [The \[lints\] section](#the-lints-section)
- [Targets](#targets)
  - [Target selection cli optons](#target-selection-cli-optons)
  - [Targets settings](#targets-settings)
  - [Example](#example)
- [Profiles](#profiles)
  - [The `[profile.*]` section settings](#the-profile-section-settings)
- [Workspaces](#workspaces)
  - [The `[workspace]` section settings](#the-workspace-section-settings)
  - [Package selection](#package-selection)

<br>

# Registry
A **registry** is a storage for **packages**.<br>
The *crates.io* is the Rust community’s **package registry**.<br>
Each package can be published on https://crates.io independently.<br>
Each package can be fetched from https://crates.io.<br>

<br>

# Crate’s module tree
A **crate** consists of a **hierarchy of modules**, called **crate’s module tree**.

A **module** is a **collection of items**. **Module** acts as **namespace for items**.

The **module tree** *must be built manually*. It means **every** `.rs` file in *package* is included to *module tree* **explicitly** by `mod` *keyword*.

Every *module tree* has a **root module**. 

- The *root module* is an **entry point** into *crate*. 
- The *root module* **always** corresponds to some `.rs` file in *package*.

<br>

# Crate auto-discovery
Cargo uses the **automatic target discovery** by default. *Automatic target discovery* **can be disabled**.

*Automatic target discovery* uses **package layout convention**. 

**Package layout’s convention**:
- `Cargo.toml` and `Cargo.lock` are stored in **package root**.
- Directory `src` contains **source code**.
- File `src/lib.rs` corresponds to the **root module** of **library crate**.
- File `src/main.rs` corresponds to the **root module** of **binary crate**.
- Directory `src/bin` is for other crates.

<br>

# Packages and Crates
A **package** is a *collection* of **crates**.<br>

Every **package** has `Cargo.toml` file. The `Cargo.toml` file for each package is called its **manifest**. <br>
**Directory** with `Cargo.toml` file is called **package root**.<br>

- `Cargo.toml` file contains **settings for package**.
- `Cargo.toml` file is written in the **TOML format**. 

<br>

## The `Cargo.toml` layout
Every `Cargo.toml` file consists of the following **sections**:<br>
|Section|Description|
|:------|:----------|
|`[[bin]]`|**Binary crate** settings.|
|`[build-dependencies]`|Dependencies for build scripts.|
|`[dependencies]`|Package dependencies.|
|`[dev-dependencies]`|Dependencies for **examples**, **tests**, and **benchmarks**.|
|`[features]`|**Conditional compilation** features.|
|`[lib]`|**Library crate** settings.|
|`[lints]`|Configure **linters** for this package.|
|`[package]`|Package settings.|
|`[profile.*]`|Compiler settings and optimizations.|
|`[target.*.dependencies]`|**Platform-specific** dependencies.|
|`[workspace]`|Workspace settings.|

<br>

## Platform-specific dependencies
Example of specifying **platform-specific dependencies**:
```toml
[target.'cfg(target_os = "linux")'.dependencies]
conntrack = { workspace = true }
```

<br>

## The `[package]` section settings
[**The Manifest Format**](https://doc.rust-lang.org/cargo/reference/manifest.html) of `[package]` section.<br>

Fields `name` and `version` are **required**, other fields are **optional**.<br>
**Any hyphens** in the **package name** are **replaced** with **underscores**.<br>

- `authors` specifies the **authors** of the package;
- `autobenches = true|false` enables/disables **bench** *auto discovery*;
- `autobins = true|false` enables/disables **binary** *auto discovery*;
- `autoexamples = true|false` enables/disables **example** *auto discovery*;
- `autolib = true|false` enables/disables **library** *auto discovery*;
- `autotests = true|false` enables/disables **test** *auto discovery*;
- `build` specifies path to the **package build script**;
  - the **default** is `build.rs` in the **package root**;
- `default-run` specifies a **default binary** picked by `cargo run` when there are more then one binary crate in package;
  - **example**: when there are `src/main.rs`, `src/bin/a.rs` and `src/bin/b.rs` set `default-run = "a"`;
- `description` specifies **description** of the package;
- `documentation` specifies URL of the **package documentation**;
- `edition` specifies the **Rust edition**;
- `exclude` is used to **explicitly specify** which **files** are **excluded** from package to registry;
- `homepage` specifies URL of the **package homepage**;
- `include` is used to **explicitly specify** which **files** are **included** before publishing package to registry;
- `keywords` specifies an array of strings that describe this package;
  - **crates.io** allows a **maximum** of **5** keywords, each keyword must
    - be **ASCII text**;
    - have at most **20** characters, **start** with an **alphanumeric character**;
    - contain **letters**, **numbers**, **_**, **-** or **+**;
- `license` specifies the **name of the software license** that the package is released under and the **name** must be in a **SPDX license expressions**;
  - **example**: `license = "MIT OR Apache-2.0"`
  - **if** a package is using a **nonstandard license**, then the `license-file` field may be specified:
    - **example**: `license-file = "LICENSE.txt"`
- `links = %name%` informs Cargo that current package is linked with the **system library** (**C library**) `%name%`;
  - `links` is only **informational** and it does **not** actually link to anything;
  - name `%name%` of **C library** must be without any **prefix**/**suffix** (e.g. `links = "z"`, **not** `"libz.so"`);
- `name` specifies a **package name**;
  - the **package name** is like **identifier** used to refer to the package;
  - **by default**, the **package name** is used as **name** for *auto discovered crate*: **binary** (`src/main.rs`) and/or **library** (`src/lib.rs`);
- `publish` specifies array of **registries names** the package may be published to;
  - **example**: `publish = ["some-registry-name"]`;
  - to prevent a package from being published to a registry (like **crates.io**) by mistake you can disable publishing by `publish = false`;
- `readme` specifies path to the package’s `README` file;
- `repository` specifies URL of the **package source repository**;
  - **example**: `repository = "https://github.com/rust-lang/cargo"`;
- `resolver` sets the **dependency resolver** to use;
- `rust-version` specifies the **minimal supported Rust version**;
- `version` specifies the **version** of the package;
- `workspace` specifies path to the **workspace for the package**;

<br>

## The `include`/`exclude` keys
The `include`/`exclude` list is also used for change **tracking** in some situations. If the package has a **build script** that does not emit any `rerun-if-*` directives, then the `include`/`exclude` list is used for tracking if the **build script** should be **re-run** if any of those files change.<br>

- the `exclude` and `include` fields can be used to **explicitly specify** which **files** are **included** when packaging a project to be published, run `cargo package --list` to verify which files will be included in the package;
  - **if** `include` is **not** specified, then the **following** files will be **excluded**:
    - **if** the package is **in** a *git repository*, any files that are ignored by the `.gitignore` rules of the repository and **global** git configuration will be **skipped**;
    - **if** the package is **not** in a *git repository*, all **hidden** files starting with a **dot** will be **skipped**.
  - the following files are **always included**:
    - the `Cargo.toml` file of the package itself is **always included**, it does not need to be listed in include;
    - **if** a `license-file` is **specified**, it is **always included**;
- `license` specifies the **name of the software license** that the package is released under and the **name** must be in a **SPDX license expressions**:


<br>

## The [lints] section
Example:<br>
```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
enum_glob_use = "deny"
```

<br>

Cargo only applies these to the **current package** and **not** to dependencies.<br>

<br>

# Targets
Cargo packages consist of targets which correspond to source files which can be compiled into a crate.<br>
Packages can have **library**, **binary**, **example**, **test** and **benchmark** targets. The list of targets can be configured in the` Cargo.toml` manifest or **inferred automatically** by the **directory layout** of the source files.<br>

`[lib]`, `[[bin]]`, `[[example]]`, `[[test]]` and `[[bench]]` sections are also called **target tables** (aka **targets**).

Every **target** describes **settings for some crate**.

There are **2 types** of **crates**:
- **binary crate**: defined in `[[bin]]`.
- **library crate**: defined in `[lib]`. 

The **double-bracket section** like `[[bin]]` is an **array-of-tables**. It means you can write **more than one** `[[bin]]` section to describe several crates in 1 package.<br>
So,
- every **package** contains **at least 1 crate**;
- there can be **only one** library crate within a package; 
- there can be **more than one** binary crate within a package.

Compiler will produce **executable** for *binary crate* and **library** for *library crate*.

So, **crate** is an **independent compilation unit** within package.

<br>

## Target selection cli optons
`--lib`               Build only this package's library
`--bins`              Build all binaries
`--bin [<NAME>]`      Build only the specified binary
`--examples`          Build all examples
`--example [<NAME>]`  Build only the specified example
`--tests`             Build all test targets
`--test [<NAME>]`     Build only the specified test target
`--benches`           Build all bench targets
`--bench [<NAME>]`    Build only the specified bench target
`--all-targets`       Build all targets

<br>

When **no** target selection options are given, cargo build will build **all** **binary** and **library** targets of the selected packages:
`--lib --bins`.<br>

<br>

## Targets settings
All targets (`[lib]`, `[[bin]]`, `[[example]]`, `[[test]]`, `[[bench]]`) have the same [configurable fields](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target):
- `path` specifies path to **root module**, *relative* to the **package root**.
- `name` specifies the **name of the crate**;
  - `name` is **optional** *for library crate* and **required** *for all other crates*;
  - **by default**, the **package name** is used as **name** for *auto discovered crate*: **binary** (`src/main.rs`) and/or **library** (`src/lib.rs`);
- `edition` defines the **Rust edition** for **concrete target**. If not specified, it **defaults** to the `edition` field for the `[package]`. 
  - example: `edition = "2015"`;
- `crate-type` defines the **crate types** that will be generated by the target. The available options are `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib` and `proc-macro`. This can only be specified for **libraries** and **examples**. **Binaries**, **tests**, and **benchmarks** are **always** the `bin` **crate type**. 
  - example: `crate-type = ["lib"]`;
- `required-features` specifies which **features** the target needs in order to be built
  - **if any** of the required features are **not** enabled, the target will be **skipped**;
  - this **has no effect** on `[lib]` section;

<br>

## Example
```toml
[[bin]]
name = "foo"
test = false
bench = false

[[bin]]
name = "bar"
```

<br>

# Profiles
Cargo has 4 **built-in profiles**:
- `profile.dev`;
- `profile.release`;
- `profile.test`;
- `profile.bench`;

<br>

The profile is *automatically chosen* if a profile is **not** specified on the cli by `--profile <PROFILE-NAME>` option.<br>
In addition to the *built-in profiles*, additional **custom profiles** can be defined.<br>

More details here: https://doc.rust-lang.org/cargo/reference/manifest.html.

<br>

The **default** settings for the **dev** profile are:
```toml
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = '...'  # Platform-specific.
strip = "none"
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false
```

<br>

The **default** settings for the **release** profile are:
```toml
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platform-specific.
strip = "none"
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'
incremental = false
codegen-units = 16
rpath = false
```

<br>

The **test** profile inherits the settings from the **dev** profile.<br>
The **bench** profile inherits the settings from the **release** profile.<br>

<br>

## The `[profile.*]` section settings
- `opt-level` setting controls the `-C opt-level` flag which controls the **level of optimization**;
  - the valid options are:
    - `0`: **no** optimizations;
    - `1`: basic optimizations;
    - `2`: some optimizations;
    - `3`: **all** optimizations;
- `debug` setting controls the `-C debuginfo` flag which controls the amount of debug information included in the compiled binary.
  - the valid options are:
    - `0`, `false`, or `none`: **no** debug info at all, **default** for **release**;
    - `1` or `limited`: debug info without type or variable-level information;
    - `2`, `true`, or `full`: **full** debug info, **default** for **dev**;
- `strip` option controls the `-C strip` flag, which directs rustc to strip either **symbols** or **debuginfo** from a binary;
  - the default is `none`;
  - the valid options are:
    - `none`;
    - `debuginfo`;
    - `symbols`;
- `overflow-checks` setting controls the `-C overflow-checks` flag which controls the behavior of **runtime integer overflow**; when `overflow-checks` are **enabled**, a panic will occur on overflow.
  - the valid options are:
    - `true`: enabled;
    - `false`: disabled;
- `lto` setting controls control LLVM’s link time optimizations;
- `panic` setting controls the `-C panic` flag which controls which panic strategy to use;
  - valid options are:
    - `unwind`: **unwind** the stack upon panic;
    - `abort`: **terminate** the process upon panic;
- `rpath` setting controls the `-C rpath` flag which controls whether or not **rpath** is enabled.
  - the valid options are:
    - `y`, `yes`, `on`, `true`: **enable** rpath;
    - `n`, `no`, `off` or `false`: **disable** rpath (the **default**);

<br>

# Workspaces
A **workspace** is a collection of **packages**, called **workspace members**, that are **managed together**.<br>

If the `[workspace]` section is added to a `Cargo.toml` that already defines a `[package]`, the package is the **root package** of the **workspace**.<br>
Alternatively, a `Cargo.toml` file can be created with a `[workspace]` section but **without** a `[package]` section. This is called a **virtual manifest**.<br>

The **workspace root** is the directory where the workspace’s C`argo.toml` is located.<br>

<br>

## The `[workspace]` section settings
The root `Cargo.toml` of a workspace supports the following **sections**:
- `[workspace]` defines a workspace;
- `[patch]` override dependencies;
- `[profile]` compiler settings and optimizations;

[**Workspaces settings**](https://doc.rust-lang.org/cargo/reference/workspaces.html).<br>

- `resolver` sets the dependency resolver to use;
- `members` specifies array of paths to packages to **include** in the **workspace**;
  - paths are **relative** to the **workspace root**;
  - the **members list** also supports **globs** like `*` and `?` to match multiple paths;
- `exclude` packages to exclude from the **members list**;
- `default-members` specifies array of paths of members to operate on when in the **workspace root** and the *package selection flags* are **not** used;
- `package` package settings that can be inherited by members of a workspace;
- `dependencies` dependencies to be inherited by members of a workspace;
- `lints` lint configuration to be inherited by members of a workspace;

<br>

## Package selection
In a **workspace** `cargo build` can use the **package selection flags**:
- `-p [<SPEC>]`, `--package [<SPEC>]`  packages to build;
- `--workspace` build **all** packages in the workspace;
- `--exclude <SPEC>` **exclude** packages from the build, can be used multiple times;

<br>

The `SPEC` is **package specifier** or just **spec**. The `cargo help pkgid` prints additional info about **spec**.<br>
A **fully qualified package specification** is a `url#name@version`, e.g. `https://github.com/rust-lang/cargo#crates-io@0.21.0`.<br>
But **partial specs** are also allowed: `name` of package or `name@version`.<br>

If neither of those flags are specified, Cargo will use the **package** in the **current working directory**.<br>
However, if the *current directory* is a **workspace root**, the `default-members` will be used.<br>
When a **root package** is present, you can only operate on it using `--package` and `--workspace` flags.<br>
