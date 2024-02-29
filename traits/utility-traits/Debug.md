# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Debug](#debug)
  - [Example: Debug](#example-debug)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Debug`|[std::fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)|

<br>

# Debug
`Debug` and `Display` traits are part of the `std::fmt` module.<br>
`Debug` is **derivable** trait, but `Display` **not** and have to manually write the implementation.<br>
Implementing `Display` trait will **automatically implement** `ToString`.<br>

<br>

## Example: Debug

