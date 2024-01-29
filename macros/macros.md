# Macros
Rust supports following types of macros:
1. **Declarative macros**.
2. **Procedural /prəˈsiːdʒərəl/ macros**.

<br>



## Import macros as any other item
### File: `m.rs`
```Rust
macro_rules! empty_macros {
    () => {println!("Empty macros")}
}

pub(crate) use empty_macros;
```

<br>

### File: `main.rs`
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

mod m;

use m::{empty_macros};

fn main() {
    empty_macros!();
}
```

<br>

## Declarative macros
**Declarative macros** are those defined using the `macro_rules!` syntax.<br>
**Declarative macros** are used to eliminate boilerplate code.

<br>

Each **declarative macros** have a **name**, and *one* or *more* **rules**.<br>
Each **rule** has **two parts**:
- a **matcher** describes the syntax that it matches;
- a **transcriber** describes the syntax that will **replace** a successfully matched invocation;

<br>

```rust
macro_rules! <NAME> {
    ( <MATCHER_1> ) => { <TRANSCRIBER_1> };
    ...
    ( <MATCHER_N> ) => { <TRANSCRIBER_N> };
}
```

<br>

Every **matcher** has following format: `$name` `:` `token type`.<br>

When a **declarative macro** is invoked by its name, then *macro engine* tries **each matcher** and **transcribes the first successful match**.<br>
In other words, once compiler has matched a declarative macro matcher it **generates code** using associated with this matcher **transcriber**.<br>
The variables defined inside matcher are called **metavariables** and compiler substitute any occurence of each **metavariable** in the transcriber with the appropriate captured input value.<br>

<br>

### Token types
*Token type* also known as *fragment type* or *fragment specifier*.<br>

|**Token type**|**Description**|
|:-------------|:--------------|
|`item`|Any **item** in Rust, e.g., *function*, *struct*, *module*, etc.|
|`block`|Any **block** `{ ... }`.|
|`stmt`|Any **statement**.|
|`pat`|A **pattern**.|
|`expr`|An **expression**.|
|`ty`|A **type**.|
|`ident`|An **identifier**.|
|`path`|A **path**, e.g., `foo`, `::std::mem::replace`,` transmute::<_, int>`.|
|`meta`|A meta item, i.e., the **things** that go **inside** `#[...]` and `#![...]` **attributes**.|
|`tt`|A **single token tree**.|
|`vis`|Possibly an empty **Visibility qualifier**.|

<br>

### Nested declarative macros
It is possible to use **nested** *declarative macros* but with **restriction**: **nested** *declarative macros* **can't** use **literal tokens** in the **matcher**, only `ident`, `lifetime`, and `tt` **token types** can be used.<br>

<br>

### Repetitions
In both the **matcher** and **transcriber**, **repetitions** are indicated by placing the tokens to be repeated inside `$( )`, followed by a **repetition operator**, optionally with a **separator token** between.<br>

The **repetition operators** are:
- `*` indicates **0 or more** repetitions;
- `+` indicates **1 or more** or **at least one**;
- `?` indicates an **optional fragment** with **0** or **1** occurrence; since `?` represents **at most one occurrence**, it **cannot** be used with a **separator**;

<br>

Examples:
- `$( $i:ident),*` represents **any number** of identifiers separated by commas, here comma `,` is **out of** parentheses, because it **isn't** a **part** of *captured value*;
- `$($field_name:ident : $field_type:ty,)*` represents **any number** of fields separated by commas, here comma `,` is **inside** parentheses, because it **is** a **part** of *captured value*;

If there is **repetition** in *matcher* then it can be used in *transcriber* and **it will be repeated once for each match in the input**.<br>
A **metavariable** must appear in **exactly** the **same number** of repetitions in the **transcriber** as it did in the **matcher**.<br>

<br>

### Textual scope
**Declarative macros** use **two** forms of scopes: **textual scope** and **path-based scope**.<br>
**Textual scope** means that **declarative macros** exist in the source code only **after** they are declared.<br>
When *declarative macro* is invoked by an **unqualified identifier** it is **first** looked up in **textual scoping** and **then** in **path-based scoping**.<br>
When *declarative macro* is invoked by a **qualified identifier** it will **skip** the textual scoping lookup and instead **only** do a look up in the **path-based scoping**.<br>

