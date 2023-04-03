The `Rust` language **gets rid** of *some issues* that `C` and `C++` languages have.

<br>

|Problems in C/C++|Solution|
|:--------------------------|:-------|
|**Double free**|**Move semantics** and **scope based** resource management.|
|**Use after free** \(aka **dangling pointers**\)|**Borrow checker** tracks *references* **lifetimes**, i.e., *reference* **can’t outlive** its *owner*.|
|**Dereferencing `Null` pointers**||
|**Buffer overflow**|`slice` types and `.last()`/`.last_mut()` methods for **sequences**.|