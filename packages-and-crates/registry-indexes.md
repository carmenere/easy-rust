# Table of contents
- [Table of contents](#table-of-contents)
- [Registry and index](#registry-and-index)
- [Index](#index)
  - [The Format of The Index](#the-format-of-the-index)
- [Source](#source)
    - [Example](#example)
- [Alternate registries](#alternate-registries)
- [cargo yank](#cargo-yank)
- [cargo publish](#cargo-publish)
    - [Merge rules for `.cargo/config.toml` files](#merge-rules-for-cargoconfigtoml-files)

<br>

# Registry and index
The **default registry** is `crates.io`.<br>

`cargo` fetches **packages** from a **registry**.<br>

Cargo supports two protocols for remote registry: `git` and `sparse`:
- `cargo` uses the `git` protocol by default;
- `cargo` uses the `sparse` protocol if the registry **index URL** **explicitly** starts with `sparse+`;

<br>

A **registry** consists of **3 components**:
- (*required*) **index**;
- (*required*) **download endpoint** at the location defined in `config.json` which is used used by `cargo` to donwload `.crate` files created by `cargo package`:
  - `GET $dl/$crate_name/$version/download` must return `.crate` file for appropriate crate;
  - the **sha256sum** of the `.crate` file needs to match the the checksum in the index file for that version of the crate.
- (*optional*) **web API** at the location defined in `config.json` to support actions listed below:
  - **publish**:
    - `PUT $api/api/v1/crates/new`
  - **yank**:
    - `DELETE $api/api/v1/crates/$crate_name/$version/yank`
  - **unyank**:
    - `PUT $api/api/v1/crates/$crate_name/$version/unyank`
  - **search**:
    - `GET $api/api/v1/crates`
  - **login** (solely for the `cargo login`);

<br>

# Index
The **default index** is `https://github.com/rust-lang/crates.io-index`.<br>

Each **cargo registry** provides an **index**, which is a **git repository** following a **particular format**.<br>
The purpose of the **index** is to provide an efficient method to **resolve the dependency graph** for a package, i.e. `cargo` uses **index** to figure out which packages it must to download to build crate.<br>
The index contains **exactly one** file for each crate in the registry.<br>
After resolution has been performed, `cargo`uses **download endpoint** to download packeges: `GET $dl/$crate_name/$version/download` returns `.crate` file for appropriate crate.

<br>

## The Format of The Index
```
.
|
...
├── 3
│   └── u
│       └── url
├── bz
│   └── ip
│       └── bzip2
...
├── en
│   └── co
│       └── encoding
├── li
    ├── bg
    │   └── libgit2
    └── nk
        └── link-config
...
└── config.json
```

There are three **special directories**: `1`, `2` and `3` for crates with names 1, 2, and 3 characters in length.<br>
The directories `1` and `2` simply have the crate files underneath them, while the directory `3` is sharded by the first letter of the crate name.<br>

There is a `config.json` file in the **root** of the **index** which contains some information used by `cargo` for accessing the **registry**.<br>

```json
{
    "dl": "https://crates.io/api/v1/crates",
    "api": "https://crates.io",
    "auth-required": false
}
```
where:
- `dl` is the **download endpoint**, i.e. it is the URL for downloading crates listed in the index;
- `api` is the URL of web API for the registry;
- `auth-required` indicates whether this is a **private registry** that requires all operations to be authenticated (crate downloads, API requests and so on).
  - If `auth-required` is set to `true`, then `cargo` must pass **auth token** in the `Authorization` header in all **download** requests and all requests to the **web API**.

<br>

Example of **file** in **index**:
`https://github.com/rust-lang/crates.io-index/blob/master/ac/ti/actix-multipart`<br>

<br>

Example of **entry**:<br>
```json
{
    "name": "actix-multipart",
    "vers": "0.6.0",
    "deps": [
        {
            "name": "actix-multipart-derive",
            "req": "=0.6.0",
            "features": [],
            "optional": true,
            "default_features": true,
            "target": null,
            "kind": "normal"
        },

        ...

    ],
    "cksum": "dee489e3c01eae4d1c35b03c4493f71cb40d93f66b14558feb1b1a807671cc4e",
    "features": {
        "default": [
            "tempfile",
            "derive"
        ],
        "derive": [
            "actix-multipart-derive"
        ],

        ...
    },
    "yanked": false
}
```

<br>

# Source
**Sources** are described in `.cargo/config.toml` file in `[source]` section.<br>
**Source** can contain **more than 1 registry**.

<br>

### Example
```toml
# For example this section defines a new source, called `my-vendor-source`, which comes from a directory
# located at `vendor` relative to the directory containing this `.cargo/config.toml` file
[source.my-vendor-source]
directory = "vendor"

# The crates.io default source for crates is available under the name "crates-io", and here we use 
# the `replace-with` key to indicate that it's replaced with our source above.
# The `replace-with` key can also reference an alternative registry name defined in the `[registries]` table.
[source.crates-io]
registry = 'https://github.com/rust-lang/crates.io-index'
replace-with = "my-vendor-source"

# Several kinds of sources can be specified (described in more detail below):
registry = "https://example.com/path/to/index"
local-registry = "path/to/registry"
directory = "path/to/vendor"
```

<br>

# Alternate registries
To use **alternate registry**, the **name** of **registry** and its **index URL** must be added to a `.cargo/config.toml` file.<br>
Example:<br>
```toml
[registries.my-registry]
index = "https://gitlab.com/my-organization/my-registry"
```

Then specify **name** of **registry** for package in `Cargo.toml`:
```toml
[dependencies]
foobar = {version = "1.4.0", registry = "my-registry"}
```

<br>

# cargo yank
The `cargo yank` command does not delete any data, and the crate will still be available for download via the registry’s download link.<br>
The `cargo yank` command **prevents new projects** from depending on a **yanked version** but it will **still be available** to projects that have a `Cargo.lock`.<br>

<br>

# cargo publish
The `cargo publish` command performs the following steps:
1. Perform some verification checks on your package.
2. Compress your source code into a `.crate` file.
3. Extract the `.crate` file into a temporary directory and verify that it compiles.
4. Upload the `.crate` file to `crates.io`.
5. The registry will perform some additional checks on the uploaded package before adding it.

<br>

### Merge rules for `.cargo/config.toml` files
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
