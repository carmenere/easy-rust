# Semicolon `;`
Rust is an **expression-oriented language**. This means that **most things are expressions**.

<br>

**Semicolon** `;` is used to **turn** an *expression* **into** *statement*.

<br>

*Semicolon* **drops** *left part* and **returns** *right part*. If there is **nothing** *on the right* *semicolon* `;` returns [**unit type**](../types/unit.md).

<br>

*Semicolon* is **necessary** after **let bindings** and **assignments**.

<br>

Examples, when `;` is **necessary**:
- After **loop**, **control** and **match** blocks to drop their results:
```Rust
for { … };
```
```Rust
if/if else/else { … };
```
```Rust
match { … };
```
- After **let bindings** and **assignments**:
```Rust
let i = 5;
```
```Rust
let mut a = 5;
a = 10;
```

<br>

Examples, when `;` can be **omitted**:
```Rust
struct Foo {}
```
```Rust
enum Bar {}
```
```Rust
fn baz() {}
```