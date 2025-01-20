# Table of contents
- [Table of contents](#table-of-contents)
- [ABI](#abi)
  - [Calling convention](#calling-convention)
- [Memory layout of process](#memory-layout-of-process)
- [ELF](#elf)
  - [ELF Header](#elf-header)
  - [ELF sections](#elf-sections)
  - [Symbols](#symbols)
    - [Relocation entries](#relocation-entries)
  - [ELF segments](#elf-segments)

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

# ELF
The **ELF** (Executable and Linkable Format) is a common standard file format for **object files**: **executable files**, **libraries**, **core dumps**.<br>
The **ELF** is specified in the **System V ABI**. File `/usr/include/elf.h` defines standard ELF types, structures, and macros.<br>

The **ELF** files are composed of 3 major components:
- **ELF header**;
- **section header table** (used by linker);
- **segment header table** (aka **program header table**) (used by loader);

<br>

**Object files** participate in both **linking** and **execution**. The ELF file provides **two separate views** on the data inside the ELF file:
- **section view** for linker;
- **segment view** for loader;

<br>

Neither the **section header table** nor the **segment header table** have fixed positions, they can be located anywhere in an ELF file. 
To find them the ELF header is used, which is located at the very start of the file.

![elf_views](/img/elf_views.png)

<br>

## ELF Header
The **ELF header** is denoted by an `Elfxx_Ehdr` structure. Mainly, this contains general information about the binary.<br>

<br>

## ELF sections
**Sections** represent the smallest **indivisible units** that can be processed within an **ELF** file.<br>
**Sections** are used by linker at **compile time**.<br>
A **section header table** contains information about all sections in object file.<br>
**Every section** has an **entry** in the table. Each entry gives information such as the section **name**, the section **size**, and so forth.<br>

<br>

|Section|Meaning|
|:------|:------|
|`.interp`|This section contains the **full path** to the appropriate **dynamic linker**, in other words every **dynamically linked** binary is linked against the **dynamic linker**.|
|`.text`|This section holds the executable instructions for CPU.|
|`.rodata`|This section holds **read-only data**, i.e. **constants** with values provided *at compile time*, like **hardcoded string**.|
|`.data`|This section holds **initialized** data, i.e. **global variables** with values provided *at compile time*.|
|`.bss`|For **uninitialized** data, i.e. **uninitialized global variables**. The section occupies no file space.|
|`.note.ABI-tag`|This section specifies ABI details.|
|`.rela.*`|**Relocation** sections.|
|`.dynamic`|This section holds dynamic linking information.|
|`.dynsym`|This section holds the **symbol table** needed for **dynamic linking**.|
|`.dynstr`|This section holds **string table** needed for `.dynsym`.|
|`.symtab`|Contains a **symbol table**.|
|`.strtab`|Contains the **string table** for `.symtab`.|
|`.debug_info`|Contains information generated by compilers to describe the source code while debugging by keeping symbols and its type, scope, file, line number, etc.|
|`.debug_*`|Additional sections for debug info.|

<br>

## Symbols
A **symbol table** of a program is a list containing **all** the program's **symbols** (function names, variables names, etc).<br>
The **debug symbols** is a special kind of symbols that attaches additional information needed for debugging.<br>
**DWARF** is a widely used, standardized debugging data format. **Version 5** of the **DWARF** format was published in February **2017**.<br>
**DWARF** uses a data structure called a **Debugging Information Entry** (**DIE**) to represent each variable, type, procedure, etc.<br>
In ELF file **DWARF** has been divided into different sections like `.debug_info`, `.debug_frame`, etc.

<br>

### Relocation entries
**Relocation information** is held in **relocatable entries**, located in specific **relocation sections** within an ELF object.<br>
There are **two** different **relocation entry structures**: `Elfxx_Rel` and `Elfxx_Rela`.<br>
It is important to note that these relocation entry types are **mutually exclusive**. The reason for using one type of entry over the other, is usually architecture dependant. For example, in `x86` only `Elf32_Rel` is used, while on `x86_64` only `Elf64_Rela` is used.<br>

```c
typedef struct {
  Elf32_Addr r_offset;
  Elf32_Word r_info;
} Elf32_Rel;

typedef struct {
  Elf64_Addr   r_offset;
  Elf64_Xword  r_info;
  Elf64_Sxword r_addend;
} Elf64_Rela;
```

<br>

The only **difference** between both structures is that `Elfxx_Rela` contains additional field **relocation addend** used to compute the value to be stored into the relocatable field.

<br>

**Fields**:
- `r_offset` field contains the **offset** to **place** where **real address** must be **injected**, this is **reserved** and filled by zero.<br>
- `r_info` defines **type of addres** and **symbol table index** that defines symbol for which relocation is performed;
- `r_addend` specifies a **constant addend** used to compute the value to be stored into the relocatable field;

**Type of addres**:
-	`R_386_32`
-	`R_386_PC32`

<br>

## ELF segments
A **segment** ((aka **program header**)) comprises one or more **sections**, in other words, a **segment** is a **set** of **sections**.<br>
**Segments** are used by `exec(2)` or by the **dynamic linker** at loading time.<br>
A **segment header table** (aka **program header table**) contains information about all segments in object file.<br>
**Every segment** has an **entry** in the table. Each entry defines **access permissions** for segment and other info.<br>
