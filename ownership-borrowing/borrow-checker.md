**Borrow checker** is responsible for enforsing:
1. **Ownership rules**:
   1. **Each value** in Rust **has** an **identifier** that’s called its **owner**.
   2. There can only be **one owner at a time**.
   3. When the **owner** goes **out of scope**, the **value** will be **dropped**, (**scope based resource management**).
2. **Borrowing rules**:
   1. Scope of **mutable reference** `&mut T` **can’t** *intersect* with scope of any other reference to type `T`.
   2. Scope of **shared reference** `&T` **can** *intersect* with scope of any other **shared reference** to type `T`.
   3. Reference **can’t outlive value it points to**, i.e. the **borrow** must be **valid** **until** the **lender** is **destroyed**.
      - For example, function **cannot** return reference to value it owns.
   4. Since reference **doesn't own** the value it points to, **reference cannot move the value**. But **reference can copy value**.
3. **Lifetimes rules**:
   1. Each function’s parameter that is **reference** gets its own **lifetime parameter** (aka **elided lifetime**).
   2. If there is exactly **one input** *lifetime parameter*, it is assigned to **all output** *lifetime parameters*.
   3. If there are **multiple input** *lifetime parameters*, but one of them is `&self` or `&mut self`, the **lifetime** of `self` is assigned to **all output** *lifetime parameters*.
