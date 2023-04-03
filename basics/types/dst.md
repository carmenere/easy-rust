# Dynamically sized types
Most types have a fixed size that is known at compile time and implement the trait ``Sized``.<br>

The ``Sized`` trait indicates that the size of this type is known *at compile-time.*<br>
``Sized`` is always implemented automatically by the compiler: by default, any type parameter or associated type has a ``Sized`` bound.<br>

**Dynamically sized type** (**DST**, **unsized type**, **non-sized**) is a type with a size that is known **only** *at run-time*.<br>

All **DST** types relax ``Sized`` bound using ``?Sized``. 

Examples of **DSTs**:
- ``Slice``;
- ``Trait``.

<br>

- Rust adds the ``T: Sized`` bound to all **generics** by default.<br>
- Adding ``?Sized`` allows **DST** too.

<br>