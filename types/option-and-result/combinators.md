# Table of contents
- [Table of contents](#table-of-contents)
- [Combinators on `Option`/`Result`](#combinators-on-optionresult)
  - [`match` example](#match-example)
  - [`if let` example](#if-let-example)

<br>

# Combinators on `Option`/`Result`
The only way to safely interact with `Option` and `Result` **inner values** is either through **pattern matching** or **if let**.<br>
This paradigm of using **matching** is a very common operation and, as such, it becomes very tedious having to write them every time.<br>
Fortunately, `Option` and `Result` come with lots of **helper methods** implemented on them, also known as **combinators** or **adapters**, that allow you to manipulate the **inner values** easily.

<br>

## `match` example
```Rust
match some_value {
    Some(v) => 1,
    None => 10
}
```

<br>

## `if let` example
```Rust
if let Some(v) = some_value {
    println!("Matched {:?}!", v);
}
```

The `if let` construct reads: if `let` destructures `some_value` into `Some(v)`, evaluate the block (`{ }`).
