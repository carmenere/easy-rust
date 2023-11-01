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
**Declarative macros** are declared using `macro_rules!`.<br>
**Declarative macros** are a bit less powerful but provide an easy to use interface for creating macros to remove duplicate code.<br>
**Declarative macros** also support taking a **nonfixed** number of arguments.<br>

The operators are very similar to the regular expression.<br>
`*` is used for *zero* or *more* **token types**.<br>
`+` is used for *zero* or *one* **argument**.<br>
Syntax `$<parameter_name>:<token_type>` is used to capture parameters is used.<br>
Syntax `$($<parameter_name>:<token_type> ),*` is used to capture nonfixed number of arguments.<br>
Syntax `$(<statement>;)*` is used to repeat code for every captured parameter inside `{ ... }`.

<br>

### Token types
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
