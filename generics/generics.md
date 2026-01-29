# Table of contents
- [Table of contents](#table-of-contents)
- [Generics](#generics)
  - [Generic kinds](#generic-kinds)
- [Generics vs. impl Trait](#generics-vs-impl-trait)

<br>

# Generics
**Generic** is a **declaration**/**definition** of **type** (**function**/**struct**/**enum**/**trait**) that contains **type variable** (aka **type parameter**).<br>
**Type var** is often defined in **angle brackets**, e.g. `<T>` or `<E>`. There can be **multiple** *type vars* in **generic**.<br>
**Generics** can be **limited** by **traits**, i.e. we can use trait as type for **type var**.

Example of **generic function**:
```Rust
fn f <T>(param: T) {}
```

Example, **type var** `T` is **limited** by trait `Bark`:
```Rust
fn bark<T: Bark> (b: T) { ... }
```

<br>

The important part is the `<T>` **after** the function name. Without this, Rust will think that `T` is a *concrete type* (concrete = **not** generic), like `String` or `i8`.<br>
For the `bark` function, you can say: The function `bark` is **generic over type** `T`.<br>

<br>

## Generic kinds
<table>
    <tr>
        <th>Kind</th>
        <th>Example</th>
    </tr>
    <tr></tr>
    <tr>
        <td>Generic struct</td>
<td>

**Syntactic forms**:
- `S<T> where T: R` **trait bound**, it guarantees type `T` **impls** trait `R`, the `S<T: R>` is *short form*;
- `S<T,P> where T: R, P: S` **independent trait bounds**, here one for `T` and one for `P`;
- `S<const N: usize>` **generic const bound**;
  - `S<const N: u8 = 0>` **default parameter** for constants;
`S<T = R>` **default parameter**, e.g.,`S<T = u8>`;

<br>

```Rust
struct Point<T, V> {
    x: T,
    y: V
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
}
```

</td>
    </tr>
    <tr></tr>
    <tr>
        <td>Generic function</td>
<td>

```Rust
trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str
}

struct Cat {
    color: &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species)
    }
}

fn bark<T: Bark>(b: T) {
    println!("{}", b.bark())
}

fn main() {
    let dog = Dog { species: "retriever" };
    let cat = Cat { color: "black" };
    bark(dog);
    // bark(cat); → ERROR
}
```

</td>
    </tr>
    <tr></tr>
    <tr>
        <td>Generic enum</td>
<td>

```Rust
enum Colors<T> {
    Red(T),
    Blue(T)
}
```

</td>
    </tr>
    <tr></tr>
    <tr>
        <td>Generic trait</td>
<td>

**Syntactic forms**:
- `trait T<X> {}` a **trait generic** over `X`, can have **multiple** `impl T<X> for S` (one per `X`);
- `trait T { type X; }` defines **associated type**, **only one** `impl T for S` **possible**;
- `trait T { type X<G>; }` defines **generic associated type**;
- `trait T { type X<'a>; }` defines a **generic associated type** generic over a lifetime;
  - `type X = R;` set **associated type**, e.g. `impl T for S { type X = R; }`;
  - `type X<G> = R<G>;` set **associated type** for **generic associated type**, e.g., `impl T for S { type X<G> = Vec<G>; }`.
- `fn f() where Self: R;` in trait `T {}`, make `f` accessible **only** on types known to also `impl R`;
  - `fn f() where Self: Sized;`;

<br>

```Rust
#[allow(unused_variables)]
#[allow(unused_assignments)]
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());
    // let b = vec![1.0, 2.0, 3.0];
    // println!("sum float = {}", b.sum()); → ERROR, not implemented for float!
}
```

<br>

More generic version:
```Rust
trait Summable<T> {
    fn sum(&self) -> T;
}

impl<T: std::ops::Add + Copy + Default + std::ops::AddAssign> Summable<T> for Vec<T> {
    fn sum(&self) -> T {
        let mut sum: T = T::default();
        for i in self {
            sum += *i;
        }
        sum
    }
}
```

<br>

When a **trait** has a **generic parameter**, it **can** be implemented for a **some type**, (e.g. `Mytype`) **multiple times**, **changing** the **concrete types** of the **type var each time**, example:


```Rust
trait SomeTrait<T> {
    fn abc (&self) → T;
}

impl SomeTrait<i32> for Mytype {
    fn abc (&self) → i32 {
        ...
    };
}

impl SomeTrait<f64> for Mytype {
    fn abc (&self) → f64 {
        ...
    };
}
```

</td>
    </tr>
</table>

<br>

# Generics vs. impl Trait
```rust
trait MyTrait {}
impl MyTrait for u32 {}


fn foo() -> impl MyTrait
{
    100u32 // We can create specific type if we return impl MyTrait.
}

fn bar<R>() -> R
where
    R: MyTrait
{
    100u32 // ERROR here! We cannot create specific type if we return generic.
}

fn baz<T,R>(v: T) -> R
where
    T: Into<R>,
    R: MyTrait
{
    v.into()
}

fn sink (v: u32) {

}

fn main() {
    let a = foo();
    // let b: u32 = bar();
    let z = baz(100u32);
    sink(z);
}
```