<br>

```rust
mod a {
    // X!(); // UNdefined
}

mod b {
    // X!(); // UNdefined
    macro_rules! X { () => {}; }
    X!();    // Defined
}

mod c {
    // X!(); // UNdefined
}
```

<br>

If it has the `#[macro_export]` attribute, then it is declared in the crate root scope and can be referred to similar to how you refer to any other item.<br>

<br>

### Hygiene
**Hygiene** works by attaching an invisible syntax context value to all identifiers.<br>
When two identifiers are compared, both the identifiers' textual names and syntax contexts must be identical for the two to be considered equal.<br>

Tokens that were passed to macros **retain** their **original syntax context**.<br>

Consider macros:
```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}

let four = using_a!(a / 10);
```

<br>

It expands to:<br>
```rust
let four = {
    let a = 42;
    a / 10
};
```

Here `let a = 42;` and `a / 10` are 2 **different** identifiers `a`.<br>

The solution is to modify the macro as follows:
```rust
macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let $a = 42;
            $e
        }
    }
}

let four = using_a!(a, a / 10);
```

Because tokens **retain** their **original syntax context**, here `$a` will be expanded to `a` and `$e` will be expanded to `a / 10` and in both variants `a` is **the same** identifier.

<br>

### `$crate`
For the macro to be truly reusable you cann't assume anything about what items are in scope at the caller. Maybe the caller has its own type `Option`.<br>
To write safe macros use fulli qualified paths for items, e.g., `::core::option::Option`.<br>
If you want to refer to something in the crate that defines the macro, use special **metavariable** `$crate`.<br>

<br>

# Procedural macros
**Procedural macros** allow you to operate on the **AST** (**abstract syntax tree**).<br>
A **procedural macro** is a **function** that recieves one or more `TokenStream` and returns `TokenStream` and its **output** **replaces** the **macro invocation**.<br>

There are **3 types** of **procedural macros**:
1. **Custom** `#[derive]` **macros** that specify code added with the **derive attribute** used on **structs** and **enums**.
2. **Attribute-like macros** that define **custom attributes** usable on **any item**.
3. **Function-like macros** that look like function calls but operate on the tokens specified as their argument. **Function-like macros** are invoked with the macro invocation operator `!` like *declarative macros*.<br>


<br>

Every type of macro must be annotated with appropriate attribute:
1. Function for **derive macros** with `proc_macro_derive`.
2. Function for **attribute-like macros** with `proc_macro_attribute`:
```Rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

}
```
Here, we have **2 parameters** of type `TokenStream`:
   - the **first** is for the **contents of the attribute**: the `GET, "/"` part. 
   - the **second** is the **body of the item** the attribute is attached to: in this case, `fn index() {}`.
```Rust
#[route(GET, "/")]
fn index() {

}
```
3. Function for **function-like macros** with `proc_macro`.
```Rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {

}
```
```Rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

<br>

## Procedural macro crate
**Procedural macros** need to be in their **own crate**.<br>
The convention for structuring crates and macro crates is as follows: for a crate named `foo`, a **custom derive procedural macro** crate is called `foo_derive`.<br>

<br>

# Example: custom derive macros
1. Create **procedural macro crate**:
```bash
$ cargo new hello_macro_derive --lib
```
2. Then **declare** the `hello_macro_derive` crate as a **procedural macro crate** (set `proc-macro = true`) in `hello_macro_derive/Cargo.toml`:
```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```
3. Define the **custom derive procedural macro**:
```Rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

<br>

The `hello_macro_derive` function will be called when `#[derive(HelloMacro)]` is specified on **structs** and **enums**.<br>
The `quote!` macro lets us define the Rust code that we want to return.<br>
The `quote!` macro also provides some very cool **templating mechanics**: we can enter `#name`, and `quote!` will replace it with the value in the variable `name`.<br>
The `stringify!` macro used here is built into Rust. It takes a Rust **expression**, such as `1 + 2`, and **at compile time** **turns the expression into a string literal**, such as `"1 + 2"`. This is different than `format!` or `println!`, macros which evaluate the expression and then turn the result into a `String`.<br>

<br>

# More examples
More examples [here](https://github.com/carmenere/easy-rust/blob/main/examples/macros/macros.md).
