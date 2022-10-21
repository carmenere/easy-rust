# Version resolution
Example of **dependency tree**:
- ``Foo`` -> ``Bar`` -> ``Baz (v1.4)``;
- ``Foo`` -> ``Baz (0.9)``.

This **dependency tree** is **not** **coherent**.

<br>

Notes:
- In Rust your **dependency tree** can have **multiple versions** of the **same crate**.
- Types in different versions of a crate are considered **distinct types**, e.g., ``baz_v1.4::ABC`` != ``baz_0.9::ABC``.

<br>

# Coherence
The problem: if you and I both **implement** a **trait** from **another crate** on **the same type** in **another crate** and we compile the code, **which implementation do we use**?<br>
**Coherence** is about knowing **exactly** which implementation of the code we use.<br>

<br>

### Example: hash table problem
Consider example:
```Rust
mod foo {
    impl Hash for i32 {
        // murmurhash
    }

    fn function(mut table: HashMap<i32, String>) {
        table.insert(0, "hello");
        bar::function(&table);
    }
}
mod bar {
    impl Hash for i32 {
        // crc32
    }
    fn function(mut table: HashMap<i32, String>) {
        assert_eq!(table.get(&0), Some("hello"));
    }
}
```

This 2 modules will calculate **different** values for **the same key**!<br>

<br>

In Rust, **trait coherence** is the property that there is **at most** **one implementation** of a trait for **any given type**.

Rust enforces **trait coherence** through **2 rules**:
- The **overlap rule** forbids you have two ``impl`` of **the same trait** for **the same type**..<br>
- The **orphan rule**, very roughly speaking, forbids you have an ``impl`` where both **the trait** **and** **the type** are defined in a **different crate**.

For example, ``impl<T: Debug>`` trait for ``T`` overlaps with ``impl<T: Display>`` trait for ``T`` because some types might implement both ``Debug`` and ``Display``, so you can't write both.

<br>

## Orphan rule
**Rule**: ``impl SomeTrait for SomeType`` is valid if:
- ``SomeTrait`` is from current crate **OR** ``SomeType`` is from current crate;
- Few edge cases (see **RFC 1023**).


Consider example:
```Rust
impl ToString for Vec<char> {
    fn to_string(&self) -> String {
        String::from_iter(self)
    }
}
```

**Output**:
```bash
cargo run 
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)                                                                                      
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types                                                                   
  --> src/main.rs:39:1                                                                                                                                          
   |                                                                                                                                                            
39 | impl ToString for Vec {                                                                                                                              
   | ^^^^^^^^^^^^^^^^^^---------                                                                                                                                
   | |                 |                                                                                                                                        
   | |                 `Vec` is not defined in the current crate                                                                                                
   | impl doesn't use only types from inside the current crate                                                                                                  
   |                                                                                                                                                            
   = note: define and implement a trait or new type instead                                                                                                     
                                                                                                                                                                
For more information about this error, try `rustc --explain E0117`.                                                                                             
error: could not compile `playrs` due to previous error                                                                                                         
```

<br>

## Overlapping rule
**Rule**: You can never have two ``impl`` of **the same trait** for **the same type**.<br>

- This implementation will **not** compile:
```Rust
impl ABC for Vec<i32>
impl<T> ABC for Vec<T>
```
- This implementation will **not** compile:
```Rust
impl<T> ABC for Vec<T> where T: Eq
impl<T> ABC for Vec<T> where T: Hash
```

Problems:
- Every time you add an ``impl``, you need to check if it overlaps with any other ``impl``.
- The **overlap rule** can only be enforced if you know all of the ``impl`` in the universe.
