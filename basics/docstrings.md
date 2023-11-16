# Docstrings in Rust
|Docstring style|Description|
|:----|:----------|
|`///` *line 1*<br>`///`<br>`///` *line 2*<br>**fn _f_() { … }**|The `///` syntax is used to document the **item next to** `///`.<br>It is called an **outer documentation**.<br>`///` without any text after it is interpreted as **line break**.|
|**fn _f_() {**<br>&nbsp;&nbsp;&nbsp;&nbsp;`//!` *line 1*<br>&nbsp;&nbsp;&nbsp;&nbsp;`//!`<br>&nbsp;&nbsp;&nbsp;&nbsp;`//!` *line 2*<br>**}**|The `//!` syntax is used to document the **item enclosing** `//!`.<br>It is called an **inner documentation**.<br>It is often used when documenting the `.rs` **file itself**, because nothing comes before it.<br>`//!` without any text after it is interpreted as **line break**.|

Docstrings in Rust support **Markdown** syntax.