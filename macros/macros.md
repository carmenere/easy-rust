# Macros
Rust supports following types of macros:
1. **Declarative macros**.
2. **Procedural /prəˈsiːdʒərəl/ macros**.

<br>

## Declarative macros
**Declarative macros** are declared using ``macro_rules!``.<br>

**Declarative macros** are a bit less powerful but provide an easy to use interface for creating macros to remove duplicate code.<br>

**Declarative macros** also support taking a nonfixed number of arguments.<br>

The operators are very similar to the regular expression.<br>
``*`` is used for *zero* or *more* **token types**.<br>
``+`` is used for *zero* or *one* **argument**.<br>
Syntax ``$<parameter_name>:<token_type>`` is used to capture parameters is used.<br>
Syntax ``$($<parameter_name>:<token_type> ),*`` is used to capture nonfixed number of arguments.<br>
Syntax ``$(<statement>;)*`` is used to repeat code for every captured parameter inside ``{ ... }``.

<br>

### Token types
|**Token type**|**Description**|
|:-------------|:--------------|
|``item``|Any **item** in Rust, e.g., *function*, *struct*, *module*, etc.|
|``block``|Any **block** ``{ ... }``.|
|``stmt``|Any **statement**.|
|``pat``|A **pattern**.|
|``expr``|An **expression**.|
|``ty``|A **type**.|
|``ident``|An **identifier**.|
|``path``|A **path**, e.g., ``foo``, ``::std::mem::replace``,`` transmute::<_, int>``.|
|``meta``|A meta item, i.e., the **things** that go **inside** ``#[...]`` and ``#![...]`` **attributes**.|
|``tt``|A **single token tree**.|
|``vis``|Possibly an empty **Visibility qualifier**.|

<br>

# Procedural macros
**Procedural macros** allow you to operate on the **abstract syntax tree** (**AST**).<br>
A **procedural macro** is a function that recieves one or more ``TokenStream`` and returns ``TokenStream`` and its **output** **replaces** the **macro invocation**.<br>

There are **3 types** of **procedural macros**:
1. **Attribute-like macros**.
2. **Derive macros**.
3. **Function-like macros**.

<br>

# Attribute-like macros
**Attribute-like macros** enable you to create a **custom attribute**.<br>
They can also take arguments.<br>

```Rust
#[some_attribute_macro(some_argument)]
fn perform_task(){
    // some code
}
```

<br>

## Derive macros
**Derive macros** implements **traits** automatically.<br>

```Rust
#[derive(SomeTrait)]
struct MyStruct{}
```

<br>

## Function-like
**Function-like macros** are **similar** to *declarative macros* in that they’re invoked with the macro invocation operator ``!`` and **look like function calls**.<br>

#### Example
```Rust
#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
            fn anwser() → i32 {
                5
            }
    ))
}
```
