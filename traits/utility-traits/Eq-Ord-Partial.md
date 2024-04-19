# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declarations](#declarations)
  - [`PartialEq`](#partialeq)
  - [`Eq`](#eq)
  - [`PartialOrd`](#partialord)
  - [`Ord`](#ord)
- [In a nutshell](#in-a-nutshell)
  - [Correspondence to binary relations](#correspondence-to-binary-relations)
  - [`PartialEq`](#partialeq-1)
  - [`Eq`](#eq-1)
  - [`PartialOrd`](#partialord-1)
  - [`Ord`](#ord-1)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`PartialEq`|[std::cmp::PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)|
|`Eq`|[std::cmp::Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)|
|`PartialOrd`|[std::cmp::PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)|
|`Ord`|[std::cmp::Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)|

<br>

# Declarations
## `PartialEq`
```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    // Required method
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
```

<br>

## `Eq`
```rust
pub trait Eq: PartialEq<Self> { }
```

<br>

## `PartialOrd`
```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    // Required method
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
```

<br>

## `Ord`
```rust
pub trait Ord: Eq + PartialOrd<Self> {
    // Required method
    fn cmp(&self, other: &Self) -> Ordering;

    fn max(self, other: Self) -> Self { ... }
    fn min(self, other: Self) -> Self { ... }
    fn clamp(self, min: Self, max: Self) -> Self { ... }
}
```

<br>

# In a nutshell
## Correspondence to binary relations
1. **Partial equivalence relation** is **not reflexive**, i.e., `a != a`.
2. **Full equivalence relation** must satisfy **reflexivity**: any value a is equal to itself.
3. **Partial order relation** means that **not every pair of elements are comparable**.
4. **Total order relation** means that **every pair is comparable**.

<br>

|Trait|Relation type|
|:----|:------------|
|`PartialEq`|`PartialEq` is for types that form a [**partial equivalence**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#partial-equivalence-relation).|
|`Eq`|`Eq` is **marker trait**. It is just a **promise** that type is **reflexive**. This is for types that form a [**full equivalence**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#equivalence-relation).|
|`PartialOrd`|`PartialOrd` is for types that form a **partial order**: [**weak partial order**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#weak-partial-order-or-just-partial-order-) (`≤`, `≥`) or [**strict partial order**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#strict-partial-order-) (`<`, `>`).|
|`Ord`|`Ord` is for types that form a **total order**: [**weak total order**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#weak-total-order-or-just-total-order-) (`≤`, `≥`) or  [**strict total order**](https://github.com/carmenere/easy-math/blob/main/set-theory/binary-relations.md#strict-total-order-) (`<`, `>`).|

All 4 traits are **derivable** and can be used with `#[derive]`.

<br>

## `PartialEq`
- This trait allows for comparisons using the **equality operator**, for types that **don't** have a **full equivalence relation**.
  - For example, in **floats** `NaN != NaN`, so **floats** implement `PartialEq` but **not** `Eq`.
- `PartialEq` **isn't** suitable to be **key** in a **hash** table.
- Implementations must ensure that `eq` and `ne` are consistent with each other: `a != b` **if and only if** `!(a == b)`.
- By default, `Rhs=Self`. If `Rhs != Self`, for example, `impl PartialEq<Rhs=B> for A`, we can **compare different types**: `A` and `B`.

<br>

## `Eq`
- Implement `Eq` in addition to `PartialEq` if it’s guaranteed that `PartialEq::eq(a, a)` always returns **true** (**reflexivity**).
- `Eq` **allows** type to be **key** in a **hash** table.

<br>

## `PartialOrd`
- `PartialOrd` is useful for types which **don't** have a **total order**, in other words **not** every pair of elements are **comparable**.
  - For example, for **floats**:
    - `NaN < 0 == false`;
    - `NaN >= 0 == false`;
- `partial_cmp()` returns `Option<Ordering>` ([Ordering](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)):
```rust
#[repr(i8)]
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```
- The `<` and `>` operators behave according to a **strict partial order**.
- The `<=` and `>=` **don't** behave according to a **weak partial order**. That is because a **weak partial order** requires **reflexivity**, i.e. `a <= a` must be `true` **for every** `a`.

<br>

## `Ord`
- Implementations must be consistent with the `PartialOrd` implementation, and ensure `max`, `min`, and `clamp` are consistent with `cmp`. It’s easy to accidentally make `cmp` and `partial_cmp` **disagree** by deriving some of the traits and manually implementing others.