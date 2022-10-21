# Generics
**Generic** is a **declaration**/**definition** of **type** (**function**/**struct**/**enum**/**trait**) that contains **type variable** (aka **type parameter**).<br>
**Type var** is often defined in **angle brackets**, e.g. ``<T>`` or ``<E>``. There can be **multiple** *type vars* in **generic**.<br>
**Generics** can be **limited** by **traits**, i.e. we can use trait as type for **type var**.

Example of **generic function**:
```Rust
fn f <T>(param: T) {}
```

Example, **type var** ``T`` is **limited** by trait ``Bark``:
```Rust
fn bark<T: Bark> (b: T) { ... }
```

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

When a **trait** has a **generic parameter**, it **can** be implemented for a **some type**, e.g., ``Mytype`` **multiple** **times**, **changing** the **concrete** **types** of the **type var** **each time**, example:


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