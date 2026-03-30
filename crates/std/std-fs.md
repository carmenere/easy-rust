# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Readers and Writers](#readers-and-writers)
  - [Readers](#readers)
  - [Buﬀered Readers](#buﬀered-readers)
  - [Writers](#writers)
  - [Files](#files)
  - [Seeking](#seeking)
  - [`std::fs::FileType`](#stdfsfiletype)
  - [`std::fs::DirEntry`](#stdfsdirentry)
  - [Platform-Specific features](#platform-specific-features)
- [Using files](#using-files)
- [`std::ffi::OsStr`, `std::ffi::OsString`, `std::path::Path` and `std::path::PathBuf`](#stdffiosstr-stdffiosstring-stdpathpath-and-stdpathpathbuf)
<!-- TOC -->

<br>

# Readers and Writers
In most languages, files are **buﬀered by default**. If you want **unbuﬀered** input or output, you have to figure out how to **turn buﬀering oﬀ**.<br>
In Rust, `File` and `BufReader` are two **separate** library features, because sometimes you want **files without buﬀering**, and sometimes you want **buﬀering without files**.<br>

In Rust, a `File` implements `Read` but **not** `BufRead`, so `File` is an **unbuﬀered reader**.<br>

However, it’s easy to create a **buﬀered reader** for a `File`, or any other **unbuﬀered reader**. The `BufReader::new(reader)` does this. To set the *size of the buﬀer*, use `BufReader::with_capacity(size, reader)`.<br>

<br>

Rust’s standard library features for **input** and **output** are organized around **3 traits**, `std::io::Read`, `std::io::BufRead`, and `std::io::Write`:
- types that **implement** `Read` have methods for **byte-oriented input**. They’re called **readers**;
- types that implement `BufRead` are **buﬀered readers**. They support all the methods of `Read`, plus methods of `BufRead`;
- types that **implement** `Write` support both **byte-oriented** and **UTF-8 text output**. They’re called **writers**;

There’s a **prelude module** containing only those 3 traits: use `std::io::prelude::*`.<br>

**Note**, there is **no** method for **closing** a *reader* or *writer*: *readers* and *writers* typically implement `Drop` so that they are **closed automatically**.<br>

<br>

- `std::io::Read`:
  - `std::io::Stdin` is returned by `io::stdin()`
  - `std::fs::File`
  - `std::net::TcpStream`
  - `std::process::Command`:
    - the `std::process::Command` does **not** implement the `std::io::Read` trait, instead, you use methods on `Command` to spawn a child process, which then provides handles (like `ChildStdout`) that do implement `std::io::Read`:
      - `ChildStderr` a handle to a child process’s **stderr**;
      - `ChildStdin` a handle to a child process’s **stdin**;
      - `ChildStdout` a handle to a child process’s **stdout**;
  - `std::io::BufRead`:
    - `std::io::BufReader<R>`
    - `Cursor<&[u8]>`
    - `StdinLock`
- `std::io::Write`:
  - `std::io::Stdout` is returned by `io::stdout()`
  - `std::io::Stderr` is returned by `io::stderr()`
  - `std::fs::File`
  - `std::net::TcpStream`
  - `std::io::BufWriter<W>`
  - `Vec<u8>`

<br>

## Readers
`std::io::Read` methods for reading data:
- `reader.read(&mut buffer)`
  - type of the `buffer` is `mut [u8]`;
  - this reads up to `buffer.len()` bytes from the data source and stores them in the given `buffer`;
  - returns `Result<u64, io::Error>`
    - **on success**, returns `Ok(n)`, where `n` is the **number of bytes** read which **may be equal to or less than** `buffer.len()`;
      - `Ok(0)` means there is **no more input to read**;
    - **on error**, returns `Err(err)`, where `err` is an `io::Error` value;
      - an `io::Error` has a `.kind()` method that returns an **error code** of type `io::ErrorKind`;
- `reader.read_to_end(&mut buffer)`
  - type of the `buffer` is `mut Vec<u8>`;
  - this reads **unlimited** number of bytes from the data source and stores them in the given `buffer`;
- `reader.read_to_string(&mut buffer)`
  - type of the `buffer` is `mut String`;
  - this reads **unlimited** number of bytes from the data source and stores them in the given `buffer`;
  - if the stream **isn’t** valid UTF-8, this returns an `ErrorKind::InvalidData` error;
- `reader.read_exact(&mut buffer)`
  - type of the `buffer` is `mut [u8]`;
  - this reads exactly `buffer.len()` bytes from the data source and stores them in the given `buffer`;
  - if the `reader` **runs out** of data before reading `buf.len()` bytes, this returns an `ErrorKind::UnexpectedEof` error;

In addition, there are methods that take the `reader` **by value**, transforming it into an iterator or a diﬀerent reader:
- `reader.chain(reader2)`
  - returns a **new reader** that produces all the input from `reader`, followed by all the input from `reader2`;
- `reader.take(n)`
  - returns a **new reader** that reads from the same source as `reader`, but is **limited to** `n` **bytes** of input;

<br>

The members of **enum** `io::Error` have names like `PermissionDenied` and `ConnectionReset`. Most indicate **serious errors** that **can’t be ignored**, but one kind of error should be handled specially: `io::ErrorKind::Interrupted` corresponds to the Unix error code `EINTR`, which means the read was interrupted by a signal. Unless the program is designed to do something clever with signals, it should **just retry the read**.<br>

<br>

## Buﬀered Readers
Readers and writers can be buﬀered , which simply means they have a buﬀer that holds some input or output data in memory. This saves on system calls:
![](/img/buffered_reader.png)

<br>

**Buﬀered readers** implement both `Read` and `BufRead`, which adds the following methods:
- `reader.read_line(&mut line)`
  - reads a **line of text** and appends it to `line`, which is a `mut String`;
  - the **newline characters**, like `\n`, at the end of the line are **included** in `line`;
  - returns `Result<u64, io::Error>`:
    - **on success**, returns `Ok(n)`, where `n` is the **number of bytes** read;
      - `Ok(0)` means there is **no more input to read**;
    - **on error**, returns `Err(err)`, where `err` is an `io::Error` value;
      - an `io::Error` has a `.kind()` method that returns an **error code** of type `io::ErrorKind`;
- `reader.lines()`
  - returns an **iterator** over the lines of the input;
  - the item type is `io::Result<String>`;
  - the **newline characters** are **not included** in the iterator's items;
- `reader.read_until(stop_byte, &mut byte_vec)`
  - `stop_byte` is the **delimiter**;
  - this like `reader.read_line(&mut line)`, but **byte-oriented** and produces `Vec<u8>`;
- `reader.split(stop_byte)`
  - `stop_byte` is the **delimiter**;
  - this like `reader.lines()`, but **byte-oriented** and produces `Vec<u8>`;

<br>

`BufRead` also provides a pair of **low-level methods**, for direct access to the reader’s internal buﬀer:
- `reader.consume(n)`
- `reader.fill_buf()`

<br>

## Writers
The `write!()` and `writeln!()` are called **write macros**.<br>
To send output to a writer, use the `write!()` and `writeln!()` macros.<br>
The **write macros** each take a **writer** and they return a `Result`, so errors must be handled:
```rust
writeln!(io::stderr(), "error: world not helloable")?;
writeln!(&mut byte_vec, "The greatest common divisor of {:?} is {}", numbers, d)?;
```

The **print macros** **don’t** return a `Result`; they simply **panic** if the **write fails**.<br>

The Write trait has these methods:
- `writer.write(&buffer)`
  - type of the `buffer` is `[u8]`;
  - this writes up to `buffer.len()` bytes from the slice `buffer` to the underlying stream;
  - returns `Result<u64, io::Error>`:
    - **on success**, returns `Ok(n)`, where `n` is the **number of bytes** written, which **may be equal to or less than** `buffer.len()`;
      - `Ok(0)` means there is **no more input to read**;
    - **on error**, returns `Err(err)`, where `err` is an `io::Error` value;
      - an `io::Error` has a `.kind()` method that returns an **error code** of type `io::ErrorKind`;
- `writer.write_all(&buffer)`
  - writes **all the bytes** from the slice `buffer` to the underlying stream;
  - returns `Result<(), io::Error>`;
- `writer.flush()`
  - **flushes** any buﬀered data to the underlying stream;
  - returns `Result<(), io::Error>`;

<br>

**Note** that while the `println!` and `eprintln!` macros **automatically flush** the `stdout` and `stderr` stream, the `print!` and `eprint!` macros **do not**. You have to call `flush()` manually when using them.<br>

Just as `BufReader::new(reader)` **adds a buﬀer to any reader**, `BufWriter::new(writer)` **adds a buﬀer to any writer**:
```rust
let file = File::create("tmp.txt")?;
let writer = BufWriter::new(file);
```

To set the **size of the buﬀer**, use `BufWriter::with_capacity(size, writer)`.<br>

When a `BufWriter` is dropped, all remaining buﬀered data is written to the underlying writer. However, if an **error** **occurs** during this write, the **error** is **ignored** . Since this happens inside `BufWriter`’s `.drop()` method, there is no useful place to report the error. To make sure your application notices all output errors, **manually** `.flush()` buﬀered writers **before dropping** them.<br>

<br>

## Files
Two ways to open a file:
- `std::fs::File::open(filename)`
  - **opens** an **existing file** for reading;
  - it returns an `io::Result<File>`, and it’s an **error** if the file `filename` **doesn’t exist**;
- `std::fs::File::create(filename)`
  - **creates** a **new file** for writing;
  - if a file **exists** with the given `filename`, it is **truncated**;

<br>

Once a `File` has been opened, it **behaves like** any other **reader** or **writer**. You can **add** a **buﬀer** if needed.<br>

The `OpenOptions`:
```rust
use std::io::prelude::*;
use std::fs::OpenOptions;


fn main() -> std::io::Result<()> {
    let foo = OpenOptions::new()
    .append(true)
    .open("foo.txt")?;

    let bar = OpenOptions::new()
    .write(true)
    .create_new(true) .open("bar.txt")?;

    Ok(())
}
```

**Methods**:
- `append(true)`
- `create_new(true)` **creates** a file if it **doesn't exist**, **failing** otherwise;
- `create(true)` **creates** a file if it does not exist, and **truncates** it if it does;
- `read(true)`
- `truncate(true)` deletes the contents
- `write(true)`

<br>

You can **access** this `OpenOptions` struct **directly** from `File` through a method called `options()`:
```rust
use std::fs::{write, File};


fn main() -> std::io::Result<()> {
    let foo = File::options()
    .write(true)
    .create_new(true)
    .open("foo.txt")?;

    Ok(())
}
```

<br>

## Seeking
`File` also implements the `Seek` trait:
```rust
pub trait Seek {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
}

pub enum SeekFrom {
  Start(u64),
  End(i64),
  Current(i64)
}
```

- use `file.seek(SeekFrom::Start(0))` to set to the beginning;
- use `file.seek(SeekFrom::Current(-n))` to go back `n` bytes, and so on;

<br>

## `std::fs::FileType`
The `FileType` has `.is_file()`, `.is_dir()`, and `.is_symlink()` methods.<br>

<br>

## `std::fs::DirEntry`
The `read_dir(path)` returns items of `std::fs::DirEntry` type, and it’s a struct with just a few methods:
- `entry.file_name()`
  - the `OsString` with name of the `entry` (**file** or **directory**) inside opened directory;
- `entry.path()`
  -  the same, but with the original path **joined** to it, producing a new `PathBuf`;
- `entry.file_type()`
  - returns an `io::Result<FileType>`
- `entry.metadata()`
  - returns the metadata about this `entry`;

<br>

## Platform-Specific features
There is **no portable way** to create symbolic links that work on **both** **Unix** and **Windows**, but the standard library oﬀers a **Unix-specific** symlink function:
```rust
#[cfg(unix)]
use std::os::unix::fs::symlink;
```

<br>

# Using files
Files take **bytes**.<br>

Functions and methods:
- the `.read_to_string(data)` reads the contents of a whole file into a `String`;
- the `.write_all("some string")` on the `fs::File` **requires** the `b` in front of **string**;
- the `std::fs::write(path, contents)` lets you write a `&str` **without** `b` in front because `write()` takes anything that implements `AsRef<[u8]>` and `str` implements `AsRef<[u8]>`:
```rust
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()>
```


**Example**:
```rust
use std::io::Write;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file: std::fs::File = std::fs::File::create("myfilename1.txt")?;
    file.write_all(b"Foo Bar")?;

    let r = std::fs::write("myfilename2.txt", "Foo Bar")?;
    
    // Opens the file
    let mut my_file = std::fs::File::open("myfilename1.txt")?;
    let mut data = String::new();
    my_file.read_to_string(&mut data)?;

    println!("{}", data);
    Ok(())
}
```

**Output**:
```bash
Foo Bar
```

<br>

# `std::ffi::OsStr`, `std::ffi::OsString`, `std::path::Path` and `std::path::PathBuf`
Inconveniently, your operating system does not force filenames to be valid Unicode.<br>
This is why Rust has:
- `std::ffi::OsStr`
  - the `OsStr` is a string type that’s a superset of UTF-8;
  - its job is to be able to represent all filenames, command-line arguments, and environment variables on the current system, whether they’re valid Unicode or not;
- `std::ffi::OsString` owns a **heap-allocated** `OsStr`;
  - `.to_os_string()`: **converts** `OsStr` **to** `OsString`;
- `std::path::Path`
  - it is exactly like `OsStr`, but it adds many handy filename-related methods;
- `std::path::PathBuf` owns a **heap-allocated** `Path`;
  - `.to_path_buf()`: **converts** `Path` **to** `PathBuf`;

<br>

All three of these types `str`, `OsStr` and `Path` implement `AsRef<Path>`, so we can easily declare a generic function that accepts "any filename type" as an argument:
```rust
use std::path::Path;

fn foo<P>(path: P)
where P: AsRef<Path>
{
    let path = path.as_ref();
}
```

Use `Path` for both absolute and relative paths.<br>

<br>

Some of the functions in `std::fs` and their approximate equivalents on **POSIX**:
|`std::fs` function|**POSIX**|
|:-----------------|:---|
|`create_dir(path)`|`mkdir`|
|`create_dir_all(path)`|`mkdir -p`|
|`remove_dir(path)`|`rmdir`|
|`remove_dir_all(path)`|`rm -r`|
|`remove_file(path)`|`unlink`|
|`copy(src_path, dest_path) -> Result<u64>`|`cp -p`|
|`rename(src_path, dest_path)`|`rename`|
|`hard_link(src_path, dest_path)`|`link`|
|`canonicalize(path) -> Result<PathBuf>`|`realpath`|
|`metadata(path) -> Result<Metadata>`|`stat`|
|`symlink_metadata(path) -> Result<Metadata>`|`lstat`|
|`read_dir(path) -> Result<ReadDir>`|`opendir`|
|`read_link(path) -> Result<PathBuf>`|`readlink`|
|`set_permissions(path, perm)`|`chmod`|

<br>

The special directories `.` and `..` are **not** listed when reading a directory.<br>

As a convenience, the `Path` type has a few of these built in as methods:
- `path.metadata()`;
- `path.read_dir()`;

<br>

The `OsString` has an interesting method called `.into_string()` that **tries** to make it into a regular `String`. It **returns** a `Result`, but the `Err` part is just the **original** `OsString`:
```rust
pub fn into_string(self) -> Result<String, OsString>
```

So if it **doesn’t work**, you just get the previous `OsString` back. You **can’t call** `.unwrap()` because it **will panic**, but you **can use** `match` to get the `OsString` back:
```rust
use std::ffi::OsString;

fn osstr(s: OsString) {}
fn string(s: String) {}

fn main() {
  let os_string = OsString::from("This string works for your OS too.");
  match os_string.into_string() {
    Ok(valid) => {
      println!("String: {:?}", valid);
      string(valid)
    },
    Err(not_valid) => {
      println!("OsString: {:?}", not_valid);
      osstr(not_valid);
    }
  }
}
```
