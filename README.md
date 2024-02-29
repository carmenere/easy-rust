# About
`easy-rust` contains easy manual of Rust language.

<br>

# Issues the Rust language solves
The `Rust` language **gets rid** of *some issues* that `C` and `C++` languages have.

<br>

|Problems in C/C++|Solution|
|:--------------------------|:-------|
|**Double free**|**Move semantics** and **scope based** resource management.|
|**Use after free** \(aka **dangling pointers**\)|**Borrow checker** tracks *references* **lifetimes**, i.e., *reference* **can’t outlive** its *owner*.|
|**Buffer overflow**|`slice` types and `.last()`/`.last_mut()` methods for **sequences**.|

<br>

# Docs
1. [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).
2. [The rustdoc book](https://doc.rust-lang.org/rustdoc/index.html).
3. [The Rust Edition Guide](https://doc.rust-lang.org/edition-guide/index.html).
4. [The rustc book](https://doc.rust-lang.org/rustc/index.html).

<br>

# Table of contents
1. **Rust toolchain**
      - [Toolchain](toolchain/toolchain.md)
      - [Rustup](toolchain/rustup.md)
      - [Rustc](toolchain/rustc.md)
      - [Cargo](toolchain/cargo.md)
      - [Lints](toolchain/lints.md)
      - [Features](toolchain/features.md)
      - [Attributes](toolchain/attributes.md)
      - [Rustfmt](toolchain/rustfmt.md)
      - [Clippy](toolchain/clippy.md)
2. **Workspaces. Packages. Crates**
      - [Workspaces. Packages. Crates](packages-and-crates/packages-and-crates.md)
      - [Modules](packages-and-crates/modules.md)
      - [Prelude](packages-and-crates/prelude.md)
      - [Registry and Index](packages-and-crates/registry-index.md)
3. **Basics**
      - [Comments](basics/basics.md)
      - [Formatted print](basics/formatted-print.md)
4. **Basic types**
      - [Basic types](basics/types.md)
5. **Ownership. Borrowing. Lifetimes**
      - [What is borrow checker?](ownership-borrowing/borrow-checker.md)
      - [Ownership](ownership-borrowing/ownership.md)
      - [Borrowing](ownership-borrowing/borrowing.md)
      - [Lifetimes](ownership-borrowing/lifetimes.md)
      - [Slices](ownership-borrowing/slices.md)
      - [Non lexical lifetimes examples](ownership-borrowing/nll-examples.md)
6. **Generics**
      - [Polymorphism](generics/polymorphism.md)
      - [Dispatch](generics/dispatch.md)
      - [Generics](generics/generics.md)
7. **Traits**
      - [Traits](traits/traits.md)
      - [Trait coherence](traits/trait-coherence.md)
      - [Trait objects](traits/trait-objects.md)
      - [Phantom data](traits/phantom-data.md)
8.  **Common traits**
       - [AsRef and AsMut](traits/utility-traits/AsRef-AsMut.md)
       - [Borrow and BorrowMut](traits/utility-traits/Borrow-BorrowMut.md)
       - [Clone](traits/utility-traits/Clone.md)
       - [Copy](traits/utility-traits/Copy.md)
       - [Debug](traits/utility-traits/Debug.md)
       - [Default](traits/utility-traits/Default.md)
       - [Deref and DerefMut](traits/utility-traits/Deref-DerefMut.md)
       - [Display](traits/utility-traits/Display.md)
       - [Drop](traits/utility-traits/Drop.md)
       - [Eq and PartialEq](traits/utility-traits/Eq-PartialEq.md)
       - [Fn, FnOnce and FnMut](traits/utility-traits/Fn-FnOnce-FnMut.md)
       - [From and TryFrom](traits/utility-traits/From-TryFrom.md)
       - [Into and TryInto](traits/utility-traits/Into-TryInto.md)
       - [Ord and PartialOrd](traits/utility-traits/Ord-PartialOrd.md)
       - [Pin and Unpin](traits/utility-traits/Pin-Unpin.md)
       - [Sized](traits/utility-traits/Sized.md)
       - [Sync and Send](traits/utility-traits/Sync-Send.md)
       - [ToOwned](traits/utility-traits/ToOwned.md)
       - [ToString](traits/utility-traits/ToString.md)
       - [Write, Read, BufRead and Seek](traits/utility-traits/Write-Read-BufRead-Seek.md)
9.  **Option and Result**
       - [Option](option-and-result/option.md)
       - [Result](option-and-result/result.md)
       - [Combinators](option-and-result/combinators.md)
         - [Option](option-and-result/combinators-option.md)
         - [Result](option-and-result/combinators-result.md)
10. **Error handling**
       - [Error handling](error-handling/error-handling.md)
       - [Anyhow and Thiserror](error-handling/anyhow-and-thiserror.md)
11. **Tests**
       - [Tests](tests/tests.md)
12. **Smart pointers**
       - [Box](smart-pointers/Box.md)
       - [Rc](smart-pointers/Rc.md)
       - [Arc](smart-pointers/Arc.md)
       - [Cell types and Interior mutability](smart-pointers/Interior-mutability-and-Cell-types.md)
       - [CoW](smart-pointers/CoW.md)
       - [Mutex and MutexGuard](smart-pointers/Mutex-MutexGuard.md)
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
       - [Function pointers](closures/function-pointers.md)
15. **Unsafe**
       - [Unsafe](unsafe/unsafe.md)
       - [Raw pointers](unsafe/raw-pointers.md)
16. **Macros**
       - [Macros](macros/macros.md)
       - [Examples](macros/examples.md)
17. **Concurrency**
       - [Synchronization primitives](synchronization-primitives.md)
       - [Threads](concurrency/threads/threads.md)
       - [Scoped threads](concurrency/threads/scoped-threads.md)
       - [Channels](concurrency/threads/channels.md)
       - [Async](concurrency/async/async.md)
18. **Useful crates**
       - *Crates for configuration*
         - [Overview](useful-crates/configure/configure.md)
       - *Hashmap*
         - [Insert and Get](useful-crates/hashmap/insert-get.md)
       - *Logging*
         - [Logging](useful-crates/logging/logging.md)
       - *Serde*
         - [Custom range type](useful-crates/serde/custom-range-type.md)
         - [serde_json](useful-crates/serde/serde_json.md)
