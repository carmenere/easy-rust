# Thiserror and Anyhow
There are 2 useful crates to handle errors in Rust:
- **Anyhow**
- **Thiserror**

<br>

**Anyhow** just **wraps any** *error type* ``E`` with ``Box<dyn Error + Send + Sync>``, i.e., **Anyhow** abstracts from any kind of returned error and simplifies writing of code.<br>
**Thiserror** is a **macro** to derive ``std::error::Error``, ``std::fmt::Display``, ``std::convert::From`` traits. Anything that you write with **Thiserror** could sensibly be written by hand.<br>

Recommendations:
- use **Anyhow** if you **don't care** what error type your functions return. This is common in application code. 
- use **Thiserror** if you **writes a library** that wants to design your **own dedicated** error type(s) so that on failures the caller gets exactly the information that you choose.
