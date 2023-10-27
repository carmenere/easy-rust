# Example: function that returns `&str` and `String`
Consider function:<br>
```Rust
fn remove_spaces(input: &str) -> String {
   let mut buf = String::with_capacity(input.len());

   for c in input.chars() {
      if c != ' ' {
         buf.push(c);
      }
   }

   buf
}
```

<br>

If `input` hasn't spaces then `remove_spaces()` **allocates** new memory **anyway**.<br>
Obvious it needn't to create `buf` when `input` **hasn't** spaces and it's reasonable to return given `input` back to the caller. There are some ways to do that.<br>

<br>

## Variant 1: using `String` for `input`
```Rust
fn remove_spaces(input: String) -> String { ... }
```

<br>

This solution has 2 drawbacks:
1. It forces the caller to **move** the ownership of `input` into function `remove_spaces()`.
2. It forces the caller to **convert** `&str` into a `String` which causes to allocation of new memory.

<br>

## Variant 2: using `Cow` type for return value
```Rust
use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}
```


<br>

## Variant 3: using `Cow` + `.into()`
```Rust
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        let v: Vec<char> = input.chars().collect();

        for c in v {
            if c != ' ' {
                buf.push(c);
            }
        }

        return buf.into();
    }
    return input.into();
}
```

<br>

## Variant 4: using adapters
```Rust
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into()
    } else {
        input.into()
    }
}
```
