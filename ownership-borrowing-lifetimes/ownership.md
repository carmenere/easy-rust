# Ownership
## Basic terms
|Term|Meaning|
|:---|:------|
|**Type**|Is a *set of* 1) **allowed values**; 2) **memory layout** for values; 3) **allowed operations** for values.|
|**Owner**|Identifier that is owning value.|
|**Bind**|Associate identifier with a value.|
|**Copy**|Associate identifier with **bitwise copied** value and **keep the original identifier valid**.|
|**Move**|Associate identifier with **bitwise copied** value and **invalidate the original identifier**.|
|**Clone**|Associate identifier with **completely independed duplicate** of value and **keep the original identifier valid**.|
|**Mutate**|Change the value associated with a mutable identifier. By default, Rust data is immutable - it can't be changed.|
|**Borrowing**|The **action of creating a reference**. Reference is a named pointer to some value.|

<br>

## Variable scope
**Scope** (or **variable scope**) is the **range** within a program for which a variable is **valid**.<br>
In Rust scope has **explicit boundaries**: **opening** curly bracket ``{`` and **closing** curly bracket ``}``.<br>
In Rust variable is valid from the point at which it was declared by ``let`` keyword until the **end of scope**: closing curly bracket ``}``.<br>
**Variable** or **variable’s identifier** or **identifier** are the synonyms.<br>

```Rust
{ // Scope has started
    // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point
    // ...
} // Scope is over, and s is no longer valid
```

<br>

## Ownership rules
**Ownership** enables Rust to make **memory safety guarantees** without needing GC.

Ownership rules:
1. **Each value** in Rust **has** an **identifier** that’s called its **owner**.
2. There can only be **one owner at a time**.
3. When the **owner** goes **out of scope**, the **value** will be **dropped**, (**scope based resource management**).

In C++ this **pattern of deallocating resources at the end of variable lifetime** is called **RAII**.  Doing this correctly without GC is a difficult problem.

<br>

## Blittable and Non-blittable types
**Completely independent copy** of object is such copy that can be safely used separately to the origin one.<br>

