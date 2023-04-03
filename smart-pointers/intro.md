# Smart pointers
**Smart pointers** are data structures that act like a pointer but also have additional metadata and capabilities. 
Rust, with its concept of ownership and borrowing, has an additional difference between **references** and **smart pointers**: while *references* **only** **borrow** data, in many cases, *smart pointers* **own** the data they point to.

<br>

|Pointer type|Path in `std`|Description|
|:-----------|:------------|:----------|
|`Arc<T>`|[std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)|**Reference counting** type (**thread-safe**) that enables **multiple ownership**.|
|`Box<T>`|[std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)|For allocating values on the **heap**.|
|`Cell<T>`|[std::cell::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)|For **interior mutability**.|
|`Cow`|[std::borrow:Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)|Clone on write smart pointer.|
|`Rc<T>`|[std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)|**Reference counting** type that enables **multiple ownership**.|
|`Ref<T>`|[std::cell::Ref](https://doc.rust-lang.org/std/cell/struct.Ref.html)|For enforcing the *borrowing rules* **at runtime** instead of compile time.|
|`RefCell<T>`|[std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)|For **interior mutability**.|
|`RefMut<T>`|[std::cell::RefMut](https://doc.rust-lang.org/std/cell/struct.RefMut.html)|For enforcing the *borrowing rules* **at runtime** instead of compile time.|
