1. Liveness constraints
2. Outlives constraints

The **HIR** (**High-Level Intermediate Representation**) is the **primary IR** used in most of rustc.
The **MIR** (**Mid-level Intermediate Representation**) is constructed **from** HIR and it is based on **CFG**.
**MIR** is used for certain safety checks by the borrow checker and also for optimization and LLVM IR generation.

# 
When we talk about borrow, there are following different kinds of “lifetime” involved:
- the **lexical scope** of identifier;
- the **liveness scope** of borrowed value;
- **duration** of the whole **borrow**;

**Lexical scope** of variable is the region of code where the **variable** is **valid**. 
_Lexical scope_ **starts** from the point where **variable** is **declared** by `let` keyword and **ends** at end of scope `}`.
**Liveness scope** of value is the region of code where the **value** is **valid**.
_Liveness scope_ **starts** from the point where **value** is **created** and **ends** where **value** is **dropped**.

A reference has a lifetime which is a part of the type of reference.
The borrow must last **at most** (<=) as long as the **liveness scope** of borrowed value.


<br>


**Lifetimes** as a **set of points** in the **CFG**.

# 
Lifetimes appear in various places:
- a lifetime can be a **part** of the **reference type**: `let mut p: &'p T`;
- a lifetime can be a **part** of the **borrow expression**: `p = &'foo foo;`;

**Example**:
```rust
let mut foo: T = ...;
let mut bar: T = ...;
let mut p: &'p T;
//      --
p = &'foo foo;
//   ----
if condition {
print(*p);
p = &'bar bar;
//   ----
}
print(*p);
```

As you can see, the lifetime `'p` is **part** of the **type** of the variable `p`.
It indicates the portions of the CFG where `p` can safely be dereferenced.
The lifetime `'foo` is **part** of the **borrow expression** `&foo`.
The lifetime `'bar` is **part** of the **borrow expression** `&bar`.
Both lifetimes `'foo` and `'bar` are **different**: they refer to the lifetimes for which values in vars `foo` and `bar` are **borrowed**, respectively.

Lifetime in borrow expression means **duration of borrow**.<br>



# 
```rust
let ref1 = &s1;
let r;
{
    let ref2 = &s2;
    r = longest(ref1,fer2);
}
*r;
```


The signature means that the referents of `ref1` and `ref2` must remain borrowed 

The value s1 and s2 must remain borrowed as long as `r` is in use.


Lets say borrow &s1 has lifetime 's1 and borrow &s2 has lifetime 's2. And the type of r has a lifetime 'r.

Assigning longest::<'a> to r  imposes constraints:


's1: 'a  // coercion to argument

's2: 'a  // coercion to argument

'a:  'r  // assignment from `longest(..)` to `res`


That means, whenever 'a is valid, 's1 and 's2 have to be valid too, and wherever 'r is valid, 'a must be too. Or more practically phrased: so long as r is still in use, S1 and S2 must remain borrowed.


The borrow checker doesn't choose 's1 and 's2 ahead of time and then pick one when calling longest.

Instead, it figures out all the lifetimes based on what gets used where, and by applying the constraints and making sure they can be satisfied.






# Regions
The value of a **region** (aka **lifetime**) can be thought of as a **set**.<br>

Consider code:
```rust
fn foo<'a>(x: &'a u32) {
// ...
}
```
here `'a` is a **universal region** (aka **named lifetime**).

The kinds of **region elements** are as follows:
- any **location** in the **MIR CFG**;
    - a **location** is just the **pair** of a **basic block** and an **index**;
- **element** `end('a)` corresponding to each **universal region** `'a`;
    - if the value for some **region** `R0` includes `end('a)`, then this implies that `R0` must **extend** until the **end** of `'a` in the **caller**;

<br>

# Constraint propagation
There are **3** kind of constraints that are used in **NLL**:
- **liveness constraints**, each region needs to be live at points where it can be used.;
  - if we have a _liveness constraint_ `R live at E`, then we can apply `Values(R) = Values(R) union {E}`
- **outlives constraints**, which arise from **subtyping**;
  - if we have an _outlives constraints_ `R1: R2`, we can apply `Values(R1) = Values(R1) union Values(R2)`
- **member constraints**, which arise from `impl Trait`;



So how do we compute the contents of a region? This process is called **region inference**.
Once the constraints are created, the inference algorithm solves the constraints.
This is done via **fixed-point iteration**:
- each **lifetime variable** (**region**) begins as an **empty set**
- and we iterate over the constraints, repeatedly growing the lifetimes until they are big enough to satisfy all constraints;

A **liveness constraint** `R live at E` is **satisfied** if when some **variable** whose **type** includes a **region** (**lifetime parameter**) `R` is **live** at some point `E`.
This simply means that the value of `R` must include the point `P`. To "apply" such a constraint to `Values`, we just have to compute `Values(R) = Values(R) union {E}`.

An **outlives constraint** `R1: R2` is satisfied if `Values(R1)` is a **superset** of `Values(R2)`. To "apply" such a constraint to `Values`, we just have to compute `Values(R1) = Values(R1) union Values(R2)`.

1. From the **liveness constraints** we can fill each region with appropriate locations, then **region** will contain **all points** in the **MIR CFG** where the **region** is **valid**.
2. From the **outlives constraints** we enrich each region, for example, for each region `'a`, if `'a: 'b`, then we add all elements of `'b` to `'a`, including `end('b)`.

A lifetime `L` is **live** at a point `P` if there is some variable `p` which is **live** at `P`, and `L` is the **part** of **type** of `p`.

