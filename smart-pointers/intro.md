**Smart pointers** are data structures that act like a pointer but also have additional metadata and capabilities. 
Rust, with its concept of ownership and borrowing, has an additional difference between **references** and **smart pointers**: while *references* **only** **borrow** data, in many cases, *smart pointers* **own** the data they point to.

The most common **smart pointers** in the standard library:
- ``Box<T>`` for allocating values on the **heap**;
- ``Rc<T>`` and ``Arc<T>`` a reference counting type that enables multiple ownership;
- ``Ref<T>`` and ``RefMut<T>`` for **enforcing** the *borrowing rules* **at runtime** instead of compile time.
- ``Cell<T>`` and ``RefCell<T>`` for **interior mutability**.
