# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)
  - [Readers and writers](#readers-and-writers)
  - [`io::Read`](#ioread)
  - [`io::Write`](#iowrite)
  - [`io::BufRead` and `io::BufReader`](#iobufread-and-iobufreader)
  - [`io::BufWriter`](#iobufwriter)
    - [Example](#example)
  - [`io::Seek`](#ioseek)
    - [Example](#example-1)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`fmt::Write`|[std::fmt::Write](https://doc.rust-lang.org/stable/std/fmt/trait.Write.html)|
|`io::Write`|[std::io::Write](https://doc.rust-lang.org/stable/std/io/trait.Write.html)|
|`io::Read`|[std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html)|
|`io::BufRead`|[std::io::BufRead](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html)|
|`io::BufReader`|[std::io::BufReader](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html)|
|`io::BufWriter`|[std::io::BufWriter](https://doc.rust-lang.org/stable/std/io/struct.BufWriter.html)|
|`io::Seek`|[std::io::Seek](https://doc.rust-lang.org/stable/std/io/trait.Seek.html)|

<br>

# In a nutshell
## Readers and writers
Traits `io::Read`, `io::Write` and `fmt::Write` provide general interface for reading and writing data:
- **reader** is a type that implements `io::Read`;
- **writer** is a type that implements `io::Write` or/and `fmt::Write`;

<br>

**Note**, traits `io::Read`, `io::Write` and `fmt::Write` are **not** part of Rust prelude and **must be imported explicitly**.

<br>

## `io::Read`
Trait `io::Read` is for reading bytes from a **source**.<br>
Method `.read()` is unbuffered, it returns how many bytes were read:
- if `.read()` returns `Ok(n)` then implementation **must guarantee** that `0 <= n <= buf.len()`;

<br>

## `io::Write`
Trait `io::Write` is for writing bytes.
Method `.write()` is unbuffered, it returns how many bytes were written:
- if `.write()` returns `Ok(n)` then implementation **must guarantee** that `n <= buf.len()`;
- if `.write()` returns `Ok(0)` it means that destination object is no longer able to accept bytes;

<br>

## `io::BufRead` and `io::BufReader`
A `BufRead` is a type of `Reader` which has an **internal buffer**, allowing it to perform extra ways of reading.<br>
The `BufReader<R>` struct wraps **any reader** adding **internal buffer** to it.<br>

If something implements `Read` (for example, `std::fs::File`) it can be **wrapped** by `BufReader`:
```rust
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("log.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {len} bytes long");
    Ok(())
}
```

<br>

## `io::BufWriter`
Wraps **any writer** and **buffers** its output.<br>
It can be excessively inefficient to work directly with something that implements `Write`. For example, every call to write on `TcpStream` results in a **system call**. A `BufWriter<W>` keeps an in-memory buffer of data and writes it to an underlying writer in large, infrequent batches.<br>
It is **critical** to call `flush` before `BufWriter<W>` is dropped.

<br>

### Example
Let’s write the numbers one through ten to a TcpStream:
```rust
use std::io::prelude::*;
use std::net::TcpStream;

let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

for i in 0..10 {
    stream.write(&[i+1]).unwrap();
}
```

<br>

Because we’re not buffering, we write each one in turn, incurring the overhead of a **system call per byte written**. We can fix this with a `BufWriter<W>`:
```rust
use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;

let mut stream = BufWriter::new(TcpStream::connect("127.0.0.1:34254").unwrap());

for i in 0..10 {
    stream.write(&[i+1]).unwrap();
}
stream.flush().unwrap();
```

<br>

By wrapping the stream with a `BufWriter<W>`, these ten writes are all grouped together by the buffer and will all be written out in **one system call** when the stream is flushed.

<br>

## `io::Seek`
Trait `Seek` provides cursor which can be moved within a stream of bytes.<br>

Possible positions for cursor:
```rust
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
```

<br>

### Example
```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt")?;

    // move the cursor 42 bytes from the start of the file
    f.seek(SeekFrom::Start(42))?;

    // move the cursor 4 bytes from the end of the file
    f.seek(SeekFrom::End(-4))?;
    Ok(())
}
```