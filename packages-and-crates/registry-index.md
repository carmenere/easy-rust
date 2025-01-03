# Table of contents
- [Table of contents](#table-of-contents)
- [Registry](#registry)
- [Index](#index)
  - [The layout of Index](#the-layout-of-index)
  - [`config.json`](#configjson)
- [Publish and yank](#publish-and-yank)
  - [cargo yank](#cargo-yank)
  - [cargo publish](#cargo-publish)
- [Alternate registries](#alternate-registries)
    - [Example](#example)
- [Source replacement](#source-replacement)
- [Example](#example-1)
  - [`.cargo/config.toml`](#cargoconfigtoml)
  - [Source replacement: `source.panamax`](#source-replacement-sourcepanamax)
    - [Download index](#download-index)
    - [Print out `config.json`](#print-out-configjson)
  - [Example of full URL to download `atomic-waker` from `source.panamax`](#example-of-full-url-to-download-atomic-waker-from-sourcepanamax)
  - [Alternate registry: `registries.mirror01`](#alternate-registry-registriesmirror01)
    - [Download index](#download-index-1)
    - [Print out `config.json`](#print-out-configjson-1)
  - [Example of full URL to download `some_crate` from `registries.mirror01`](#example-of-full-url-to-download-some_crate-from-registriesmirror01)
- [Merge rules for `.cargo/config.toml` files](#merge-rules-for-cargoconfigtoml-files)

<br>

# Registry
The **default registry** is `crates.io`.<br>

`cargo` fetches **packages** from a **registry**.<br>

Cargo supports two protocols for remote registry: `git` and `sparse`:
- `cargo` uses the `git` protocol by default;
- `cargo` uses the `sparse` protocol if the registry **index URL** **explicitly** starts with `sparse+`;

<br>

A **registry** consists of **3 components**:
- (*required*) **index** (aka **registry index**);
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
Each **cargo registry** provides an **index**. **Index** is a **git repository** following a **particular layout**.<br>
The **default index** is `https://github.com/rust-lang/crates.io-index`.<br>
The purpose of the **index** is to provide an efficient method to **resolve the dependency graph** for a package, i.e. `cargo` uses **index** to figure out which packages it must to download to build crate. After resolution has been performed, `cargo`uses **download endpoint** to download packeges: `GET $dl/$crate_name/$version/download` returns `.crate` file for appropriate crate.
**Index** contains **exactly one** file for each crate in the registry.<br>

<br>

## The layout of Index
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

<br>

## `config.json`
**Index** must contain `config.json` file in its **root**.<br>
`config.json` contains information used by `cargo` for accessing the **registry**.<br>

Example of `config.json`:<br>
```json
{
    "dl": "https://crates.io/api/v1/crates",
    "api": "https://crates.io",
    "auth-required": false
}
```
where:
- `dl` is the **download endpoint**, i.e. it is the URL for downloading crates listed in the index. 
- `api` is the URL of web API for the registry;
- `auth-required` indicates whether this is a **private registry** that requires all operations to be authenticated (crate downloads, API requests and so on).
  - If `auth-required` is set to `true`, then `cargo` must pass **auth token** in the `Authorization` header in all **download** requests and all requests to the **web API**.

The value of `dl` may have the following **markers** which will be replaced with their corresponding value:
  - `{crate}`: the **name** of crate;
  - `{version}`: the crate **version**;
  - `{prefix}`: a directory **prefix** computed from the crate name, for example, a crate named **cargo** has a prefix of **ca/rg**;
  - `{lowerprefix}`: lowercase variant of `{prefix}`;
  - `{sha256-checksum}`: the crate’s **sha256 checksum**;

If **none** of the markers are present, then the value `/{crate}/{version}/download` is appended to the end of `dl` **by default**.<br>

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

# Publish and yank
## cargo yank
The `cargo yank` command does not delete any data, and the crate will still be available for download via the registry’s download link.<br>
The `cargo yank` command **prevents new projects** from depending on a **yanked version** but it will **still be available** to projects that have a `Cargo.lock`.<br>

<br>

## cargo publish
The `cargo publish` command performs the following steps:
1. Perform some verification checks on your package.
2. Compress your source code into a `.crate` file.
3. Extract the `.crate` file into a temporary directory and verify that it compiles.
4. Upload the `.crate` file to `crates.io`.
5. The registry will perform some additional checks on the uploaded package before adding it.

<br>

# Alternate registries
Configuration of **alternative registries** is done through `.cargo/config`.<br>
The `[registries]` table is used for specifying **alternative registries**. It consists of a sub-tables `[registries.<name>]` for each **named registry**.<br>

The `registries.<name>.index` parameter specifies the **URL** of the **index** for the **registry**.<br>

<br>

### Example
```toml
[registries.my-registry]
index = "https://example.com/path/to/index"
```

Then specify **name** of **registry** for package in `Cargo.toml`:
```toml
[dependencies]
foobar = {version = "1.4.0", registry = "my-registry"}
```

<br>

The `[registry]` table controls the **default registry** used when one is not specified.

<br>

# Source replacement
A **source** (aka **registry source**) is one that is the same as `crates.io` itself. A **source** is a provider that contains crates. The `crates.io` is a **default** *source* and it is available under the name **crates-io**, e.g. `[source.crates-io]`.<br>
The `[source]` table in `.cargo/config.toml` is used for specifying **registry sources**. It consists of a sub-tables `[source.<name>]` for each **named source**. Every such **named source** must define **one kind** of *source* (**directory**, **registry**, **local registry**, or **git**).<br>

<br>

There are **several kinds** of *sources* and every kind has **special key** to be set:
- `source.<name>.directory` defines **path** to a **directory source**;
- `source.<name>.registry` defines **URL** to a **registry source**, in other word it sets URL of **index** of **registry**, e.g. `https://example.com/path/to/index`;
- `source.<name>.local-registry` defines **path** to a **local registry** source;
- `source.<name>.git` defines **URL** of a **git repository** source;

<br>

Configuration of **source replacement** is done through `source.<name>.replace-with = <some-source>|<some-registry>` parameter. **If set**, it **replaces** *current source* `<name>` with the given **named source** (`<some-source>`) or **named registry** (`<some-registry>`).

<br>

# Example
## `.cargo/config.toml`
```toml
[source.panamax]
registry = "http://mirror01/git/crates.io-index"

[registries.mirror01]
index = "http://mirror01/repository/crates/index"

[source.crates-io]
# To use sparse index, change "panamax" to "panamax-sparse".
replace-with = "panamax"

[net]
git-fetch-with-cli = true
```

<br>

To **download index** use `git clone`.<br>

<br>

## Source replacement: `source.panamax`
### Download index
```bash
git clone http://mirror01/git/crates.io-index
```

<br>

### Print out `config.json`
```bash
$ cat crates.io-index/config.json | jq '.'
{
  "dl": "http://mirror01/crates/{prefix}/{crate}/{version}/{crate}-{version}.crate",
  "api": "http://mirror01/crates"
}
$
```

<br>

## Example of full URL to download `atomic-waker` from `source.panamax`
```bash
curl -o atomic-waker-1.0.0.crate -v -X GET http://mirror01/crates/at/om/atomic-waker/1.0.0/atomic-waker-1.0.0.crate
```

<br>

## Alternate registry: `registries.mirror01`
### Download index
```bash
git clone http://mirror01/repository/crates/index
```

<br>

### Print out `config.json`
```bash
$ cat index/config.json | jq '.'
{
  "dl": "http://mirror01/repository/crates/api/v1/crates",
  "api": "http://mirror01/repository/crates",
  "auth-required": false,
  "allowed-registries": [
    "https://github.com/rust-lang/crates.io-index"
  ]
}
```

<br>

## Example of full URL to download `some_crate` from `registries.mirror01`
```bash
curl -o some_crate-1.2.0.crate -v -X GET http://mirror01/repository/crates/api/v1/crates/some_crate/1.2.0/download
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
