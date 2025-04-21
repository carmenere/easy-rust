# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [ABI](#abi)
  * [Calling convention](#calling-convention)
* [Memory layout of process](#memory-layout-of-process)
<!-- TOC -->

<br>

# ABI
The **System V ABI** (**Application Binary Interface**) is a set of specifications that defines
- calling conventions;
- object file formats;
- dynamic linking semantics;
- memory layout
- and much more;

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
Layout for 32-bit arch:
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
|      Text       |
|                 |
|=================| <- 0x08048000
|                 |
|                 |
|=================| <- 0x00000000
Lower Addresses
```

<br>

Segments:
1. **Text segment** (aka **Code segment**). It contains instructions for CPU.
2. **Data segment**, it is divided into **two** parts:
- **initialized** data, it contains global and static variables that are initialized. Its not a read-only segment and hence the values can be modified;
- **uninitialized** data (aka **BSS** segment), data in this segment are initialized by the kernel to **0** before the program starts execution;
    - **uninitialized** data occupies no file space;
1. **Heap segment**.
2. **Stack segment**.
3. **Kernel segment**. It is mapped into process kernel memory, but **not** accessible to program, it contains process specific data (**pages**, ...).

<br>

Kernel segment is **kernel space** (1 GB). All other segments are **user space** (3 GB).<br>

<br>

**Program break** points to the address of **heap end**. When `malloc()` is called, it increases the **program break**.<br>

<br>