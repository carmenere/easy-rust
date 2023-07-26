# Registry index
The `cargo` tool needs some efficient method of querying<br>
- what **packages** are available on a registry;
- what **versions** are available;
- what the **dependencies** for each version is;

<br>

The purpose of the **index** was to provide an efficient method to resolve the **dependency graph** for a package.<br>
After resolution has been performed, however we need to download the contents of packages so we can read the full manifest and build the source code.<br>

<br>

- each **file** in the **index** refers to particular crate and describes its history over time;
- each **entry** (line) in the file corresponds to one version of a crate and stored in JSON format (`cargo::sources::registry::RegistryPackage`).

<br>

Example of **file**:
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


## config.json
The `cargo` tool communicates with registry through a **git repository** aka **index** of a registry.<br>
The **root of the index** contains a `config.json` file with a few entries corresponding to the registry.<br>

```json
{
    "dl": "https://crates.io/api/v1/crates",
    "api": "https://crates.io"
}
```

- `dl`: this is the URL for downloading crates listed in the index. The value may have the markers which will be replaced with their corresponding value. If **none** of the markers are present, then the value `/{crate}/{version}/download` is appended to the `dl` value. Avaliable markers are:
  - `{crate}`: the **name of crate**.
  - `{version}`: the **crate version**.
  - `{prefix}`: a **directory prefix** computed from the crate name, for example, a crate named `cargo` has a prefix of `ca/rg`.
  - `{lowerprefix}`: lowercase variant of `{prefix}`.
  -` {sha256-checksum}`: the **crate’s sha256 checksum**.
- `api`: API endpoint for the registry. This key is optional, but if it is not specified, commands such as cargo **publish** will not work.

<br>

## Registry source
A **registry** consists of at least:
- a **git repository** that contains an **index** (aka **registry source**);
- **server** that contains the compressed `.crate` files created by `cargo package`;

<br>

The `crates.io` is the default **registry source**.<br>

**Sources** are described in `.cargo/config.toml` file in `[source]` section.

<br>

So, `https://github.com/rust-lang/crates.io-index` is a **default source** (**index**) of **default registry** `crates-io`.

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

### Merge rules
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

<br>

When being invoked from a workspace, Cargo does not read config files from crates within the workspace.

<br>

## Internals
The hardcoded constants:
```rust
pub const CRATES_IO_INDEX: &str = "https://github.com/rust-lang/crates.io-index";
pub const CRATES_IO_HTTP_INDEX: &str = "sparse+https://index.crates.io/";
pub const CRATES_IO_REGISTRY: &str = "crates-io";
pub const CRATES_IO_DOMAIN: &str = "crates.io";
```

<br>

- the `SourceConfigMap` data structure represents the entire `[source]` table in `.cargo/config.toml`.
- the `SourceConfig` data structure represents the configuration for a **particular source** in `.cargo/config.toml`, e.g., `[source.crates-io]`.

<br>

## Alternate registries
To use a registry other than **crates.io**, the **name** and **index URL** of the registry must be added to a `.cargo/config.toml` file.<br>
The `[registries]` table has a key for each registry.

<br>