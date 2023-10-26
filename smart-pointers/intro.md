# Smart pointers
**Smart pointers** are data structures that act like a pointer but also have additional metadata and capabilities. 
Rust, with its concept of ownership and borrowing, has an additional difference between **references** and **smart pointers**: while *references* **only** **borrow** data, in many cases, *smart pointers* **own** the data they point to.

<br>

|Pointer type|Path in `std`|Description|
|:-----------|:------------|:----------|
|`Arc<T>`|[std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)|`Arc` stands for **Atomic Reference Counter**.<br>It's **threadsafe** version of `Rc`.|
|`Box<T>`|[std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)|It's for allocating values on the **heap**. It's for **single ownership**.|
|`Cell<T>`|[std::cell::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)|It's for **interior mutability** for `Copy` types.|
|`Cow`|[std::borrow:Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)|Clone on write smart pointer.|
|`Rc<T>`|[std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)|`Rc` stands for **Reference Counter**.<br>`Rc` type enables **multiple ownership** or **shared ownership**.<br>It's **non-threadsafe**.|
|`Ref<T>`|[std::cell::Ref](https://doc.rust-lang.org/std/cell/struct.Ref.html)|It's for enforcing the *borrowing rules* **at runtime** instead of compile time.|
|`RefCell<T>`|[std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)|It's for **interior mutability** for `non-Copy` types.|
|`RefMut<T>`|[std::cell::RefMut](https://doc.rust-lang.org/std/cell/struct.RefMut.html)|It's for enforcing the *borrowing rules* **at runtime** instead of compile time.|

<br>

# Raw pointrs
- `*const T`
- `*mut T`

These are **raw pointers** with **no lifetime** or **ownership** attached to them.<br>
They just point to some location in memory with no other restrictions. 