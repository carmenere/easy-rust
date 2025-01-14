# Table of contents
- [Table of contents](#table-of-contents)
- [Object files](#object-files)
	- [ELF](#elf)
	- [ELF Header](#elf-header)
	- [ELF Sections](#elf-sections)
	- [ELF Segments](#elf-segments)
- [Linking](#linking)
- [PIC and lazy linking](#pic-and-lazy-linking)
- [Relocation entries](#relocation-entries)

<br>

# Object files
Types of object files:
- **executable** binary;
- **shared library** `*.so`;
- **linkable** object file `*.o`;
- **staic libraries** `*.a` (collections of **linkable** object file);
- **vDSO** (virtual dynamic shared object), virtual ELF dynamic shared object;
  -  the **vDSO** is a small shared library that the kernel automatically maps into the address space of all user-space applications, it has **no** real file in filesystem;

<br>

**Shared libraries** have many names - **shared libraries**, **shared objects**, **dynamic shared objects**, **dynamically linked libraries** (**DLL**).<br>

<br>

## ELF
**ELF** stands for **Executable and Linkable Format**. Object files participate in both program **linking** and program **execution**. So, ELF files are composed of **sections** and **segments**.<br>

Generally speaking, ELF files are composed of 3 major components:
- **ELF Header**;
- **Sections** (used by linker);
- **Segments** (aka **program headers**) (used by loader);

<br>

## ELF Header
The **ELF header** is denoted by an `Elfxx_Ehdr` structure. Mainly, this contains general information about the binary.<br>

A common tool to quickly parse ELF files is the `readelf` utility from GNU binutils: `readelf -h <executable>`.<br>

<br>

## ELF Sections
**Sections** represent the smallest **indivisible units** that can be processed within an ELF file.<br>
**Sections** are used by linker at compile time.<br>
A **section header table** contains information about all sections in object file.<br>
**Every section** has an **entry** in the table. Each entry gives information such as the section **name**, the section **size**, and so forth.<br>

<br>

## ELF Segments
**Segments** are a **collection** of **sections**. The OS creates the memory layout from the file’s **segments** and passes control to the file’s **entry point**.<br>
A **segment header table**, if present, tells the system how to create a **process image**.<br>
**Sections** are used by `exec(2)` or by the **dynamic linker** at loading time.<br>

<br>

# Linking
There are two linking types:
- **static linking**: completed at the end of the **compilation process**;
- **dynamic linking**: completed at **load time** by the system;


For statically linked programs, execution is straight forward.
The dynamic linking process begins immediately after execution.


With dynamically linked programs, the system executes the **dynamic linker**, which should set up the environment and only then execute the main binary.

When the linker creates a **shared library**, it **doesn't** know in advance where it might be loaded. Shared libraries are loaded into **non-deterministic** addresses and **absolute** addresses used inside them **must** be **patched**.<br>

There are two main approaches to solve this problem in Linux ELF shared libraries:
- **Load-time relocation**;
- **Position Independent Code** (**PIC**);

<br>

**Load-time relocation** has a couple of problems:
  - it takes time to perform, because **dynamic linker** performs **all** relocations **before** calling `_start` entrypoint;
  - it makes the `.text` section of the library **non-shareable**;

<br>

**PIC** is the more common and nowadays-recommended solution.

<br>

# PIC and lazy linking
**PIC** is only applied to **shared library**.<br>
The idea behind **PIC** is simple - add an additional level of indirection to all global data and function references in the code.<br>

This is achieved by the **Procedure Linkage Table** (**PLT**) and the **Global Offset Table** (**GOT**).<br>

The **GOT** holds entries of addresses of **global variables** and **external functions**.<br>
The **PLT** consists of short entries of instructions (aka **trampolines**), used to reach external functions by redirecting control flow of execution to its corresponding **GOT** entry.<br>

There are 2 approaches of relocating **GOT** entries:
- **early binding**, **GOT** entries are relocated by the dynamic linker at **load-time**;
- **lazy linking**, the **GOT** entries will be relocated on-demand **when they are called**;
  - the GOT is populated dynamically as the program is running;
  - the first time a shared function is called, the **GOT** contains a pointer back to the **PLT**, where the **dynamic linker** is called to find the actual location of the function;
  - when the location found it is then written to the **GOT**;
  - the second time a function is called, the **GOT** contains the known location of the function;

**Lazy linking** can cause to security issues.<br>

<br>

# Relocation entries
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

The `r_offset` field gives the **location** at which to apply the **relocation action**.<br>

The only difference between both structures is that `Elfxx_Rela` contains additional field **relocation addend** used to compute the value to be stored into the relocatable field.
