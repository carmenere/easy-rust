# Table of contents
- [Table of contents](#table-of-contents)
- [Combinators on `Option`/`Result`](#combinators-on-optionresult)
  - [`match` example](#match-example)
  - [`if let` example](#if-let-example)
  - [Option/Result methods](#optionresult-methods)
  - [`.unwrap_or()` and  `.unwrap_or_else()`](#unwrap_or-and--unwrap_or_else)

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

<br>

## Option/Result methods
- `.ok()` it converts **from** `Result<T, E>` **to** `Option<T>`
  - What if you have a function that returns a `Result` but you must to return an `Option`. No problem: there is a method called `.ok()`;
  - all that can be passed on from a `Result` to an `Option` is the information inside an `Ok` variant and because `None` **doesn’t hold** any information any `.ok()` **drops any error** inside `Err` variant;
- `.ok_or(error)` and `.ok_or_else()`:
  - both **turn** an `Option` **into** `Result`;
  - because `None` in an `Option` **doesn’t have** any information, but `Result` can have `Ok` **or** `Err`, so you have to let it know what the `error` **value** will be for `None`;
  - `ok_or_else(|| ...)` receives the **closure**;
- `.unwrap_or(value)` returns a default `value` if an `Option` is a `None` or `Result` is an `Err`;
- `.unwrap_or_else(|| ... )` allows us to give a default value, but it uses a **closure** that we can use to write some more **complex logic**;

<br>

## `.unwrap_or()` and  `.unwrap_or_else()`
Consider example:
```rust
fn main() {
    let v = vec![1, 2, 3];

    let fourth = v.get(3).unwrap_or_else(|| {
        if let Some(val) = v.get(2) {
            val
        }
        else {
            &0
        }
    });
    println!("{}", fourth);
}
```

*First*, we try to get an item at index **3**, *then* if it was `None` we try to get item one index back and *finally* we return a `&0` in case **no items have been found** at either index.<br>

<br>