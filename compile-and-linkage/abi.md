# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [ABI](#abi)
  - [Calling convention](#calling-convention)
- [Memory layout of process](#memory-layout-of-process)
<!-- TOC -->

<br>

# ABI
The **System V ABI** (**Application Binary Interface**) is a set of specifications that defines
- calling conventions;
- object file formats;
- memory layout;
- dynamic linking semantics;
- system calls interface:

<br>

## Calling convention
The **calling convention** specifies:
- how **arguments** are passing to function;
- how function **returns** its result;
- how **registers** are used;
- how **stack** is set up;

<br>

The **C calling convention** (aka **C declaration**, **cdecl**) is the most common one.<br>

<br>

# Memory layout of process
The terms **section** and **segment** refer to different views of an **object file** (**ELF file**):
- the **section** is used by compiler for **linking**;
- the **segment** corresponds to **memory layout** of a loaded programm and it is used by the OS loader;

The standard **sections** used by compilers, including `rustc`, are:
- `.rodata` (read-only data):
  - **zero** or **non-zero** *initialized* **immutable** `static`;
  - `&'static str` also reside in this section;
- `.data` (initialized data):
  - **non-zero** *initialized* **mutable** `static`;
- `.bss` (block start by symbol):
  - **zero** *initialized* **mutable** `static`;
  - **uninitialized mutable** `static`;

The `.bss` section takes up **no space** in the binary file on disk. The operating system loader simply allocates the required memory space and fills it by zeros **before** the program starts.<br>

<br>

**Example**:
```rust
// .rodata
static NON_ZERO_INIT_IMMUTABLE: u128 = 1000;

// .rodata
static ZERO_INIT_IMMUTABLE: u128 = 0;

// .data
static mut NON_ZERO_INIT_MUTABLE: u128 = 1000;

// .bss
static mut ZERO_INIT_MUTABLE: u128 = 0;

// .bss
static UNINIT_IMMUTABLE: std::sync::OnceLock<[u128; 10]> = std::sync::OnceLock::new();

// .bss
static mut UNINIT_MUTABLE: std::sync::OnceLock<[u128; 10]> = std::sync::OnceLock::new();

fn main() {
    println!("NON_ZERO_INIT_IMMUTABLE = {:?}", NON_ZERO_INIT_IMMUTABLE);
    println!("ZERO_INIT_IMMUTABLE = {:?}", ZERO_INIT_IMMUTABLE);
    println!("UNINIT_IMMUTABLE = {:?}", UNINIT_IMMUTABLE);

    unsafe {
        println!("NON_ZERO_INIT_MUTABLE = {:?}", NON_ZERO_INIT_MUTABLE);
        println!("ZERO_INIT_MUTABLE = {:?}", ZERO_INIT_MUTABLE);
        println!("UNINIT_MUTABLE = {:?}", UNINIT_MUTABLE); 
    }
}
```

<br>

**Segments** of process in 32-arch:
```c
Higher Addresses
|=================| <- 0xFFFFFFFF
|                 |
|      Kernel     |
|                 |
|=================| <- 0xC0000000
|=================| <- 0xBFFFFFFF Stack (grows downwards) 
|                 | cli args and env vars at the bottom of stack
|      Stack      |
|                 |
|-----------------| <- Top of the stack
|                 |
|                 |
|=================| <-- 0x40000000 For shared libraries and mmap()
|                 |
|                 |
|-----------------| <-- Program break
|                 |
|      Heap       |
|                 |
|=================| <- Heap (grows upwards)
|                 |
|       BSS       | Initializeed to 0 by exec()
|                 | Zero-initialized data or uninitialized data occupies no space in binary file
|=================|
|                 |
|   Initialized   | Read from binary file by exec()
|      data       |
|                 |    
|=================|
|                 |
|      Text       | Read from binary file by exec()
|                 |
|=================| <- 0x08048000
|                 |
|=================| <- 0x00000000
Lower Addresses
```

<br>

**Segments**:
1. **Text segment** (aka **Code segment**). It contains instructions for CPU.
2. **Data segment**, it is divided into **two** parts:
- **initialized** data, it contains **global** and **static** variables that are **statically initialized**. It's **not** a read-only segment and hence the values can be modified **at runtime**.
- **uninitialized** data (aka **BSS** section), data in this segment are initialized by the kernel to **0** before the program starts execution;
    - **uninitialized** data occupies no space in binary file;
1. **Heap segment**.
2. **Stack segment**.
3. **Kernel segment**. It is mapped into process kernel memory, but **not** accessible to program, it contains process specific data (**pages**, ...).

<br>

Kernel segment is **kernel space** (1 GB). All other segments are **user space** (3 GB).<br>

<br>

**Program break** points to the address of **heap end**. When `malloc()` is called, it increases the **program break**.<br>

<br>