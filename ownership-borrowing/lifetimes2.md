<!-- TOC -->
* [Compiler pipeline](#compiler-pipeline)
* [Lexical scope. Liveness scope](#lexical-scope-liveness-scope)
* [Lifetimes](#lifetimes)
* [Internals of NLL](#internals-of-nll)
  * [Regions](#regions)
  * [Liveness constraints](#liveness-constraints)
  * [Location-aware subtyping constraints](#location-aware-subtyping-constraints)
  * [Example on subtyping constraints](#example-on-subtyping-constraints)
* [Region inference](#region-inference)
  * [Universal regions](#universal-regions)
  * [Region variables](#region-variables)
  * [Region constraints](#region-constraints)
  * [Region inference algorithm](#region-inference-algorithm)
<!-- TOC -->

<br>

# Compiler pipeline
Rust's chain of transformations:
`AST` -> `HIR` -> `MIR` -> `LLVM IR`<br>

<br>

**Compiler pipeline**:<br>
![HIR_MIR](/img/HIR-MIR.png)

<br>

**Transformations**:
- the **HIR** is constructed from **AST**;
- the **MIR** is constructed from **HIR**;
- the **LLVM IR** is constructed from **MIR**;

<br>

The **HIR** (**high-level IR**) is used for **type inference** and **type checking**.<br>
The **MIR** (**mid-level IR**) is used for **borrow checking** and **optimizations** use. Internaly of the compiler the **MIR** is represented as **CFG**.<br>

<br>

# Lexical scope. Liveness scope
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
    let mut v;                               //---------------------------------------+-- lexical scope of viable v
    {                                        //                                       |
        v = Box::new(10);                    //--------+-- liveness scope of Box<10>  |
        println!("{}", v);                   //        |   that is bound to v         |
        drop(v);                             //        |                              |
        v = Box::new(20);                    //--------+-- liveness scope of Box<20>  |
        println!("{}", v);                   //        |   that is bound to v         |
    }                                        //--------+                              |
    {                                        //                                       |
        v = Box::new(30); println!("{}", v); //--------+-- liveness scope of Box<30>  |
    }                                        //--------+   that is bound to v         |
}                                            //---------------------------------------+
```

**In common** _lexical scope_ **is not equal** _liveness scope_, because variables can be **dropped** or **reassigned**.<br>
But **in particular** _lexical scope_ **can be equal to** _liveness scope_.<br>


<br>

# Lifetimes
A **reference type** has a **lifetime** which is a **part of its type**.<br>
The borrow must last **at most** (<=) as long as the **liveness scope** of borrowed value.<br>

Lifetimes appear in various places:
- a lifetime can be a **part** of the **reference type**: `let mut p: &'p T`;
- a lifetime can be a **part** of the **borrow expression** (aka **reference** or **borrow**): `p = &'foo foo;`;

The **lifetime** of the **reference** must be **at most as long as** the **liveness scope** of the **value** the **reference points to**.<br>
In other words, **lifetime** of the **borrow expression** cannot be **longer** than the **liveness scope** of the borrowed value (referent).<br>

<br>

# Internals of NLL
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

The key point is that variable `p` becomes **dead** in the span **before** it is **reassigned**.<br>
So how does **liveness** relate to **NLL**? The key rule is this: **whenever a variable is live, all references that it may contain are live**.<br>

A new **NLL**-based borrow-checker processes **MIR** rather than the **AST**. The code above in **MIR** will look like:
```shell
// let mut foo: i32;
// let mut bar: i32;
// let p: &i32;

A
[ p = &foo     ]
[ if condition ] ----\ (true)
       |             |
       |     B       v
       |     [ print(*p)     ]
       |     [ ...           ]
       |     [ p = &bar      ]
       |     [ ...           ]
       |     [ goto C        ]
       |             |
       +-------------/
       |
C      v
[ print(*p)    ]
[ return       ]
```

The `A`, `B` and `C` are **basic blocks**.<br>

<br>

## Regions
We can think about **lifetime of a reference** as a **region of the CFG** or just **region**.<br>
Each **region** can be represented as a **set of points** in the **CFG**.<br>
Each **point** in the **CFG** can be represented by **block**/**index**.<br>
Note that in _MIR_ **each basic block** will also have **point** for the **terminator** (**goto**/**return** statements).<br>

<br>

For example, in the _MIR_ above:
- `A/0` refers to `p = &foo`;
- `B/2` refers to `p = &bar`;
- `A/1`, `B/4`, and `C/1` refer to **terminators** in `A`, `B` and `C` respectively;

<br>

The term **region** is often used in place of **lifetime**.<br>
For each **reference type** compiler create a **region** to represent the **lifetime as part of the type** of such variable.<br>
For each **borrow expression** like `&foo` compiler create a **region** to represent the **lifetime of the borrow**.<br>

**CFG with regions** looks like:
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

<br>

## Liveness constraints
A liveness constraint arises when some variable whose type includes a region R is live at some point P. This simply means that the value of R must include the point P. Liveness constraints are computed by the MIR type checker.

**Rules**:
- **if a variable is live on entry to a point** `P`, **then all regions in its type must include** `P`.<br>
- **for each borrow expression like** `&'foo foo`, the **region** `'foo` **must include the point of borrow**.<br>

<br>

For the CFG above we have:
- **variable** `p` and its type has one **region** `'p`;
- **borrow expression** `&'foo foo` with **region** `'foo`;
- **borrow expression** `&'bar bar` with **region** `'bar`;

Taking these 2 above rules into account we obtain:
- the **variable** `p` is **live on entry to** `A/1`, `B/0`, `B/3`, `B/4`, and `C/0`;
  - `'p = {A/1, B/0, B/3, B/4, C/0}`
- **points of borrow**:
  - `'foo = {A/0}`
  - `'bar = {B/2}`

<br>

## Location-aware subtyping constraints
**Subtyping** is the idea that one type `Sub` can be used in place of another `Super`. Notation: `Sub <: Super`.<br>

<br>

**Location-aware subtyping constraints** means that the **current location is taken into account**.<br>
In other words, instead of writing `T1 <: T2` (`T1` is **required** to be a **subtype** of `T2`) we will write `(T1 <: T2) @ P` (`T1` is **required** to be a **subtype** of `T2` **at the point** `P`).<br>
This in turn will translate to **region constraints** like `(R2 <= R1) @ P`.<br>

A **region constraint** like `('R1: 'R2) @ P` means that, starting from the point `P`, the **region** `'R1` must include all points that are reachable without leaving the region `'R2`.<br>
The **search stops** if we **exit the region** `'R2`, e.g. **points where variable that has** `'R2` **in its types is dead**. Otherwise, for each point we find, we add it to `'R1`.<br>

<br>

**Rules**:
- the **borrow** must last **at most as long as the liveness scope** of **borrowed value**, in other words, the **reference cannot live longer than referent**;
- whenever references are copied from one location to another, e.g. `ref1 = ref2`, the lifetime of the **source** reference must **outlive** the lifetime of the **target** location;
  - in other words, if reference with **region** `'source` is copied into a **variable** whose **type includes the region** `'target`, then `'source: 'target`;
- **reborrow** creates new reference with shorter lifetime: `'original: reborrowed`

<br>

For our CFG above at point `A/0`, we have `p = &'foo foo`, so:
- the type of `&'foo foo` is `&'foo i32`;
- the type of `p` is `&'R2 i32`;
- so:
  - **location-aware subtyping constraint**: `(&'foo i32 <: &'p i32) @ A/1`;
  - which in turn implies **region constraint**: `('foo: 'p) @ A/1`;

<br>

A **region constraint** `('foo: 'p) @ A/1` means that, starting from the point `A/1`, the region `'foo` must include all points that are reachable **without leaving the region** `'p`.
The **search stops** if we **exit the region** `'p`, e.g. **points where variable that has** `'p` **in its types is dead**. Otherwise, for each point we find, we add it to `'foo`.<br>

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
Though the `'p` includes `B/3` and `B/4`, but you **cannot reach** `B/3` and `B/4` **from** `A/1` **without going through** `B/1`, and `'p` **does not include** `B/1` (because `p` is **dead** at `B/1`).<br>
Similarly, `bar` is **borrowed for the region** `'bar`, which begins at B/4 and extends to C/0 (and need not include earlier points, which are not reachable).<br>

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
Because references are passed to functions it means they are assigned to variables `x` and `y` with **region** `'a` in their types.<br>
The **result** of function has **region** `'a` is assigned to variable `result` that has **region** `'r` in its type.<br>
So, compiler creates **set of subtyping constraint**:
- `'s1: 'a`
- `'s2: 'a`
- `'a:  'r`

<br>

This **set of subtyping constraint** means:
- whenever `'a` is **valid** the **values** `s1` and `s2` **must be considered borrowed**;
- region `'a` is **extended by** `'r`;

<br>

# Region inference
[**Region inference**](https://rustc-dev-guide.rust-lang.org/borrow_check/region_inference.html).<br>

<br>

## Universal regions
Consider code:
```rust
fn foo<'a>(x: &'a u32) {
// ...
}
```
here `'a` is a **universal region** (aka **named lifetime**).

<br>

## Region variables
The kinds of **region elements** are as follows:
- any **location** in the **MIR CFG**;
    - a **location** is just the **pair** of a **basic block** and an **index**;
- **element** `end('a)`
  - each **universal region** `'a` contains element `end('a)`;
  - if the value for some **region** `R0` includes `end('a)`, then this implies that `R0` must **extend** until the **end** of `'a` in the **caller**;

<br>

## Region constraints
There are **3** kind of constraints that are used in **NLL**:
- **liveness constraints**, which arise from **liveness**:
  - for each variable `p` whose **type includes a region** `'R` the compiler creates **region variable** and assigns to it **empty set**: `'p = {}`;
  - the **region** `'p` would be **live** at precisely the same points where **variable** `p` is **live**;
  - every time compiler sees variable `p` at point `P` it adds `P` to set: `'p = 'p Union {P}`;
  - as a result **region** `'p` will contain **all points** in the **MIR CFG** where this **region** is **valid**;
- **outlives constraints**, which arise from **subtyping rules**:
  - when **reference** with **lifetime** `'ref` is **assigned** to variable whose **type includes a region** `'v`, the compiler **adds outlives constraints** `'ref: 'v` to **set of constraints**.<br>
- **member constraints**, which arise from `impl Trait`;

<br>

## Region inference algorithm
Once the **set of constraints** is created, the **region inference algorithm** solves it. This is done via **fixed-point iteration**:
- each **region** begins as an **empty set**
- repeatedly growing the regions until they are big enough to satisfy all constraints;

<br>
