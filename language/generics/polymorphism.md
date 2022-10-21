# Polymorphism
**Polymorphism** (aka **generic programming**) is an approach that allows you to use the same function name with different data types.<br>

**Function** is **polymorphic** if it works for several different types.<br>
**Function** is **monomorphic** if it works only for one type.<br>

The term **polymorphism** means the **ability** of **the same identifier** to **represent** multiple **different types**.<br>

Kinds of polymorphism:
|**Kind of polymorphism**|**Dispatch form**|**Way of implementation**|
|:-----------------------|:----------------|:------------------------|
|**Ad hoc** polymorphism|**Static**|Function or operator **overloading**|
|**Parametric** polymorphism|**Dynamic**: monomorphization|**Generics**|
|**Subtyping** polymorphism|**Dynamic**:- late binding<br>- vtable|**Inheritance**|

<br>

## Ad hoc polymorphism
**Ad hoc** polymorphism means that language allows to define **multiple functions** with **the same name** but with **different signatures** and **definitions**, i.e., every implementation of **ad hoc** function has the same name.<br>
Compiler will dispatch every **ad hoc** function call with appropriate implementation depending on the type of argument to which this call is applied.

**Ad hoc** polymorphism is **not** **part** of the **type system**.

<br>

#### Example
The same function Add has 2 different implementations for 2 types:
```text
function Add(x, y : Integer) : Integer;
begin
    Add := x + y
end;

function Add(s, t : String) : String;
begin
    Add := Concat(s, t)
end;

begin
    Writeln(Add(1, 2));                   (* Prints "3"             *)
    Writeln(Add('Hello, ', 'Mammals!'));  (* Prints "Hello, Mammals!" *)
end.
```

<br>

## Parametric polymorphism
**Parametric** polymorphism means that language allows to define **one function** that can handle values of **different types**.<br>
**Parametric** polymorphism implies that function does not depend on a specific type and has **the same logic** for **every** possible **type**.<br>
**Generic** is a **declaration**/**definition** of **type** (**function**/**struct**/**enum**/**trait**) that contains **type variable** (aka **type parameter**).<br>
**Type var** is often defined in **angle brackets**, e.g. ``<T>`` or ``<E>``. There can be **multiple** *type vars* in **generic**.<br>

**Parametric** polymorphism is a **part of the type system**.<br>

<br>

#### Example of **generic function**:
```Rust
fn f <T>(param: T) {}
```

<br>

## Subtyping polymorphism
**Subtyping** polymorphism is often implemented through **inheritance** in **OOP** languages.<br>

<br>

### Terminology

#### Virtual method
**Virtual method** is a method that **inheritable** and **may be overridden**.<br>
**Virtual methods** are **dispatched** **dynamically**.<br>

<br>

#### Abstract method
A **pure virtual method** (aka **abstract method**) is a **virtual method** that **has** a **declaration** (**signature**) and **no definition** (**implementation**).<br>

<br>

#### Abstract type
**Abstract type** (also **class**) is a type that contains **at least** **one abstract method**.<br>
**Abstract type** **cannot** be instantiated.<br>

<br>

#### Concrete type
**Concrete type** is a type that is **not abstract**.<br>
**Concrete type** **can** be instantiated.<br>

So, an abstract type may provide no implementation, or an incomplete implementation.<br>

<br>

#### Interface
**Interface** (aka **protocol**, **trait**) is an **abstract type** whose methods are **all abstract methods**.<br>
