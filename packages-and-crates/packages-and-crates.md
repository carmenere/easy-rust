# Table of contents
- [Table of contents](#table-of-contents)
- [Registry](#registry)
- [Workspace, Packages and Crates](#workspace-packages-and-crates)
- [Targets](#targets)
- [Crate’s module tree](#crates-module-tree)
- [Crate auto-discovery](#crate-auto-discovery)
- [The `[package]` section settings](#the-package-section-settings)
    - [The `include`/`exclude` keys](#the-includeexclude-keys)
- [Targets settings](#targets-settings)
  - [Example](#example)
- [Specifying dependencies](#specifying-dependencies)
  - [Specifying dependencies from crates.io](#specifying-dependencies-from-cratesio)
  - [Specifying dependencies from other registries](#specifying-dependencies-from-other-registries)
  - [Specifying dependencies from git repositories](#specifying-dependencies-from-git-repositories)
  - [Specifying path dependencies](#specifying-path-dependencies)
  - [Inheriting a dependency from a workspace](#inheriting-a-dependency-from-a-workspace)
- [Dependency resolution](#dependency-resolution)
  - [Dependency updates](#dependency-updates)

<br>

# Registry
A **registry** is a storage for **packages**, i.e., it is a central location that serves as permanent storage for versions of a crate over time.<br>
The *crates.io* is the Rust community’s **package registry**.<br>
Each package can be published on https://crates.io independently.<br>
Each package can be fetched from https://crates.io.<br>

<br>

# Workspace, Packages and Crates
A **package** is a *collection* of **crates**.  
A **workspace** is a collection of **packages**, called **workspace members**, that are **managed together**.

Every **package** has `Cargo.toml` file. The `Cargo.toml` file for each package is called its **manifest**. <br>
**Directory** with `Cargo.toml` file is called **package root**.<br>

- `Cargo.toml` file contains **settings for package**.
- `Cargo.toml` file is written in the **TOML format**. 

Every `Cargo.toml` file consists of the following sections:

|Section|Description|
|:------|:----------|
|`[workspace]`|Workspace settings.|
|`[package]`|Package settings.|
|`[features]`|**Conditional compilation** features.|
|`[lib]`|**Library crate** settings.|
|`[[bin]]`|**Binary crate** settings.|
|`[dependencies]`|Package dependencies.|
|`[dev-dependencies]`|Dependencies for **examples**, **tests**, and **benchmarks**.|
|`[build-dependencies]`|Dependencies for build scripts.|
|`[target]`|Platform-specific dependencies.|
|`[profile.*]`|Compiler settings and **optimizations**.|
|`[lints]`|Configure **linters** for this package.|

<br>

Cargo has 4 **built-in profiles**:
- `profile.dev`;
- `profile.release`;
- `profile.test`;
- `profile.bench`.

<br>

The profile is *automatically chosen* if a profile is **not** specified on the cli by `--profile <PROFILE-NAME>` option.<br>
In addition to the *built-in profiles*, additional **custom profiles** can be defined.<br>

More details here: https://doc.rust-lang.org/cargo/reference/manifest.html.

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

# The `[package]` section settings
[Configurable fields](https://doc.rust-lang.org/cargo/reference/manifest.html) of `[package]` section.<br>

Fields `name` and `version` are **required**.<br>
**Any hyphens** in the **package name** are **replaced** with **underscores**.<br>

- `name` specifies a **package name**, **package name** is like **identifier** used to refer to the package;
  - **by default**, the **package name** is used as **name** for *auto discovered crate*: **binary** (`src/main.rs`) and/or **library** (`src/lib.rs`);
- `version` specifies the **version** of the package;
- `authors` specifies the **authors** of the package;
- `edition` specifies the **Rust edition**;
- `rust-version` specifies the **minimal supported Rust version**;
- `description` specifies **description** of the package;
- `documentation` specifies URL of the **package documentation**;
- `readme` specifies path to the package’s `README` file;
- `homepage` specifies URL of the **package homepage**;
- `repository` specifies URL of the **package source repository**;
- `workspace` specifies path to the **workspace for the package**;
- `default-run` specifies a **default binary** picked by `cargo run`;
- `build` specifies a file in the *package root* which is a **build script** for **building native code**;
  - the default is `build.rs`, which loads the script from a file named `build.rs` in the **root of the package**;
- `links = %name%` informs Cargo that current crate links with the given **C library** (aka **system library**, **native library**);
  - `links` is only **informational** and it does **not** actually link to anything;
  - name `%name%` of **C library** must be without any **prefix**/**suffix** (e.g. `links = "z"`, **not** `"libz.so"`);
- the `exclude` and `include` fields can be used to **explicitly specify** which **files** are **included** when packaging a project to be published, run `cargo package --list` to verify which files will be included in the package;
  - **if** `include` is **not** specified, then the **following** files will be **excluded**:
    - **if** the package is **in** a *git repository*, any files that are ignored by the `.gitignore` rules of the repository and **global** git configuration will be **skipped**;
    - **if** the package is **not** in a *git repository*, all **hidden** files starting with a **dot** will be **skipped**.
  - the following files are **always included**:
    - the `Cargo.toml` file of the package itself is **always included**, it does not need to be listed in include;
    - **if** a `license-file` is **specified**, it is **always included**;
- `resolver` sets the **dependency resolver** to use;
- The repository field should be a URL to the source repository for your package:
  - example: `repository = "https://github.com/rust-lang/cargo"`;
- `license` specifies the **name of the software license** that the package is released under and the **name** must be in a **SPDX license expressions**:
  - example: `license = "MIT OR Apache-2.0"`
  - **if** a package is using a **nonstandard license**, then the `license-file` field may be specified:
    - example: `license-file = "LICENSE.txt"`
- `autobins = true|false` enables/disables **binary** *auto discovery*;
- `autoexamples = true|false` enables/disables **example** *auto discovery*;
- `autotests = true|false` enables/disables **test** *auto discovery*;
- `autobenches = true|false` enables/disables **bench** *auto discovery*;

<br>

### The `include`/`exclude` keys
The `include`/`exclude` list is also used for change **tracking** in some situations. If the package has a **build script** that does not emit any `rerun-if-*` directives, then the `include`/`exclude` list is used for tracking if the **build script** should be **re-run** if any of those files change.<br>

<br>

# Targets settings
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


# Specifying dependencies
## Specifying dependencies from crates.io
```toml
[dependencies]
time = "0.1.12"
```

The string `0.1.12` is a **version requirement**. Although it looks like a specific version of the time crate, it actually specifies a **range of versions** (aka **compatibility range**) and allows SemVer compatible updates.<br>

Cargo uses **SemVer** (`major`.`minor`.`patch`) for specifying version numbers. **Versions** are considered **compatible** if their **left-most non-zero component**  is the **same**. This convention also applies to versions with **leading zeros**.<br>
For example:
- `1.0.3` and `1.1.0` are **compatible**, but `1.1.0` to `2.0.0` are **not**;
- `0.1.0` and `0.1.2` are **compatible**, but `0.1.0` and `0.2.0` are **not**;

<br>

The **version requirement syntax**: 
- **caret requirement** is the **default** version requirement strategy;
  - example: `"^1.2.3"` or just `1.2.3`, appropriate **compatibility range** is `[1.2.3, 2.0.0)`;
- **tilde requirement** specifies a minimal version with some ability to update;
  - example: `"~1.2.3"`, its appropriate **compatibility range** is `[1.2.3, 1.3.0)`;
- **wildcard requirement** allows for any version where the wildcard `*` is positioned;
  - example: `"1.*"`, its appropriate **compatibility range** is `[1.0.0, 2.0.0)`;
- **equal requirement** specifies exact version only;
  - example: `"=1.2.3"`, its appropriate **compatibility range** is `[1.2.3]`;
- **compound requirement** allows multiple version requirements separated with a comma:
  - example: `">=1.2, <1.5"`, its appropriate **compatibility range** is `[1.2.0, 1.5.0)`;

<br>

More examples:
```
1.2.3  =>  [1.2.3, 2.0.0)
1.2    =>  [1.2.0, 2.0.0)
1      =>  [1.0.0, 2.0.0)
0.2.3  =>  [0.2.3, 0.3.0)
0.2    =>  [0.2.0, 0.3.0)
0.0.3  =>  [0.0.3, 0.0.4)
0.0    =>  [0.0.0, 0.1.0)
0      =>  [0.0.0, 1.0.0)

~1.2.3  => [1.2.3, 1.3.0)
~1.2    => [1.2.0, 1.3.0)
~1      => [1.0.0, 2.0.0)

*       => [0.0.0, +∞)
1.*     => [1.0.0, 2.0.0)
1.2.*   => [1.2.0, 1.3.0)
```

<br>

## Specifying dependencies from other registries
```toml
[dependencies]
some-crate = { version = "1.0", registry = "my-registry" }
```

<br>

## Specifying dependencies from git repositories
```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

Cargo assumes that we intend to use the **latest commit** on the **default branch** to build our package if we only specify the repo URL.<br>
You can combine the `git` key with the `rev`, `tag`, or `branch` keys to be more specific about which **commit to use**. Anything that is **not** a `branch` or a `tag` falls under `rev` key. This can be a **commit hash** like `rev = "4c59b707"`.<br>

```toml
# a commit with a particular tag
regex = { git = "https://github.com/rust-lang/regex.git", tag = "1.10.3" }

# a commit by its SHA1 hash
regex = { git = "https://github.com/rust-lang/regex.git", rev = "0c0990399270277832fbb5b91a1fa118e6f63dba" }
```

<br>

## Specifying path dependencies
```toml
[dependencies]
foo = { path = "../foo_dir" }
```

This tells Cargo that we **depend on** a crate called `foo` which is found in the `../foo_dir` folder, **relative** to the `Cargo.toml` file it’s written in.<br>

<br>

## Inheriting a dependency from a workspace
**Dependencies** can be **inherited** from a **workspace** by specifying the dependency in the workspace’s `[workspace.dependencies]` table. After that, add it to the `[dependencies]` table with `workspace = true`. Inherited dependencies also **can only** use `optional` and `features` keys, but they **cannot** use **any other** dependency key (such as `version` or `default-features`).

<br>

# Dependency resolution
One of Cargo’s primary tasks is to **determine the versions of dependencies to use** based on the **version requirements** specified in each package. This process is called **dependency resolution** and is performed by the **resolver**. The **result** of the *resolution* is stored in the `Cargo.lock`.<br>

<br>

## Dependency updates
1. `cargo build` **updates** versions in `Cargo.lock` if **new** *versions* of crates are considered **compatible** with **old**.
2. `cargo build` will **avoid** automatically using **pre-releases** unless explicitly asked. **SemVer** has the concept of **pre-releases** with a dash in the version, such as `1.0.0-alpha`, or `1.0.0-beta`.
3. If **version requirements** in `Cargo.toml` have been **modified**, then the **resolver** will select a **new version** for that dependency that matches the **new requirement**.

<br>

`cargo update` **without** any options updates **all packages** in the `Cargo.lock`. The `-p` **flag** can be used update for a **specific package**.
