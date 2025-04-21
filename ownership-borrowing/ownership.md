# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Terms](#terms)
* [Stack vs. Heap](#stack-vs-heap)
  * [Stack](#stack)
  * [Heap](#heap)
* [Approaches to memory management](#approaches-to-memory-management)
* [Bitwise copy vs. Semantic copy](#bitwise-copy-vs-semantic-copy)
  * [Blittable types](#blittable-types)
  * [Non-blittable types](#non-blittable-types)
* [Ownership concept](#ownership-concept)
  * [Variable scope](#variable-scope)
  * [Ownership rules](#ownership-rules)
  * [Move vs. Copy vs. Clone](#move-vs-copy-vs-clone)
  * [Examples of copy and move semantics](#examples-of-copy-and-move-semantics)
    * [Move semantics in assignment](#move-semantics-in-assignment)
    * [Copy semantics in assignment](#copy-semantics-in-assignment)
    * [Semantics when passing values to function](#semantics-when-passing-values-to-function)
    * [Semantics when returning values from function:](#semantics-when-returning-values-from-function)
* [Primitive and complex types](#primitive-and-complex-types)
  * [Atomic primitive types](#atomic-primitive-types)
  * [Composite primitive types](#composite-primitive-types)
  * [Complex types](#complex-types)
* [Traits](#traits)
  * [Clone trait](#clone-trait)
  * [Copy trait](#copy-trait)
  * [Deriving Copy and Clone traits](#deriving-copy-and-clone-traits)
  * [Why does Copy require Clone?](#why-does-copy-require-clone)
  * [Drop trait](#drop-trait)
  * [Explicit drop](#explicit-drop)
    * [Double free problem](#double-free-problem)
    * [Drop flags](#drop-flags)
* [Example: `Vec`](#example-vec)
<!-- TOC -->

<br>

# Terms
|Term|Meaning|
|:---|:------|
|**Type**|Is a *set of* 1) **allowed values**; 2) **memory layout** for values; 3) **allowed operations** for values.|
|**Owner**|**Identifier** (or **variable**) that is owning value.|
|**Bind**|Associate identifier with a value.|
|**Copy**|Associate identifier with **bitwise copied** value and **keep the original identifier valid**.|
|**Move**|Associate identifier with **bitwise copied** value and **invalidate the original identifier**.|
|**Clone**|Associate identifier with **completely independed duplicate** of value and **keep the original identifier valid**.|
|**Mutate**|Change the value associated with a mutable identifier. By default, Rust data is immutable - it can't be changed.|
|**Borrowing**|The **action of creating a reference**. Reference is a named pointer to some value.|

<br>

# Stack vs. Heap
There are two sections of a program’s memory space:
1. **Stack section** (aka **stack memory**): for values that have **fixed** sizes **known** *at compile time*;
2. **Heap section** (aka **heap memory**): for values that have **dynamic** sizes determined *at runtime*.

*Stack memory* and *heap memory* **differ** in several ways.<br>

<br>

## Stack
1. **Easy** to manage. *Stack* is a **continuous** area of memory that **follows function calls**:
   - when a **function is called**, a new stack memory is *allocated* **automatically** to store *all* **arguments** that were passed to function and *all* its **local variabels**;
   - when the **function is finished**, the stack memory is *deallocated* **automatically**, and the program returns to the previous point of execution;
2. **Avoids** memory leaks due to **automatic** allocating and deallocating, **automatic** means it is managed **by compiler**;
2. **Faster** access than *heap*.
   - *stack* is **arranged** in **LIFO** data structure, meaning that the **most recently added** item is the **first to be removed**;
   - *stack* has **great memory locality** due to its architecture;
3. Used for **small** and **simple** data structures.

<br>

## Heap
1. More **challenging** to manage: some book-keeping needed.
2. Slightly **slower** than the *stack*.
3. The data in heap is **not** arranged in any specific structure, unlike *stack*. 
4. The data in heap can be reached in **any order**.
5. **No guarantee** of memory locality.
6. Used for **large**, **complex** data structures.

<br>

# Approaches to memory management
Traditionally, languages have fallen into two broad categories:
1. **Full control** via **manual** *memory management*:
   - programmer decides when to allocate or free heap memory;
   - programmer must determine whether a pointer still points to valid memory;
   - examples: C, C++;
2. **Full safety** via **automatic** *memory management* **at runtime**:
   - all such languages need **garbage collector**;
   - examples: Java, Python, Go, Haskell;

<br>

Rust offers a new mix:<br>
**Full control** and **full safety** via **compile time enforcement** of correct *memory management*. It does this with an **explicit ownership concept**.<br>

<br>

# Bitwise copy vs. Semantic copy
**Completely independent copy** of object is such copy that can be safely used separately to the origin one.<br>

Types of copying:
- **Bitwise copy** (aka **shallow copy**) is **type-independed logic** to duplicate values, in other words, **bitwise copy** copies contiguous block of memory bit-by-bit (byte-by-byte) to another location. For example, syscall `memcpy()` implements **bitwise copy**.
- **Semantic copy** (aka **deep copy**) requires **custom logic** to duplicate values safely.

**bitblt**/**bit blit**/**blit** are contractions for **bit block transfer**.

<br>

## Blittable types
Objects of **blittable types** occupy exactly **one** *contiguous block* of memory and **don't** have *references to itself* (e.g. one field of struct has reference to another value in the same struct).<br>
*Bitwise copy* **can** create *completely independent copy* of object of *blittable type*.<br>

<br>

## Non-blittable types
Objects of **non-blittable types**:
- occupy exactly **one** *contiguous block* of memory **with** *references to itself* (e.g. when one field of struct has reference to another value in the same struct);
- occupy **more** than one **not** adjacent *contiguous blocks* of memory;

*Bitwise copy* **cannot** create *completely independent copy* of object of *non-blittable type*.<br>
To create *completely independent copy* of object of non-blittable type **deep copy** **must** be used.<br>

<br>

# Ownership concept
## Variable scope
**Variable** or **variable’s identifier** or **identifier** are the synonyms.<br>
**Scope of variable** (or **variable scope**, or just **scope**) is the **range** within a program for which a variable is **valid**.<br>
_Scope_ is always **lexical**, because it has **explicit boundaries**: _scope_ **starts** from the point at which **variable** was **declared** by `let` keyword **until** the **end of scope**: closing curly bracket `}`.

```Rust
{ // Scope has started
    // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point
    // ...
} // Scope is over, and s is no longer valid
```

<br>

## Ownership rules
**Ownership** enables Rust to make **memory safety guarantees** without needing GC and in many cases, get the performance of C.<br>

Ownership rules:
1. **Each value** in Rust **has** an **identifier** that’s called its **owner**.
2. There can only be **one owner at a time**.
3. When the **owner** goes **out of scope**, the **value** will be **dropped**, (**scope based resource management**).

In C++ this **pattern of deallocating resources at the end of variable lifetime** is called **RAII**. Doing this correctly without GC is a difficult problem.

<br>

## Move vs. Copy vs. Clone
Both **move** and **copy** operations do the **exact same thing**, they **both** *implicitly* perform **bitwise copy** (**shallow copy**), but they are **differ**:
- *move* **transfer ownership** and **invalidates** the **original** identifier;
- *copy* **doesn’t transfer ownership** and **keeps** the **original** identifier **valid**;

The **clone** operation is similar *copy* but it performs **deep copy**.<br>

<br>

|Operation|Ownership|Original|Under the hood|
|:--------|:--------|:-------|:-------------|
|**Copy**|**Doesn’t** *transfer ownership* to another identifier after operation.|**Original** identifier **remains valid** after operation and **can be used further**.|**Bitwise copy**|
|**Move**|**Transfers ownership** to another identifier after operation.|**Original** identifier **becomes invalidated** after operation, i.e., it **no longer valid**  and **cannot be used further**.|**Bitwise copy**|
|**Clone**|**Doesn’t** *transfer ownership* to another identifier after operation.|**Original** identifier **remains valid** after operation and **can be used further**.|**Deep copy**|

<br>

Relationships between **Move**/**Copy**/**Clone**:
1.	**Move types** are types that **don't** *implement* the **Copy trait**. 
    - If type is the **Move type** it *may* implement or *may not* implement **Clone trait**.
2.	**Copy types** are types that *implement* the **Copy trait**. 
    - **Clone trait** is a supertrait of **Copy trait**, if type is the **Copy type** it also must implement **Clone trait**.
3.	**Clone types** are types that implement the **Clone trait**.

Operations such as **assignment**/**passing** a value to function and **returning** a value from function have **semantics**.<br>
**Semantics of operation** is determined by **semantics of type**:
- *Copy types* have **copy semantics**, i.e. Rust **copies** values if they are of *Copy types*;
- *Move types* have **move semantics**, i.e. Rust **moves** values if they are of *Move types*;

<br>

For example, consider a `u32` type. The value of `u32` is stored entirely in the **stack** and **don't** have any pointers to anywhere.<br>
So, **bitwise copying** of value of `u32` creates *completely independent copy*.<br>
Such types are called **Copy types**. They implement the `Copy` marker trait.<br>

<br>

Both **move** and **copy** implies **shallow copy**, but **clone** implies **deep copy**:
- consider 2 variables of a `Vec` type: `v` and `v1`:
  - when the variable `v` is **assigned** to `v1` the variable `v` is **moved** to `v1`, it means the **static part** of `Vec` on the **stack** is **bitwise copied** and the **original** variable `v` is **invalidated**; 
  - when the variable `v` is **cloned** to `v1`:
    - the **static part** of `Vec` on the **stack** is **bitwise copied**; 
    - the **new buffer** is **allocated** and all values from old buffer are **copied** to new;
    - the variables `v` and `v1` are *completely independent*;
- consider 2 variables of a `u32` type: `v` and `v1`:
  - when the variable `v` is **assigned** to `v1` the variable `v` is **copied** to `v1`, it means that value of `v` is **bitwise copied** to `v1` and they both can be used **independently**;


<br>

## Examples of copy and move semantics
### Move semantics in assignment
```Rust
let v: Vec<i32> = Vec::new();
let v1 = v;
println!("v's length is {}", v.len());
//error: borrow of moved value: `v`
```

<br>

### Copy semantics in assignment
```Rust
let v: i32 = 42;
let v1 = v;
println!("v is {}", v);// Ok!
```

<br>

### Semantics when passing values to function
```Rust
fn main() {
    let s = String::from("hello");  	// s comes into scope
    takes_ownership(s);             	// s's value moves into the function
						// and so s is no longer valid further

    let x = 5;                      	// x comes into scope
    makes_copy(x);                  	// i32 is Copy, so it's okay to still use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // variable some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and drop is called. The memory is freed.

fn makes_copy(some_integer: i32) { // variable some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

<br>

### Semantics when returning values from function:
```Rust
fn returns_ownership() -> String { 	// gives_ownership will move its return value
						// into the function that calls it
    let some_string = String::from("hello");	// some_string comes into scope

    some_string    					// some_string is returned and moves out
							// to the calling function
							// The memory will not be freed.
}
```

<br>

# Primitive and complex types
All **primitive types** implement **Copy trait** and have **Copy semantics**.<br>
All **complex types** have **Move semantics**.<br>

<br>

All **primitive types** are stored entirely on the **stack** because their sizes are **known** *at compile time* (**fixed size**).<br>
All **complex types**, like `Vec`, are stored in the **heap** because their sizes **aren't known** *at compile time*. **Complex types** usually consist of **2 parts**:
- **static part** (aka **control part**) is stored entirely on the **stack** and is used to manage **dynamic part**;
- **dynamic part** is stored in the **heap** and is **dynamically changed** *at run time*;

<br>

Examples:
- `&str` **type** (**string literal**), e.g., `let s: &str = "ABC"`, has **known size** *at compile time*. So the text is **hardcoded directly into the executable**.
- `String` **type**, e.g., `String::from("ABC")`, has **unknown size** *at compile time*. It is growable piece of text. Memory for `String` is **requested** and **dynamically changed** *at run time*.

<br>

## Atomic primitive types
**Atomic primitive types** are defined by the compiler and cannot be customized by the user.

|**Type**|**Description**|
|:-------|:--------------|
|**Bool**|The `bool` type has two values: `true` and `false`.|
|**Machine-independent integer**|**Signed** integers:`i8`, `i16`, `i32`, `i64`.<br>**Unsigned** integers: `u8`, `u16`, `u32`, `u64`.|
|**Machine-dependent integer**|**Signed** integers: `isize`.<br>**Unsigned** integers: `usize`.|
|**Floating point**|32-bit floating point: `f32`. <br>64-bit floating point: `f64`|
|Textual types|`char`: contains a Unicode scalar value in 4 bytes.<br>`&str`: **string literal**, contains a Unicode string (a sequence of Unicode scalar values).|

<br>

## Composite primitive types
|**Type**|**Description**|
|:-------|:--------------|
|**Arrays** and **slices**|Array `[T; N]` contains `N` elements of the type `T`.|
|**Tuples**|The tuple type `(T1, T2, ..., Tn)` contains a sequence of elements where each element may be of a different type.|

**Composite primitive types** automatically implement the **Copy trait** if all their constituent types implement the **Copy trait**.

<br>

## Complex types
- `Vec<T>`
- `String`
- `Map<T>`
- `Set<T>`

<br>

# Traits
## Clone trait
**Deep copy** in Rust **is always explicit action**: `x.clone()`. Rust **will never automatically** create deep copies of your data.<br>
The [**Clone trait**](https://doc.rust-lang.org/std/clone/trait.Clone.html) is used to implement **deep copy** in `clone()` method.<br>

**Deep copy** creates *completely independent copy* of objects.<br>

<br>

## Copy trait
The [**Copy trait**](https://doc.rust-lang.org/std/marker/trait.Copy.html) is **marker trait**.<br>
**Copy trait** is implemented in Rust language on:
- the **atomic primitive types**;
- the **composite primitive types** if all their constituent types implement the **Copy trait**;
- **shared references**;

<br>

Example of implementing **Copy trait** manually:
```Rust
struct MyStruct;
impl Copy for MyStruct {}
``` 

<br>

`Clone::clone` implementation for **Copy type** should just be a `memcpy()` and it is enough to return `*self`, example:
```Rust
struct MyStruct;
impl Copy for MyStruct {}
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
```

<br>

## Deriving Copy and Clone traits
**Copy trait** and **Clone trait** are **derivable traits**. It means their implementations can be derived automatically by **derive attribute**.<br>

Example:
```Rust
#[derive(Debug, Copy, Clone)]
struct Foo;
```

<br>

## Why does Copy require Clone?
**Clone trait** is a **supertrait** of **Copy trait**, if type is the **Copy type** it also must implement **Clone trait**.<br>
But *copy semantics* **will never call** `Clone::clone`. So, why does **Copy require Clone**? **Convention**?

```Rust
struct CopyableTime {
    hour: u8,
    min: u8,
    sec: u8,
}

impl Copy for CopyableTime {}

impl Clone for CopyableTime {
    fn clone(&self) -> Self {
        CopyableTime {hour: 99, min: 99, sec: 99}
    }
}

fn main() {
    let owner = CopyableTime {
        hour: 10,
        min: 10,
        sec: 10,
    };
    
    let copied = owner;
    println!("{} {} {}", copied.hour, copied.min, copied.sec);

    let cloned = owner.clone();
    println!("{} {} {}", cloned.hour, cloned.min, cloned.sec);
}
```

Output:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/playrs`
10 10 10
99 99 99
```

<br>

## Drop trait
A **resource management** in Rust is **scope based**. The [**Drop trait**](https://doc.rust-lang.org/std/ops/trait.Drop.html) is used to implement **destructors** for types.<br>
If type implements the **Drop trait** it **cannot** implement **Copy trait**, because this implies that **deallocating** values of this type is **more complex** than simply moving a stack pointer.<br>
So, if type **implements** the **Drop trait** it implies that this type is also **Move type**.<br>

<br>

## Explicit drop
Note that `std::mem::drop` is **not** the same as `std::ops::Drop::drop`.<br>
The `std::mem::drop` is just an **empty function** that takes **any value**: 
```rust
pub fn drop<T>(_x: T) {}
```

It **takes ownership** of the value, so at the end of its scope it gets dropped. This makes it a convenient way to **explicitly** drop values **before** end of scope. This can be useful for objects that *do some work on drop*: **releasing locks**, **closing files**, etc.<br>

Why doesn't `std::ops::Drop` take `self`?  If it did, `std::mem::drop` would be called at the end of the block, resulting in another call to `Drop::drop`, and a **stack overflow**!<br>
Compiler calls `std::mem::drop` to drop each value at the end of scope. If `T` implements `Drop`, the compiler automatically inserts a call to `Drop::drop(_x)` at the end of the function.<br>

<br>

### Double free problem
Rust must call `.drop()` **exactly one time** and there is a problem.<br>
Consider following example:
```Rust
{
    let x: Vec<u8> = vec![1, 2];
    let y: Vec<u8> = x;
}
```

For which var `x` or `y` should Rust call the **destructor**? We cannot call the destructor for both because this will result in a **double free**.<br>
To overcome **double free** there are concept of **ownership** and **2nd ownership rule**. So, if variable was moved it is no longer valid and it cannot be used further inside scope and compiler doesn’t call destructor for it at the end of scope.<br>

<br>

### Drop flags
In most cases compiler **knows** all variables for which to call `drop()` **at compile time**.<br>
But sometimes **call** `drop()` or **not** for value **depends on runtime**.<br>
For example, in case of the **assignment is conditional** the **variable** that **should free the memory** can be determined **only at runtime**.<br>

**Drop flags** allow Rust to track whether a variable should be dropped or not **at runtime**.<br>
**Drop flags** store information about which of the alternative variables was actually used.<br>

<br>

**Example**:
```Rust
let x: Box<i32>;
let y: Box<i32>;
let z: Box<i32>;

x = Box::new(10);

if condition {
    y = x;
}
else {
    z = x;
}
```

<br>

# Example: `Vec`
For example, consider a `Vec` type. A `Vec` consists of 2 parts:
- a **static part** that is allocated on the **stack**, it contains
    - a **pointer** to the **buffer** on the heap;
    - the **capacity** (max length) of the buffer;
    - the **length** (current length);
- a **buffer** that is allocated on the **heap** and contains the actual elements of the `Vec`;

<br>

**Bitwise copying** of *static part* of `Vec` duplicates the *static part*, but the **buffer** on the **heap** stays **intact**.<br>
**After** *bitwise copying* two *static parts* of `Vec` are **not** *completely independent copy* and both points to the **same buffer** of `Vec`:<br>
![Bitwise-copy](/img/bitwise_copy.png)

<br>

**After cloning**:<br>
![Deep-copy](/img/deep_copy.png)

<br>