Types of copying:
- **Bitwise copy** (aka **shallow copy**/**bit block transfer**) **type-independed logic** to duplicate values and implemented in syscall ``memcpy()``, in other words, **bitwise copy** copies contiguous block of memory bit-by-bit (byte-by-byte) to another location.
- **Semantic copy** (aka **deep copy**) requires **type-specific logic** to duplicate values safely.

**bitblt**/**bit blit**/**blit** are contractions for **bit block transfer**.

|Type|Layout in memory|Type of copying|
|:---|:---------------|:---|
|**Blittable type**|Object of blittable type occupies exactly one contiguous /kəntɪgjuəs/ block of memory.|**Bitwise copy** *can* create completely independent copy of object of blittable type.|
|**Non-blittable type**|Object of non-blittable type occupies more than one not adjacent contiguous /kəntɪgjuəs/ blocks of memory.|**Bitwise copy** *cannot* create completely independent copy of object of non-blittable type. To create completely independent copy of object of non-blittable type **semantic copy** must be used.|

<br>

## Copy and move semantics
Any value in Rust has either **Move type** or **Copy type**.

Types:
1.	**Move types** are types that **don't** *implement* the **Copy trait**. 
    - If type is the **Move type** it *may* implement or *may not* implement **Clone trait**.
2.	**Copy types** are types that *implement* the **Copy trait**. 
    - **Clone trait** is a supertrait of **Copy trait**, if type is the **Copy type** it also must implement **Clone trait**.
3.	**Clone types** are types that implement the **Clone trait**.

Following operations in Rust **assignment**/**passing a value to function**/**returning a value from function** have **semantics**. <br>

**Semantics of operation** is determined by **semantics of type**:
- *Copy types* have **copy semantics**.
- *Move types* have **move semantics**.

In Rust language: **copy semantics** and a **move semantics** are **mechanically the same** – they **both** implicitly **use bitwise copy**, e.g., ``memcpy()``.<br>

**Copy semantics** and a **move semantics** are differ in ownership:
|**Type’s semantics**|**Ownership**|**Original (source) identifier**|
|:---------------|:--------|:---------------------------|
|**Copy semantics**|**Doesn’t transfer ownership** to another identifier after operation.|Original identifier **remains valid** after operation and **can be used further**.|
|**Move semantics**|**Transfers ownership** to another identifier after operation.|Original identifier **becomes invalidated** after operation, i.e., it **no longer valid**  and **cannot be used further**.|

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

## Clone trait
Rust **will never automatically** create deep copies of your data.<br>
**Deep copy** in Rust **is always explicit action**: ``x.clone()``.<br>
**Clone trait** is used to implement **deep copy** in ``clone()`` method.

<br>

## Copy trait
**Copy trait** is **marker trait**.<br>
**Copy trait** is implemented in Rust language on:
- the **atomic primitive types**;
- the **composite primitive types** if all their constituent /kənstɪtʃuənt/ types implement the **Copy trait**;
- **shared references**.

<br>

Example of implementing **Copy trait** manually:
```Rust
struct MyStruct;
impl Copy for MyStruct {}
``` 

<br>

``Clone::clone`` implementation for **Copy type** should just be a ``memcpy()`` and it is enough to return ``*self``, example:
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
**Copy trait** and **Clone trait** are **derivable traits**. It means their implementations can be derived automatically by **derive attribute**.

The **derive attribute** generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax.

Example:
```Rust
#[derive(Debug, Copy, Clone)]
struct Foo;
```

<br>

## Why does Copy require Clone?
**Clone trait** is a **supertrait** of **Copy trait**, if type is the **Copy type** it also must implement **Clone trait**.<br>
But *copy semantics* **will never call** ``Clone::clone``. So, why does **Copy require Clone**? **Convention**?

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
A **resource management** in Rust is **scope based**.
**Drop trait** is used to implement **destructors** for types.<br>
If type implements the **Drop trait** it cannot implement **Copy trait**. So, it can only be of **Move type**, **even if blittable**.


### Double free problem
Rust must call ``.drop()`` **exactly one time** and there is a problem.<br>
Consider following example:
```Rust
{
    let x: Vec<u8> = vec![1, 2];
    let y: Vec<u8> = x;
}
```

For which var ``x`` or ``y`` should Rust call the **destructor**? We cannot call the destructor for both because this will result in a **double free**. 

To overcome **double free** there are concept of **ownership** and **2nd ownership rule.**. So, if variable was moved it is no longer valid and it cannot be used further inside scope and compiler doesn’t call destructor for it at the end of scope.

<br>

### Drop flags
If the **assignment is conditional**, then Rust can define the variable that should free the memory **only at runtime**. To do this, the compiler generates a flag on the stack (drop flags), which stores information about which of the alternative objects was actually used and calls the appropriate drop.

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

## Primitive and Non-primitive types
In Rust language:
1. If type has a **known size** *at compile time* (**fixed size**) it is called **primitive type**.
- all **primitive types** implement **Copy trait** and have **copy semantics**.
- all **primitive types** are stored entirely on the **stack**.
2. If type has an **unknown size** a*t compile time* it is called **complex type**. 
- **complex types** have **move semantics**.
- **static part** (aka **control part**) of complex type is stored entirely on the **stack** and it is used to manage **dynamic part**.
- **dynamic part** of complex type is **dynamically changed** *at run time* and it is stored in the **heap**.

So, in Rust language **Copy type** is **primitive type**.

Examples:
- ``&str`` **type** (**string literal**), e.g., ``let s: &str = "ABC"``, has **known size** *at compile time*. So the text is **hardcoded directly into the executable**.  
- ``String`` **type**, e.g., ``String::from("ABC")``, has **unknown size** *at compile time*. It is growable piece of text. Memory for ``String`` is **requested** and **dynamically changed** *at run time*.

<br>

### Atomic primitive types
**Atomic primitive types** are defined by the compiler and cannot be customized by the user.

|**Type**|**Description**|
|:-------|:--------------|
|**Bool**|The ``bool`` type has two values: ``true`` and ``false``.|
|**Machine-independent integer**|**Signed** integers:``i8``, ``i16``, ``i32``, ``i64``.<br>**Unsigned** integers: ``u8``, ``u16``, ``u32``, ``u64``.|
|**Machine-dependent integer**|**Signed** integers: ``isize``.<br>**Unsigned** integers: ``usize``.|
|**Floating point**|32-bit floating point: ``f32``. <br>64-bit floating point: ``f64``|
|Textual types|``char``: contains a Unicode scalar value in 4 bytes.<br>``&str``: **string literal**, contains a Unicode string (a sequence of Unicode scalar values).|

<br>

### Composite primitive types
|**Type**|**Description**|
|:-------|:--------------|
|**Arrays** and **slices**|Array ``[T; N]`` contains ``N`` elements of the type ``T``.|
|**Tuples**|The tuple type ``(T1, T2, ..., Tn)`` contains a sequence of elements where each element may be of a different type.|

Composite primitive types automatically implement the **Copy trait** if all their constituent types implement the **Copy trait**.

<br>

### Complex types
- ``Vec<T>``
- ``String``
- ``Map<T>``
- ``Set<T>``
