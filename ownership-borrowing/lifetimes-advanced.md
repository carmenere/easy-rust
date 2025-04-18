# Table of contents
<!-- TOC -->
* [Definitions](#definitions)
  * [At most. At least](#at-most-at-least)
  * [Lexical scope. Liveness scope](#lexical-scope-liveness-scope)
  * [Lifetimes](#lifetimes)
  * [Liveness](#liveness)
  * [Regions](#regions)
  * [Universal region](#universal-region)
  * [Example](#example)
* [Subtyping and variance](#subtyping-and-variance)
  * [Notations](#notations)
  * [Variance over lifetime `'a`](#variance-over-lifetime-a)
  * [Variance over generic type `T`](#variance-over-generic-type-t)
* [Nested references](#nested-references)
  * [Nested references coercions](#nested-references-coercions)
    * [Example 1](#example-1)
    * [Example 2](#example-2)
    * [Example 3](#example-3)
  * [Dereferencing nested references](#dereferencing-nested-references)
* [Region inference](#region-inference)
  * [Liveness constraints](#liveness-constraints)
  * [Location-aware subtyping constraints](#location-aware-subtyping-constraints)
  * [Example on subtyping constraints](#example-on-subtyping-constraints)
* [Drop as last use](#drop-as-last-use)
* [Borrowing a variable for longer than its scope](#borrowing-a-variable-for-longer-than-its-scope)
<!-- TOC -->

<br>

# Definitions
## At most. At least
**At most as long as** means **less than or equal to** (**<=**) or **cannot be longer**.<br>
**At least as long as** means **greater than or equal to** (**>=**) or **cannot be shorter**.<br>

<br>

## Lexical scope. Liveness scope
**Lexical scope** usually refers to **variables** (**identifier**).<br>
_Lexical scope_ is a **part of code** where particular **variable** is **valid**. So, **lexical scope** is a **scope of variable**.<br>
_Lexical scope_ **starts** from the point where **variable** is **declared** by `let` keyword and **ends** to the end of lexical scope `}`.<br>

**liveness scope** usually refers to the **actual value** that a **variable is bound to**.<br>
_Liveness scope_ is a **part of code** where particular **value** is **valid**.<br>
_Liveness scope_ **starts** from the point where **value** is **created** and **ends** where **value** is **dropped** or **reassigned**.<br>

<br>

**Example**:
```rust
fn main() {
    let mut v; //---------------------------------------------------------------------+-- lexical scope of viable v
    {                                        //                                       |
        v = Box::new(10);                    //--------+-- liveness scope of Box<10>  |
        println!("{}", v);                   //        |   that is bound to v         |
        drop(v);                             //<-------+                              |
                                             //                                       |
        v = Box::new(20);                    //--------+-- liveness scope of Box<20>  |
        println!("{}", v);                   //        |   that is bound to v         |
    }                                        //<-------+                              |
    {                                        //                                       |
        v = Box::new(30); println!("{}", v); //--------+-- liveness scope of Box<30>  |
                                             //        |   that is bound to v         |
    }                                        //<-------+                              |
} //<---------------------------------------------------------------------------------+
```

<br>

**In common** _lexical scope_ **is not equal** _liveness scope_, because variables can be **dropped** or **reassigned**.<br>
But **in particular** _lexical scope_ **can be equal to** _liveness scope_.<br>

<br>

## Lifetimes
A **reference type** has a **lifetime** which is a **part of its type**.<br>
The borrow must last **at most as long as** the **liveness scope** of **borrowed value** (**referent**). 

<br>

Lifetimes appear in various places:
- a **lifetime** can be a **part** of the **reference type**: `let mut p: &'p T`;
- a **lifetime** can be a **part** of the **borrow expression** (aka **reference** or **borrow**): `p = &'foo foo;`;

<br>

The **lifetime** of the **reference** must be **at most as long as** the **liveness scope** of the **value** the **reference points to**.<br>
In other words, **lifetime** of the **borrow expression** cannot be **longer** than the **liveness scope** of the **borrowed value**.<br>

<br>

## Liveness
The key ingredient to understanding how **NLL** should work is understanding **liveness**.<br>
A **variable** is **live** if the current **value** that **it holds** may be **used later**.<br>

**Example**:
```rust
let mut foo, bar;
let p = &foo;
// `p` is live here: its value may be used on the next line.
if condition {
    // `p` is live here: its value will be used on the next line.
    print(*p);
    // `p` is DEAD here: its value will not be used.
    p = &bar;
    // `p` is live here: its value will be used later.
}
// `p` is live here: its value may be used on the next line.
print(*p);
// `p` is DEAD here: its value will not be used.
```

The key point is that variable `p` becomes **dead before** it is **reassigned**.<br>
So how does **liveness** relate to **NLL**? The key rule is this: **whenever a variable is live, all references that it may contain are live**.<br>

<br>

## Regions
We can think about **lifetime of a reference** as a **region of the CFG** or just **region**.<br>
Each **region** can be represented as a **set of points** in the **CFG**.<br>
Each **point** in the **CFG** is represented by the **pair** of a **basic block** and an **index** (**basic block**/**index**).<br>
The term **region** is often used instead of **lifetime**.<br>

<br>

For each **reference type** compiler _implicitly assign_ a **region as part of the type**.<br>
For each **borrow expression** like `&foo` compiler _implicitly assign_ a **region** to represent the **lifetime of the borrow**.<br>

<br>

A new **NLL-based borrow-checker** processes **MIR** rather than the **AST**.<br>

The code in section `Liveness` above being transformed to **MIR** will look like:
```shell
// let mut foo: i32;
// let mut bar: i32;
// let p: &'p i32;

A
[ p = &'foo foo  ]            A/0
[ if condition ] ----\ (true) A/1
       |             |
       |     B       v
       |     [ print(*p)     ] B/0
       |     [ ...           ] B/1
       |     [ p = &'bar bar ] B/2
       |     [ ...           ] B/3
       |     [ goto C        ] B/4
       |             |
       +-------------/
       |
C      v
[ print(*p)    ] C/0
[ return       ] C/1
```

**Explanation**:
- the `A`, `B` and `C` are **basic blocks**;
- `A/0` refers to `p = &foo`;
- `B/2` refers to `p = &bar`;
- `A/1`, `B/4`, and `C/1` refer to **terminators** in `A`, `B` and `C` respectively;

<br>

## Universal region
Consider code:
```rust
fn foo<'a>(x: &'a u32) -> &'a i32 {
x
}
```
here region `'a` is a **universal region** (aka **named lifetime parameter**).<br>

If function returns **type** that **includes** a **universal region**, e.g. `&'a i32`, this means that compiler will add **element** `end('a)` to its **region variable** `'a`.<br>
If region includes element `end('a)`, then this implies that `'a` must be **extended** until the **end** of `'a` in the **caller**, in other words the **region** `'a` must be **extended** beyond function body **until last use** of returned reference in the **caller**. <br>

<br>

In the function `foo` above the **function argument** `x` and **returning result** are both **references** and both have **the same region** `'a`, this implies that the **referent** `x` points to **must remain borrowed** until **last use** of **returned reference** in the **caller**.<br>

<br>

## Example
Consider code:
```rust
fn foo<'a>(x: &'a String) -> &'a String {
  x
}

fn caller() {
  let mut s = String::from("abc");
  let r = {
    let r = foo(&s);    // ----+-- 'a
    // +-- foo()--+     //     |
    // |   ....   |     //     |
    // +----------+     //     |
    println!("{}", r);  //     |
    r                   //     |
  };                    //     |
  println!("{}", r);    // <---+
  let s = &mut s;
}

fn main() {
  caller()
}
```

In the above example, universal region is extended until last use of `r` and liveness scope of value is longer than region `'a`.<br>

<br>

But, in the code below, **liveness scope** of value **cannot** be extended to the region `'a`:
```rust
fn caller() {
  let r = {
    let mut s = String::from("abc"); //---+-- lexical scope of s and 
    let r = foo(&s);   // ----+-- 'a     |    liveness scope of String::from("abc")
    // +-- foo()--+    //     |          |
    // |   ....   |    //     |          |
    // +----------+    //     |          |
    println!("{}", r); //     |          |
    r                  //     |          |
  }; //<----------------------|----------+
  println!("{}", r); // <-----+
}
```

<br>

# Subtyping and variance
**Subtyping** is the idea that one type (aka **subtype**) `Sub` can be used in place of another `Super`. Notation: `Sub <: Super`.<br>
**Regions** (**lifetimes**) are connected through **subtyping**: in Rust **lifetimes** relate to each other as: `'long <: 'short` or `'long: 'short`.<br>

The fact that `'l <: 's` implies that `&'l T <: &'s T`. This is known as **variance**.<br>
**Variance** defines **subtyping relationships** between **types**.<br>

There are **three** kinds of **variance**:
- if type `T` is a **subtype** of `U` **and** type `F<T>` is also **subtype** of `F<U>`, then type `F<T>` is **covariant** over `T`; 
- if type `T` is a **subtype** of `U`, **but** type `F<U>` is a **subtype** of `F<T>`, then type `F<T>` is **contravariant** over `T`
- if **no subtyping relation can be derived** between `F<U>` and `F<T>` types, then type `F<T>` is **invariant** over `T`;

<br>

In Rust **variance** can be over **lifetime** and/or over **generic type**:

| Type                | over `'a`   | over `T`      |
|:--------------------|:------------|:--------------|
| `&'a T`             | _covariant_ | _covariant_   |
| `&'a mut T`         | _covariant_ | **invariant** |
| `*const T`          | n/a         | _covariant_   |
| `*mut T`            | n/a         | **invariant** |
| `UnsafeCell<T>`     | n/a         | **invariant** |
| `Cell<T>`           | n/a         | **invariant** |
| `Box<T>`            | n/a         | _covariant_   |
| `Vec<T>`            | n/a         | _covariant_   |
| `PhantomData<T>`    | n/a         | _covariant_   |
| `[T]` and `[T; n]`  | n/a         | _covariant_   |
| `dyn Trait<T> + 'a` | _covariant_ | **invariant** |

Explanations:
- the **raw pointers** have **no** lifetime;
- types `UnsafeCell<T>` and `Cell<T>` are **invariant** over `T`, they follow the logic of `&mut T` because of **interior mutability**;

<br>

> Note:<br>
> The **functional type** `fn(T) -> U` is **contravariant** over `T` and **covariant** over `U`.<br>

<br>

## Notations
Notations:
- `'l` means `'long`;
- `'m` means `'medium`;
- `'s` means `'short`;

<br>

Constraints:
- `'l: 'm`;
- `'l: 's`;
- `'m: 's`;

<br>

## Variance over lifetime `'a`
**Variance over generic lifetime parameter** `'a`:
- type `&'a T` is **covariant** over `'a`, in other words, if `'l` **outlives** `'s`, then `&'l T` is a **subtype** of `&'s T`;
- type `&'a mut T` is also **covariant** over `'a`, in other words, if `'l` **outlives** `'s`, then `&'l mut T` is a **subtype** of `&'s mut T`;

In other words, `&'l T` can be used wherever `&'s T` is expected and `&'l mut T` can be used wherever `&'s mut T` is expected, because `'l` **lives at least as long as** `'s`. But **not vice versa**.<br>

For example, since `'static` **outlives** the lifetime parameter `'a`, `&'static str` is a **subtype** of `&'a str`. So we can assign `'static` to reference with **shorter** lifetime.<br>

<br>

Consider function:
```Rust
fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        x
    } else {
        y
    }
}
```

The signature of function promises that **returned value** has lifetime `'a`, but in fact it receives 2 arguments with **different** lifetimes `'a` and `'b` and **conditionally returns one of them**.<br>
According to **variance over lifetime** passing value of type `&'b i32` in place where type `&'a i32` is expected implies `'b: 'a`.<br>

<br>

## Variance over generic type `T`
**Variance over generic type** `T`:
- type `&'a T` is **covariant** over `T`, in other words, if `T` is a **subtype** of `U` then, then `&'a T` is a **subtype** of `&'a U`.
- type `&'a mut T` is **invariant** over `T`, in other words, if `T` is a **subtype** of `U` then, then **neither** `&'a mut T` is a _subtype_ of `&'a mut U` **nor** `&'a mut U` is a _subtype_ of `&'a mut T`.

A type `&'a T` can be used wherever `&'a U` is expected. **But** `&'a mut T` **cannot** be used wherever `&'a mut U` is expected and **vice versa**.<br>

<br>

Consider `T = &'l str` and `U = &'s str` and `T` is a **subtype** of `U`, this means that:
- `&'a &'l str` is a **subtype** of `&'a &'s str`;
- `&'a mut &'l str` **cannot** be a **subtype** of `&'a mut &'s str`, even if `&'l str` is a **subtype** of `&'s str`;

<br>

**Explanation**:
```rust
fn foo<'s,'l>(r: &'s mut &'l u32) {
    let x = 1;
    *r = &x;
}
```

If we allow `&'s mut &'l u32` to be coerced into `&'s mut &'x u32`, where `'x` is a lifetime of local var `x`, then we can get **dangling pointer**: after `foo` returns we have `r` pointing to `x` which is deallocated.<br>

<br>

# Nested references
Consider **struct** that **contains reference**:
```rust
struct S<'l, T> { 
    x: &'l T 
}
```

And consider **reference to struct**:
```rust
&'s S<'l>
```

The **lifetime** `'l` associated with the **inner member of struct** must **outlive** the **lifetime** `'s` associated with an **outer reference to struct**: `'l: 's`.<br>

When you have a type with a **nested reference** such as `&'s &'l U`, compilers infers `'l: 's`.<br>
In other words, compiler consider that the **outer** reference has **shorter lifetime** `'s` and the **inner** reference has **longer lifetime** `'l`: `&'s &'l U`.<br>

<br>

## Nested references coercions
Consider **nested references**:
- a `&'m &'l U` coerces to a `&'s &'s U`;
- a `&'m mut &'l mut U` coerces to a `&'s mut &'l mut U`, but **not** to a `&'s mut &'s mut U`;

This is because `&mut T` is **invariant** in `T`, which means **any lifetimes** in `T` **cannot** change (**grow** or **shrink**) at all.<br>
In the example, `T = &'l mut U`, and the `'l` **cannot be changed**.<br>

<br>

### Example 1
Consider code:
```rust
fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}");
}
```

**Explanation**:
- `&mut hello` becomes `&mut &'static str` and `&world` becomes `&'world str`;
- the statement `*input = val;` means `*(&mut &'static str) = &'world str;`;
- after dereferencing `*input = val;` is transformed to `&'static str = &'world str;`;

So, we attempt to assign value of type `&'world str` to value of type `&'static str`, but this **violates constraint** `'static: 'world`, because `'static` is the **longest lifetime**.<br>

<br>

**This works**:
```rust
fn main() {
    let mut hello: &'static str = "hello";
    {
        let world: &'static str = &"world";
        assign(&mut hello, &world);
    }
    println!("{hello}");
}
```

<br>

### Example 2
Consider code:
```rust
fn bar(v: &mut Vec<&'static str>) {
    let w: &mut Vec<&'_ str> = v; // call the lifetime 'w
    let local = String::from("world");
    w.push(&local);
}
```

If `'w` was allowed to be **shorter** than `'static`, we'd end up with a **dangling reference** in `*v` after `bar` returns.<br>

<br>

### Example 3
```rust
#[derive(Debug)]
struct Movie<'a> {
    title: &'a str,
    rating: u8,
}

#[derive(Debug)]
struct Reviewer<'a, 'b: 'a> {
    movie: &'a Movie<'b>,
    name: &'a str,
}

impl<'a, 'b> Reviewer<'a, 'b> {
    fn new(name: &'a str, movie: &'b Movie) -> Self {
        Reviewer { movie: movie, name: name }
    }
}

fn main() {
    let movie = Movie {
        title: "Foo",
        rating: 10,
    };

    println!("{:?}", Reviewer::new("Bar", &movie));
}
```

Here `'b` specifies that lifetimes of the `Movie` struct **outlives** the `Reviewer` struct.

<br>

## Dereferencing nested references
You can get a `&'l U` from a `&'s &'l U`. Just copy it out!<br>
But you **cannot** get a `&'l mut U` through dereferencing a `&'s &'l mut U`. You can only **reborrow** a `&'s mut U`.<br>

Recall that once a shared reference exist, any number of copies of it could simultaneously exist. Therefore, so long as the **outer** shared reference exists, the **inner** `&mut` must **not be usable**. And once the **outer** reference **expires**, the inner `&mut` is active and **must again be exclusive**, so it **must not** be possible to obtain a `&'long U` either.<br>

That's why code below **doesn't** compile:
```rust
fn deref0<'long, 'short>(v: &'short &'long mut u32) -> &'long u32 { 
    *v
}
```

**Error**:
```
lifetime may not live long enough
consider adding the following bound: 'short: 'long
```

<br>

**But, this works**:
```rust
fn deref1<'long, 'short>(v: &'short &'long mut u32) -> &'short u32 { 
    *v
}
```

And this works:
```rust
fn deref2<'long, 'short>(v: &'short &'long u32) -> &'long u32 { 
    *v
}
```

<br>

**Explanation**:
- consider we can deref `&'short &'long mut u32` to `&'long u32`;
- this means that caller can get **immutable ref** `&'long u32` **with the same lifetime as** `&'long mut u32`;

In **example** below `a_mut` is used when value is borrowed as `&'long u32` in `a_ref`:
```rust
fn reborrow_unsound<'short, 'long, T>(r: &'short &'long mut T) -> &'long T {
    unsafe { &*(*r as *const T) }
}

fn main() {
    let mut a = String::from("hello");
    let a_mut = &mut a;
    let a_ref = reborrow_unsound(&a_mut);
    *a_mut = String::from("world");
    println!("{a_ref}");
}
```

<br>

# Region inference
**NLL-aware borrow checker** uses **region inference algorithm** which solves the **set of constraints** created by compiler.<br>

The **region inference algorithm** uses **fixed-point approach**:
- each **region variable** begins as an **empty set**;
- repeatedly growing the **region variables** until they are big enough to satisfy all constraints;

<br>

The **set of constraints** consists of **3** kinds of constraints:
- **liveness constraints**, which arise from **liveness**;
- **outlives constraints**, which arise from **subtyping rules**;
- **member constraints**, which arise from `impl Trait`;

<br>

[**More details about region inference is here**](https://rustc-dev-guide.rust-lang.org/borrow_check/region_inference.html).<br>

<br>

## Liveness constraints
A **liveness constraint** arises when some **variable whose type includes a region** `'R` is **live** at some **point** `P`.<br>

**Liveness constraints** are:
- **if some variable is live on entry to a point** `P`, then **all regions in its type must include** `P`;
  - this simply means that all **region variables corresponding to variable's regions in its type must include the point** `P`;
  - also phrase **live on entry to a point** means that points where **variable occurs in lvalue** must be **excluded** from **region variables**;
- **for each borrow expression like** `&'foo foo`, the **region** `'foo` **must include the point of borrow**;

**Liveness constraints** are **computed** by the **MIR type checker**.<br>

Example of **liveness constraints computation**:
- for each variable `v` whose **type includes a region** `'R` the compiler creates **region variable** and initializes it with **empty set**: `'R = {}`;
- the **region** `'R` would be **live** at precisely the same points where **variable** `v` is **live**;
- every time compiler sees variable `v` at point `P` it adds `P` to **region variable** `'R`: `'R = 'R Union {P}`;

<br>

As a result **region** `'R` will contain **all points** in the **MIR CFG** where this **region** is **valid**;

<br>

## Location-aware subtyping constraints
**Variance rules establishes constraints**:
- if **reference** with **region** `'source` is copied into a **variable** whose **type includes the region** `'target`, then `'source: 'target`;
  - in other words, whenever reference is copied from one location to another, e.g. `ref1 = ref2`, the lifetime of the **source** reference must **outlive** the lifetime of the **target** location;
- **reborrow** creates **new** reference with **shorter lifetime**: `'original: reborrowed`;

<br>

Consider types `&'R1 T1` and `&'R0 T0`.<br>
**Location-aware subtyping constraints** means that the **current location is taken into account**.<br>
In other words, instead of writing `T1 <: T0` we will write `(T1 <: T0) @ P` (`T1` is **required** to be a **subtype** of `T0` **at the point** `P`).<br>
This in turn implies **location-aware region constraints**: `('R0 <= 'R1) @ P` or `('R1: 'R0) @ P`.<br>

<br>

A **location-aware region constraint** `('R1: 'R0) @ P` means that, starting from the point `P`, the **region** `'R1` must include all points that are reachable **without leaving the region** `'R0`.<br>
Note, that region `'R0` may contain **holes** which must be interpreted as **points** where `'R0` is **dead**.<br>
In other words, **hole** means we **leave** (**exit**) **region** `'R0` and all points after hole are **not** reachable.<br>
So, constraint `('R1: 'R0) @ P` means that compiler must add **to** `'R1` points **from** `'R0` that are **only** in range: `[P, First_hole)`.<br>

<br>

When **reference** with **lifetime** `'ref` is **assigned** to variable whose **type includes a region** `'v`, the compiler **adds location-aware region constraint** `'ref: 'v` to **set of constraints**.<br>

<br>

Consider **MIR CFG**:
```shell
// let mut foo: i32;
// let mut bar: i32;
// let p: &'p i32;

A
[ p = &'foo foo  ]            A/0
[ if condition ] ----\ (true) A/1
       |             |
       |     B       v
       |     [ print(*p)     ] B/0
       |     [ ...           ] B/1
       |     [ p = &'bar bar ] B/2
       |     [ ...           ] B/3
       |     [ goto C        ] B/4
       |             |
       +-------------/
       |
C      v
[ print(*p)    ] C/0
[ return       ] C/1
```

For this CFG we have:
- **variable** `p` and its type has one **region** `'p`;
- **borrow expression** `&'foo foo` with **region** `'foo`;
- **borrow expression** `&'bar bar` with **region** `'bar`;

<br>

Taking **liveness constraints** into account we obtain **region variables**:
- the **variable** `p` is **live on entry to** `A/1`, `B/0`, `B/3`, `B/4`, and `C/0`;
  - `'p = {A/1, B/0, B/3, B/4, C/0}`
- **points of borrow**:
  - `'foo = {A/0}`;
  - `'bar = {B/2}`;

<br>

Taking **region constraints** into account we obtain:
- at point `A/0`, we have `p = &'foo foo`:
  - the type of `&'foo foo` is `&'foo i32`;
  - the type of `p` is `&'p i32`;
  - so:
    - **location-aware subtyping constraint**: `(&'foo i32 <: &'p i32) @ A/1`;
    - which in turn implies **region constraint**: `('foo: 'p) @ A/1`;
- at point `B/2`, we have `p = &'bar bar`:
  - the type of `&'bar bar` is `&'bar i32`;
  - the type of `p` is `&'p i32`;
  - so:
    - **location-aware subtyping constraint**: `(&'bar i32 <: &'p i32) @ B/3`;
    - which in turn implies **region constraint**: `('bar: 'p) @ B/3`;

<br>

Finally, the **full set of constraints** is:
- `('foo: 0) @ A/1`
- `('bar: 'p) @ B/3`
- `'p = {A/1, B/0, B/3, B/4, C/0}`
- `'foo = {A/0}`
- `'bar = {B/2}`

<br>

**Solving these constraints** results in the following lifetimes:
- `'p = {A/1, B/0, B/3, B/4, C/0}`
- `'foo = {A/0, A/1, B/0, C/0}`
- `'bar = {B/3, B/4, C/0}`

<br>

**Explanation**.<br>
The variable `foo` is **borrowed for the region** `'foo`, which does **not include** `B/3` and `B/4`.<br>
Though the `'p` includes `B/3` and `B/4`, but they **unreachable** from `B/0` because `'p` has **hole** in `B/1`.<br>
Similarly, `bar` is **borrowed for the region** `'bar`, which begins at `B/4` and extends to `C/0` (and need not include earlier points, which are not reachable).<br>

<br>

## Example on subtyping constraints
Consider example:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let s1 = String::from("longest");

  let result;
  {
    let s2 = String::from("short");
    result = longest(&s1, &s2);
  }
  println!("The longest string is '{}'.", result);
}
```

<br>

Assume that compiler will assign **region** `'s1` for borrow expression `&s1` and **region** `'s2` for borrow expression `&s2`.<br>
Because references are passed to functions it means they are assigned to variables `x` and `y` with the same **region** `'a` in their types.<br>
The **result** of function has **region** `'a` is assigned to variable `result` that has **region** `'r` in its type.<br>

So, compiler creates **set of subtyping constraint**:
- `'s1: 'a`
- `'s2: 'a`
- `'a:  'r`

<br>

This **set of subtyping constraint** means:
- whenever `'a` is **valid** the **values** `s1` and `s2` **must be considered borrowed**;
- universal region `'a` is **extended by** `'r`;

<br>

# Drop as last use
If destructor is implemented, then the last use of a variable will in its **destructor**, which will implicitly execute at the end of the enclosing scope.<br>
