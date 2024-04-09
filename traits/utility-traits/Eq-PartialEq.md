# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [PartialEq vs. Eq](#partialeq-vs-eq)
  - [`PartialEq`](#partialeq)
  - [`Eq`](#eq)
  - [Float](#float)
- [PartialOrd vs. Ord](#partialord-vs-ord)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Eq`|[std::cmp::Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)|
|`PartialEq`|[std::cmp::PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)|

<br>

# PartialEq vs. Eq
- [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
- [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)

<br>

## `PartialEq`
`PartialEq` trait is for **partial equality**. <br>

To implement `PartialEq` trait the **equality relation** `==` must satisfy the following conditions:
- **symmetric**: if `A: PartialEq<B>` and `B: PartialEq<A>`, then `a == b` implies `b == a`;
- **transitive**: if `A: PartialEq<B>` and `B: PartialEq<C>` and `A: PartialEq<C>`, then `a == b` and `b == c` implies `a == c`.

<br>

## `Eq`
`Eq` trait requires the **equality relation** `==` to be **reflexive**: `a == a`.<br>
`Eq` is a **marker** trait which merely indicates that in addition to implementing `PartialEq`.<br>
It's just a promise that your type is **reflexive**. This gives it additional functionality like being able to be the key in a hash map, which `PartialEq` is not suitable for.<br>

## Float
Floats have `NaN` which is not reflexive, because `NaN != NaN`.<br>
So, **float types** implement `PartialEq` but **not** `Eq`.<br>

<br>

# PartialOrd vs. Ord
- [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)<br>
- [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
