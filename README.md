# easy-rust
Easy manual for Rust language.

<br>

# Issues Rust solves
The ``Rust`` language **gets rid** of *some issues* that ``C`` and ``C++`` languages have.

<br>

|Problems in C/C++|Solution|
|:--------------------------|:-------|
|**Double free**|**Move semantics** and **scope based** resource management.|
|**Use after free** \(aka **dangling pointers**\)|**Borrow checker** tracks *references* **lifetimes**, i.e., *reference* **can’t outlive** its *owner*.|
|**Buffer overflow**|``slice`` types and ``.last()``/``.last_mut()`` methods for **sequences**.|

<br>

# Table of contents
1. **Rust toolchain**
      - [Toolchain](toolchain/toolchain.md)
      - [Rustup](toolchain/rustup.md)
      - [Rustc](toolchain/rustc.md)
      - [Cargo](toolchain/cargo.md)
      - [Rustfmt](toolchain/rustfmt.md)
      - [Clippy](toolchain/clippy.md)
      - [Lints](toolchain/lints.md)
      - [Features](toolchain/features.md)
      - [Attributes](toolchain/attributes.md)
2. **Workspaces. Packages. Crates**
      - [Workspaces. Packages. Crates](packages-and-crates/packages-and-crates.md)
      - [Modules](packages-and-crates/modules.md)
      - [Prelude](packages-and-crates/prelude.md)
3. **Basics**
      - [Comments](basics/comments.md)
      - [Constants](basics/constants.md)
      - [Let bindings](basics/let-bindings.md)
      - [Loops](basics/loops.md)
      - [Operators](basics/operators.md)
      - [Control flow](basics/control-flow.md)
      - [Functions](basics/functions.md)
      - [Closures](basics/closures.md)
      - [Formatted print](basics/formatted-print.md)
      - [Formatted strings](basics/formatted-strings.md)
      - [Iterators](basics/iterators.md)
      - [Semicolon](basics/semicolon.md)
      - [Assertions](basics/assertions.md)
      - [Docstrings](basics/docstrings.md)
4. **Types**
      - [Scalars](types/scalars.md)
      - [Enums](types/enums.md)
      - [Arrays](types/arrays.md)
      - [Vectors](types/vectors.md)
      - [Strings](types/strings.md)
      - [Structs](types/structs.md)
      - [Tuples](types/tuples.md)
      - [Unit](types/unit.md)
      - [Tuple-structs](types/tuple-structs.md)
      - [Unit-like structs](types/unit-like-structs.md)
      - [Newtype pattern](types/new-type-pattern.md)
      - [Dynamically sized types (DST)](types/dst.md)
5. **Ownership. Borrowing. Lifetimes**
      - [Ownership](ownership-borrowing-lifetimes/ownership.md)
      - [Borrowing](ownership-borrowing-lifetimes/borrowing.md)
      - [Lifetimes](ownership-borrowing-lifetimes/lifetimes.md)
      - [Non lexical lifetime](ownership-borrowing-lifetimes/nll.md)
      - [Slices](ownership-borrowing-lifetimes/slices.md)
6. **Generics**
      - [Polymorphism](generics/polymorphism.md)
      - [Dispatch](generics/dispatch.md)
      - [Generics](generics/generics.md)
7. **Traits**
      - [Traits](traits/traits.md)
      - [Trait coherence](traits/trait-coherence.md)
      - [Trait objects](traits/trait-objects.md)
      - [Operator overloading](traits/operator-overloading.md)
      - [Phantom data](traits/phantom-data.md)
8. **Type convertions**
      - [Into and TryInto](type-convertions/Into-and-TryInto.md)
      - [From and TryFrom](type-convertions/From-and-TryFrom.md)
      - [Examples](type-convertions/examples.md)
9.  **Utility traits**
       - [Intro](utility-traits/intro.md)
       - [AsRef](utility-traits/AsRef.md)
       - [Borrow](utility-traits/Borrow.md)
       - [Deref](utility-traits/Deref.md)
       - [ToOwned](utility-traits/ToOwned.md)
10. **Option and Result**
       - [Option](option-and-result/option.md)
       - [Result](option-and-result/result.md)
       - [Combinators](option-and-result/combinators.md)
         - [Option](option-and-result/combinators-option.md)
         - [Result](option-and-result/combinators-result.md)
11. **Error handling**
       - [Approaches](error-handling/approaches.md)
       - [Try and Question mark (?)](error-handling/try-and-question.md)
       - [Panic](error-handling/panic.md)
       - [Custom error type](error-handling/custom-error-type.md)
       - [Anyhow and Thiserror](error-handling/anyhow-and-thiserror.md)
12. **Smart pointers**
       - [Intro](smart-pointers/intro.md)
       - [Arc](smart-pointers/Arc.md)
       - [Box](smart-pointers/Box.md)
       - [Cell](smart-pointers/Cell.md)
       - [CoW](smart-pointers/CoW.md)
       - [Rc](smart-pointers/Rc.md)
       - [RefCell](smart-pointers/RefCell.md)
       - [Self as smart pointer](smart-pointers/self-as-smart-pointer.md)
13. **Iterators**
       - [Iterators](iterators/iterators.md)
       - [Iterables](iterators/iterables.md)
       - [Loops](iterators/loops.md)
       - [Adapters](iterators/adapters.md)
       - [IntoIter for Vec](iterators/intoiter-for-vec-example.md)
       - [FromIterator](iterators/FromIterator.md)
14. **Closures**
       - [Closures](closures/closures.md)
15. **Macros**
       - [Macros](macros/macros.md)
       - [Unimplemented vs. Todo](macros/unimplemented-vs-todo.md)
       - [Examples](macros/examples.md)
16. **Useful crates**
       - *Crates for configuration*
         - [Overview](useful-crates/configure/configure.md)
       - *Hashmap*
         - [Insert and Get](useful-crates/hashmap/insert-get.md)
       - *Logging*
         - [Logging](useful-crates/logging/logging.md)
       - *Serde*
         - [Custom range type](useful-crates/serde/custom-range-type.md)
         - [serde_json](useful-crates/serde/serde_json.md)
