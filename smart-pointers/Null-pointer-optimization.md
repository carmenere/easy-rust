# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)

<br>

# URLs
- [**Null pointer optimization**](https://doc.rust-lang.org/std/option/index.html#representation);
- [**std::mem::size_of**](https://doc.rust-lang.org/std/mem/fn.size_of.html);
- [**std::mem::align_of**](https://doc.rust-lang.org/std/mem/fn.align_of.html);
- [**ABI compatibility**](https://doc.rust-lang.org/std/primitive.fn.html#abi-compatibility);

<br>

# In a nutshell
Rust guarantees that that:
- `Option<T>` is **exactly** the **same** *size*, *alignment* and *function call ABI* as `T`;
- `Option<&T>` is **exactly** the **same** *size*, *alignment* and *function call ABI* as `&T`;

This is called the **null pointer optimization** or **NPO**.<br>

In other words, Rust guarantees that:
- `size_of::<T>()` **equals** `size_of::<Option<T>>()`;
- `align_of::<T>()` **equals** `align_of::<Option<T>>()`;

<br>


