# Table of contents
- [Table of contents](#table-of-contents)
- [Specifying dependencies](#specifying-dependencies)
  - [Specifying dependencies from crates.io](#specifying-dependencies-from-cratesio)
  - [Specifying dependencies from other registries](#specifying-dependencies-from-other-registries)
  - [Specifying dependencies from git repositories](#specifying-dependencies-from-git-repositories)
  - [Specifying path dependencies](#specifying-path-dependencies)
  - [Inheriting a dependency from a workspace](#inheriting-a-dependency-from-a-workspace)
- [Dependency resolution](#dependency-resolution)
  - [Dependency updates](#dependency-updates)

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
- **tilde requirement**
  - if you specify a **major**, **minor**, and **patch** version, then **only patch-level changes** are **allowed**;
  - if you specify a **major** and **minor** version, then **only patch-level changes** are **allowed**;
  - if you **only** specify a **major** version, then **minor-level** and **patch-level** changes are **allowed**;
  - examples:
    - `"~1.2.3"`, its appropriate **compatibility range** is `[1.2.0, 1.3.0)`;
    - `"~1.2"`, its appropriate **compatibility range** is `[1.2.0, 1.3.0)`;
    - `"~1"`, its appropriate **compatibility range** is `[1.0.0, 2.0.0)`;
- **wildcard requirement** allows for any version where the wildcard `*` is positioned;
  - example: `"1.*"`, its appropriate **compatibility range** is `[1.0.0, 2.0.0)`;
- **equal requirement** specifies exact version only;
  - example: `"=1.2.3"`, its appropriate **compatibility range** is `[1.2.3, 1.2.3]`;
- **compound requirement** allows multiple version requirements separated with a comma:
  - example: `">=1.2, <1.5"`, its appropriate **compatibility range** is `[1.2.0, 1.5.0)`;

<br>

**More examples**:
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

<br>