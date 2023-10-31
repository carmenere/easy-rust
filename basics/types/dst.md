# Dynamically sized types
Most types have a **fixed size** that is **known at compile time** and implement the trait `Sized`.<br>

The `Sized` trait indicates that the size of this type is known *at compile-time*.<br>
`Sized` is always implemented **automatically** by the compiler: by default, any type parameter or associated type has a `Sized` bound.<br>
Also, Rust adds the `Sized` bound to all **generics** by default, so every generic type is `T: Sized` by default.<br>

**Dynamically sized type** (**DST**, **unsized type**, **non-sized**) is a type with a size that is **only known at run-time**.<br>

All **DST** types relax `Sized` bound using `?Sized`.<br>
If type is **DST** (`T: ?Sized`), it must be wrapped by pointer: `&T` or `Box<T>`.<br>

Examples of **DST**:
- `Slice`;
- `Trait`;
- `String`;
