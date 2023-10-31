# Scalars
|Type group|Types|
|:---------|:----|
|**Integer**|**Signed**: ``u8``, ``u16``, ``u32``, ``u64``, ``u128``<br>**Unsigned**: ``i8``, ``i16``, ``i32``, ``i64``, ``i128``.<br>**Sizes of pointers**: ``isize``, ``usize`` and they depend on **arch**.|
|**Float**|``f32``, ``f64``|
|**Boolean**|``false``<br>``true``|
|**Character**|**One letter** in **single quotes**.<br>Example: ``let ch = 'A';``|

<br>


# Number separator
The ``_`` symbol is called **number separator** and is used in **literals**.

#### Example
```Rust
let a = 1_000_000;
let b = 1u64;
let c = 1_u64;
```