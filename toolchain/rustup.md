# Description
``rustup`` is a **toolchain multiplexer**. <br>
``rustup`` can **install** and **manage** *many* toolchains simultaneously. <br>
``rustup`` provides mechanisms to easily change the **active** (**default**) toolchain.

<br>

# Installation
Install ``rustup``:
### Latest toolchain
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>

### Customized toolchain
```bash
TARGET=aarch64-unknown-linux-musl
VERSION=1.58.1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${VERSION}-${TARGET}
```

<br>

# Profiles
``rustup`` has the concept of **profiles**. 

**Profile** is a **group of components** you can choose to download while installing a *toolchain*. 

The profiles available now:
- ``minimal``
- ``default``
- ``complete``

<br>

# ``rustup`` subcommands
|Subcommand|Explanation|
|:---------|:----------|
|rustup **show**|Print **default** *target triple* and other information.|
|rustup **toolchain list**|Lists all installed toolchains.|
|rustup **toolchain install** ``toolchain``|Installs last toolchain for release channel ``toolchain`` for current date.|
|rustup **default** ``toolchain``|Sets the **default toolchain** to ``toolchain``.|
|rustup **target add** ``toolchain``|Enables **cross compilation** for ``toolchain`` for **current toolchain**.|
|rustup **update**|Updates **toolchains** that begin with ``stable``-`*`.|
|rustup **component list**|Lists all **available** and **installed** components.|
|rustup **component add** ``rust-docs``|Adds component ``rust-docs`` to default toolchain.|
|rustup **set profile** ``minimal``|To select the ``minimal`` **profile** you can use.|
