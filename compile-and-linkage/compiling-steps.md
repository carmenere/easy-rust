# Table of contents
- [Table of contents](#table-of-contents)
- [Object files](#object-files)
- [Compilation stages](#compilation-stages)
- [Linking](#linking)
- [Libraries](#libraries)
  - [Static libraries](#static-libraries)
  - [Shared libraries](#shared-libraries)
- [`dl` library](#dl-library)
- [GCC search paths](#gcc-search-paths)
  - [Libraries search path](#libraries-search-path)
  - [Headers search path](#headers-search-path)
- [PIC and lazy linking](#pic-and-lazy-linking)

<br>

# Object files
Types of object files:
- **linkable** object file `*.o`;
- **executable** binary;
- **shared library** `*.so`;
- **staic libraries** `*.a` (collections of **linkable** object file);
- **vDSO** (virtual dynamic shared object), virtual ELF dynamic shared object;
  -  the **vDSO** is a small shared library that the kernel automatically maps into the address space of process;
  -  it has **no** corresponding real `*.so` or `*.a` file;

<br>

# Compilation stages
The `hello.c` file that will be used futher:
```c
#include <stdio.h>
void foo()
{
        (void) printf("initializing: foo()\n");
}

void bar()
{
        (void) printf("finalizing: bar()\n");
}

int main()
{
    printf("Hello, World!\n");
    return 0;
}
```

<br>

The **compilation process** consists of **4 stages**:<br>
|Stage|Command|Component|
|:----|:------|:--------|
|**Preprocessing**|- `gcc -E -o hello.i hello.c` from source code `hello.c`<br>- `gcc -E hello.c` this command outputs its result to stdout|**preprocessor**|
|**Compilation**|- `gcc -S hello.i -o hello.s` from intermediate `.i` file<br>- `gcc -S hello.c` *all in one* command from source code `hello.c`|**compiler**|
|**Assembling**|- `gcc -c hello.s -o hello.o` from intermediate `.s` file<br>- `gcc -c hello.c` *all in one* command from source code `hello.c`|**assembler**|
|**Linking**|- `gcc -o hello hello.o` from intermediate `.o` file<br>- `gcc -o hello hello.c` *all in one* command from source code `hello.c`|**linker**|

<br>

There is **option** `-o` to specify the output file name **explicitly**. By default `gcc` uses `a.out` for **executable** file.<br>

<br>

- **preprocessing**
  - in this stage, the **preprocessor** expands:
    – all **header files** (`#include`);
    – all **macros** (`#define`);
    - all **inline functions**;
- **compilation**
  - in this stage, the **compiler** translates the preprocessed code to **assembly instructions** specific to the **target** processor architecture;
- **assembling**
  - in this stage, the **assembler** generates **relocatable object file** (`.o` file);
- **linking**
  - in this stage, the **linker** generates **executable file** or **shared library**;

<br>

# Linking
There are 3 type of binding libraries to binary:
- **static linking**;
- **dynamic linking** at **loading time** (**by default**);
- **dynamic linking** at **runtime** (`dl` library is needed);

<br>

In **static linking**, the size of the executable becomes greater than in dynamic linking, as the library code is stored within the executable rather than in separate files.
**Dynamically linked programs** require **dynamic linker** to be loaded.<br>

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

# Libraries
There are 2 kind of libraries:
- **static** libraries (aka **statically linked** libraries, `.a` files);
- **shared** libraries (aka **dynamically linked** libraries, `.so` files);

**Shared libraries** have many names - **shared objects**, **dynamic shared objects**, **dynamically linked libraries** (**DLL**).<br>

<br>

## Static libraries
**Static library** () is a **set** of object files that are copied into a target application by a linker producing a **stand-alone executable**.<br>

<br>

To create **static library** with name `NAME` there is command `ar`.
**Example**, `ar rc libNAME.a [list of object files]`:
- option `c` stands for **create** if not exists;
- option `r` stands for **replace** old by newer version;

<br>

**Example**:
```bash
ar rc libfoo.a f1.o f2.o
```

Produces file `libfoo.a` which will contain `f1.o` and `f2.o` files. To get completed **static library** we need add **index** (list of all functions and vars that are in library) to `libfoo.a` and there is `ranlib` command for this: `ranlib libNAME.a`.

<br>

## Shared libraries
**Ordinary** *object files* are **not** suitable for *shared libraries*.<br>
To create *object files* **suitable** for *shared libraries* there is option `-fPIC` in `gcc`, this option enables **PIC**.<br>

<br>

**Example**:
```bash
gcc -fPIC -c f1.c
gcc -fPIC -c f2.c
```

<br>

To create **shared library** there is option `-shared` in `gcc`:
```bash
gcc -shared -o libfoo.so f1.o f2.o
```

<br>

# `dl` library
**Shared libraries** can be loaded **not** only loading time, but also **during runtime**. Special `dl` library provides functions `dlopen()`, `dlsym()` and `dlclose()` for doing this.

<br>

**Example**:
```c
#include <dlfcn.h>
library_handler = dlopen("/path/to/the/library.so", RTLD_LAZY);
if (!library_handler){
    fprintf(stderr,"dlopen() error: %s\n", dlerror());
    exit(1);
};
```

<br>

`dlsym` returns address of appropriate **symbol** (function ot variable) inside loaded library.<br>

<br>

# GCC search paths
## Libraries search path
Option `-L <dir>` (`-L<dir>`) adds directory `<dir>` to the **libraries search path**.<br>
Option `-l <library>` search the library named `<library>` when linking in **libraries search path**.<br>

Examples:
```bash
gcc -c main.c
gcc main.o -L. -lfoo -o rezult
```

<br>

```bash
gcc main.c -L. -lfoo -o rezult
```

<br>

## Headers search path
Option `-I <dir>` (`-I<dir>`) adds the directory `<dir>` to the list of directories to be searched for **header files**.<br>

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


![elf_got_code_data_offset](/img/elf_got_code_data_offset.png)
![elf_got_code_data](/img/elf_got_code_data.png)
![elf_plt_before](/img/elf_plt_before.png)
![elf_plt_after](/img/elf_plt_after.png)

<br>

GOT – это просто таблица с адресами, которая находится в секции data.
Предположим, что какая-то инструкция в секции code хочет обратиться к переменной. Вместо того, чтобы обратится к ней через абсолютный адрес (который потребует релокации), она обращается к записи в GOT. Поскольку GOT имеет строго определённое место в секции data, и линкер знает о нём, это обращение тоже является относительным. А запись в GOT уже содержит абсолютный адрес переменной:
Чтобы каждый раз подгружать абсолютно все функции в GOT, мы используем процедуру “ленивого связывания”, т.е. связывание функции из библиотеки с программой происходит при непосредственном вызове её. Помогает в этом механизм Procedure Linkage Table (PLT).

PLT – это часть секции text в бинарнике, состоящая из набора элементов (один элемент на одну внешнюю функцию, которую вызывает библиотека). Каждый элемент в PLT – это небольшой кусок выполняемого машинного кода. Вместо вызова функции напрямую вызывается кусок кода из PLT, который уже сам вызывает функцию. Такой подход часто называют «трамплином». Каждый элемент из PLT имеет собственный элемент в GOT, который содержит реальное смещение для функции (конечно после того как загрузчик определит её).

В коде вызывается функция func. Компилятор переводит этот вызов в вызов func@plt, который является одним из элементов PLT. После этого идет обращение в GOT, и с учетом того что функция вызывалась в первый раз – управление передаётся обратно PLT, где т. н. resolver устанавливает связь между названием функции и её кодом из библиотеки. После такого первого связывания схема будет выглядеть немного по-другому:

Библиотека при этом абсолютно не зависит от адреса, по которому она будет загружена: ведь единственное место, где используется абсолютный адрес – это GOT, а она находится в секции data и будет релоцирована загрузчиком во время загрузки. Даже PLT не зависит от адреса загрузки, так что она может находиться в секции text, доступной только для чтения.



<br>

The Global Offset Table, or GOT, is a section of a computer program's (executables and shared libraries) memory used to enable computer program code compiled as an ELF file to run correctly, independent of the memory address where the program's code or data is loaded at runtime.[1]

It maps symbols in programming code to their corresponding absolute memory addresses to facilitate Position Independent Code (PIC) and Position Independent Executables (PIE)[2] which are loaded[3] to a different memory address each time the program is started. The runtime memory address, also known as absolute memory address of variables and functions is unknown before the program is started when PIC or PIE code is run[4] so cannot be hardcoded during compilation by a compiler.

The Global Offset Table is represented as the .got and .got.plt sections in an ELF file[5] which are loaded into the program's memory at startup.[5][6] The operating system's dynamic linker updates the global offset table relocations (symbol to absolute memory addresses) at program startup or as symbols are accessed.[7] It is the mechanism that allows shared libraries (.so) to be relocated to a different memory address at startup and avoid memory address conflicts with the main program or other shared libraries, and to harden computer program code from exploitation.[8]

<br>

Well, there’s two types of binaries on any system: statically linked and dynamically linked. Statically linked binaries are self-contained, containing all of the code necessary for them to run within the single file, and do not depend on any external libraries. Dynamically linked binaries (which are the default when you run gcc and most other compilers) do not include a lot of functions, but rely on system libraries to provide a portion of the functionality. For example, when your binary uses printf to print some data, the actual implementation of printf is part of the system C library. Typically, on current GNU/Linux systems, this is provided by libc.so.6, which is the name of the current GNU Libc library.

In order to locate these functions, your program needs to know the address of printf to call it. While this could be written into the raw binary at compile time, there’s some problems with that strategy:

Each time the library changes, the addresses of the functions within the library change, when libc is upgraded, you’d need to rebuild every binary on your system. While this might appeal to Gentoo users, the rest of us would find it an upgrade challenge to replace every binary every time libc received an update.
Modern systems using ASLR (address space layout randomization feature ) load libraries at different locations on each program invocation. Hardcoding addresses would render this impossible.
Consequently, a strategy was developed to allow looking up all of these addresses when the program was run and providing a mechanism to call these functions from libraries. This is known as relocation, and the hard work of doing this at runtime is performed by the linker, aka ld-linux.so. (Note that every dynamically linked program will be linked against the linker, this is actually set in a special ELF section called .interp.) The linker is actually run before any code from your program or libc, but this is completely abstracted from the user by the Linux kernel.

<br>

The difference between .plt and .plt.got is that .plt uses lazy binding and .plt.got uses non-lazy binding.

The difference is that .got.plt is runtime-writable, while .got is not if you enable a defense against GOT overwriting attacks called RELRO (relocations read-only). To enable RELRO, you use the ld option -z relro. RELRO places GOT entries that must be runtime-writable for lazy binding in .got.plt, and all others in the read-only .got section

ELF binaries often contain a separate GOT section called .got.plt for use in conjunction with .plt in the lazy binding process


Relocations
Looking at an ELF file, you will discover that it has a number of sections, and it turns out that relocations require several of these sections. I’ll start by defining the sections, then discuss how they’re used in practice.

.got
This is the GOT, or Global Offset Table. This is the actual table of offsets as filled in by the linker for external symbols.
.plt
This is the PLT, or Procedure Linkage Table. These are stubs that look up the addresses in the .got.plt section, and either jump to the right address, or trigger the code in the linker to look up the address. (If the address has not been filled in to .got.plt yet.)
.got.plt
This is the GOT for the PLT. It contains the target addresses (after they have been looked up) or an address back in the .plt to trigger the lookup. Classically, this data was part of the .got section.
.plt.got
It seems like they wanted every combination of PLT and GOT! This just seems to contain code to jump to the first entry of the .got. I’m not actually sure what uses this. (If you know, please reach out and let me know! In testing a couple of programs, this code is not hit, but maybe there’s some obscure case for this.)

<br>

Mitigations
So, since this exploit technique has been known for so long, surely someone has done something about it, right? Well, it turns out yes, there’s been a mitigation since 2004. Enter relocations read-only, or RELRO. It in fact has two levels of protection: partial and full RELRO.

Partial RELRO (enabled with -Wl,-z,relro):

Maps the .got section as read-only (but not .got.plt)
Rearranges sections to reduce the likelihood of global variables overflowing into control structures.
Full RELRO (enabled with -Wl,-z,relro,-z,now):

Does the steps of Partial RELRO, plus:
Causes the linker to resolve all symbols at link time (before starting execution) and then remove write permissions from .got.
.got.plt is merged into .got with full RELRO, so you won’t see this section name.
Only full RELRO protects against overwriting function pointers in .got.plt. It works by causing the linker to immediately look up every symbol in the PLT and update the addresses, then mprotect the page to no longer be writable.

<br>
