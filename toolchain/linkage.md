# Linkage
The compiler supports various methods to link crates together both **statically** and **dynamically**.

<br>

## Static and dynamic C runtimes
The standard library in general strives to support both **statically** linked and **dynamically** linked **C runtimes** for targets.<br>
All targets in the compiler have a **default mode** of linking to the **C runtime**. Typically targets are linked **dynamically by default**.<br>

The `crt-static` **feature** of `target-feature` **codegen option** configure **linkage** of the *C runtime*:
- `-C target-feature=+crt-static` links to the *C runtime* **statically**;
- `-C target-feature=-crt-static` links to the *C runtime* **dynamically**;

It's recommended to **inspect the resulting binary** to ensure that it's linked as you would expect after the compiler succeeds.<br>

<br>

## Native-linking crates
A **native dependency** is any dependency that requires compilation of C++/C, because they **rely on code that is compiled on the machine**.<br>

You may be **unable** to **statically** link your binary due to **dependencies** that **mandate dynamic linking**.<br>
There is convention that all **native-linking crates** use the `-sys` suffix in their name.<br>
So, it is fairly simple to **find dependencies that dynamically link to libraries**:
- `cargo tree | grep '\-sys'`