Specifically, if a **lifetime** `L` is live at the **point** `P`, then we will introduce a **constraint** like: `(L: {P}) @ P`.

Consider variable `let mut p: &'p T`, its **lifetime** `'p` would be **live** at precisely the same points where **variable** `p` is **live**.

# Liveness
We say that a variable is live if the current value that it holds may be used later.

This is very important to Example 4:
let mut foo: T = ...;
let mut bar: T = ...;
let mut p: &'p T = &foo;
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

Here you see a variable p that is assigned in the beginning of the program, and then maybe re-assigned during the if.
The key point is that p becomes dead (not live) in the span before it is reassigned.
This is true even though the variable p will be used again, because the value that is in p will not be used.



The lifetimes `'foo` and `'bar` have no points where they are (directly) live, since they do not appear in the types of any variables.


# Subtyping
Whenever references are copied from one location to another, the Rust subtyping rules require that the lifetime of the source reference outlives the lifetime of the target location.

As discussed earlier, in this RFC, we extend the notion of subtyping to be location-aware, meaning that we take into account the point where the value is being copied.


For example, at the point A/0, our running example contains a borrow expression p = &'foo foo. In this case, the borrow expression will produce a reference of type &'foo T, where T is the type of foo.
This value is then assigned to p, which has the type &'p T. Therefore, we wish to require that &'foo T be a subtype of &'p T.
Moreover, this relation needs to hold at the point A/1 – the successor of the point A/0 where the assignment occurs (this is because the new value of p is first visible in A/1).
We write that subtyping constraint as follows:
(&'foo T <: &'p T) @ A/1

In the case of our running example, we generate the following subtyping constraints:
(&'foo T <: &'p T) @ A/1
(&'bar T <: &'p T) @ B/3

These can be converted into the following lifetime constraints:
('foo: 'p) @ A/1
('bar: 'p) @ B/3


# Reborrow constraints
Reborrow constraints. Consider the case where we have a borrow (shared or mutable) of some lvalue lv_b for the lifetime 'b:
lv_l = &'b lv_b      // or:
lv_l = &'b mut lv_b

In that case, we compute the supporting prefixes of lv_b, and find every deref lvalue *lv in the set where lv is a reference with lifetime 'a. We then add a constraint ('a: 'b) @ P, where P is the point following the borrow (that’s the point where the borrow takes effect).

Solving constraints
The meaning of a constraint like ('a: 'b) @ P is that, starting from the point P, the lifetime 'a must include all points in 'b that are reachable from the point P.

The implementation does a depth-first search starting from P; the search stops if we exit the lifetime 'b. Otherwise, for each point we find, we add it to 'a.

In our example, the full set of constraints is:
('foo: 'p) @ A/1
('bar: 'p) @ B/3
('p: {A/1}) @ A/1
('p: {B/0}) @ B/0
('p: {B/3}) @ B/3
('p: {B/4}) @ B/4
('p: {C/0}) @ C/0

Solving these constraints results in the following lifetimes, which are precisely the answers we expected:
'p   = {A/1, B/0, B/3, B/4, C/0}
'foo = {A/1, B/0, C/0}
'bar = {B/3, B/4, C/0}













Scopes always correspond to blocks with one exception: the scope of a temporary value is sometimes the enclosing statement.

As noted above, NLL uses a new borrow-checker that processes MIR rather than the AST.

Before we go any further, let's define some terminology. We already know the scope (or lexical scope) of a variable is the {} around where the variable is declared; it's the part of the code where that variable binding is valid. The actual value that a variable is bound to also has a scope (or liveness scope) of a slightly different nature - it's the portion of the code where that value is valid. The scope of a value starts where the value is created and ends where the value is dropped.

A reference in Rust has a lifetime, which is part of the reference's type. You can naively think of the lifetime as being as long as the liveness scope of the value the reference points to, and that will get you pretty far, but it would be better to think of the lifetime as being at most as long as the scope of the value.
In other words, a reference has a lifetime, shorter than or equal to the liveness scope of the value it refers to.

Some other Rust documentation and articles calls the liveness scope the "lifetime of the value". Why do we use the term "scope" instead?
but a reference is itself a kind of value with it's own liveness scope, and a reference is bound to a variable with a lexical scope (the liveness scope and lexical scope of a reference are the same, since references are Copy). If we used lifetime to talk about liveness scopes, then references would have two different lifetimes, and this chapter would get confusing very quickly.

fn main() {
let mut var;
{var=Box::new(10); println!("{}", var); drop(var); var=Box::new(20)}
{var=Box::new(30); println!("{}", var); drop(var)}
}



There are actually three different scopes. A block's scope, a variable's scope, and a reference's referent's scope.

A variable's scope is basically its liveness - where it is alive. It is usually from its declaration until the end of the block (or the function) it is declared in. However, a variable may be moved or dropped and then its life will end before or after the block.



what value does the compiler assign the lifetime parameter?
The final value of a lifetime parameter is a scope.
the lifetime parameter can have a different value for each instance of a Warrior.
The value assigned to a lifetime parameter is the value of the scope in which the Warrior instance is created.

The lifetime of a borrowed reference is constrained by the variable to which it is bound. If that variable goes out of scope then the borrow has certainly expired (and may even have expired before this). The key, however, is that assigning a borrowed reference to another variable can af f ect its lifetime.
The following example, which compiles without problem, illustrates:

fn f() -> i32 {
let x = 0;
let y;
{ let z = &x;
y = z;
}
return *y;
}







