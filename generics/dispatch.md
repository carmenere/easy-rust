# Table of contents
- [Table of contents](#table-of-contents)
- [Dispatch](#dispatch)
  - [Monomorphization](#monomorphization)
      - [Example](#example)
  - [Dynamic dispatch](#dynamic-dispatch)
    - [vtable](#vtable)
      - [Example](#example-1)

<br>

# Dispatch
**Dispatch** is the act of sending something somewhere, e.g.,
- dispatch an event to a listener;
- dispatch an interrupt to a handler;
- dispatch a process to the CPU.

**Dispatch** means the process of selecting which **implementation** of a **polymorphic** *function* or *method* to use.

There are two forms of dispatch:
- **static** dispatch (aka **early binding**) – implementation of a polymorphic operation is selected **at compile time**.
- **dynamic** dispatch – implementation of a polymorphic operation is selected **at run time**.


## Monomorphization
**Static dispatch** is used for **generics** and is called **monomorphization**.<br>

**Monomorphization** is a process of creating **specialized** versions of polymorphic entities (function/type/trait) **at compile time**.<br>

Every time compiler sees polymorphic entity it replaces type variable ``T`` with **concrete type** that is **inferred** from passed value and implicitly creates specific version as it was written manually and replaces such polymorphic entity with created monomorphized version.<br>

#### Example
```Rust
fn f<T: ToString>(arg: T) 
{ ... }
```

**Original calls**:
```Rust
f(0.0);
f('a')
```

**Monomorphized versions**:
```Rust
fn f_f64(arg: f64) 
{ ... }

fn f_char(arg: char) 
{ ... }
```

All **original calls** are **replaced** with **monomorphized versions**:
```Rust
f_f64(0.0)
f_char('a')
```

<br>

## Dynamic dispatch
There 2 approaches to implement **dynamic dispatch** for **subtyping polymorphism**:
- Calling a method by **index** in **vtable** at run time.
- Calling a method by **name** at run time (aka l**ate binding**).


### vtable
Compiler creates **vtable** for every type that has at least one virtual method at compile time.<br>
Compiler sets fixed offset in vtable for every virtual method of this class.<br>

**vtable**:
- **vtable** - virtual method table.
- **vtable** is just array of pointers to methods of some class.
- **vtable** cannot be modified at run-time.

Every instance of class inherits its **vptr**.<br>
Every time the compiler encounters a call of virtual method in the code it replaces call with some function, e.g., ``dispatch(instance, index)``.<br>
``dispatch()`` takes **vptr** on instance and access to called method through its index in **vtable** at run time.<br>



#### Example
```Java
#include <iostream>

// Pet an abstract class
class Pet {
public:
    virtual void speak() = 0;
};

class Dog : public Pet {
public:
    void speak()
    {
        std::cout << "Woof!\n";
    }
};

class Cat : public Pet {
public:
    void speak()
    {
        std::cout << "Meow!\n";
    }
};

void speak(Pet& p)
{
    // Which of speak() to use?
    // Lookup vtable of p instance to dispatch .speak() call to Cat::speak() or Dog::speak()
    p.speak(); 
}

int main()
{
    Dog fido;
    Cat simba;
    speak(fido);
    speak(simba);
    return 0;
}

```