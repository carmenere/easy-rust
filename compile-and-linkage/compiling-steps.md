# Table of contents
- [Table of contents](#table-of-contents)
- [Object files](#object-files)
- [Compilation stages](#compilation-stages)
- [Linking](#linking)
  - [Binaries](#binaries)
  - [Libraries](#libraries)
    - [Static libraries](#static-libraries)
    - [Shared libraries](#shared-libraries)
- [`dl` library: dynamic linking at runtime](#dl-library-dynamic-linking-at-runtime)
- [GCC search paths](#gcc-search-paths)
  - [Libraries search path](#libraries-search-path)
  - [Headers search path](#headers-search-path)
- [PIC/PIE](#picpie)
  - [How dynamic linker fills GOT?](#how-dynamic-linker-fills-got)
  - [Mitigations](#mitigations)
- [`.rela.text` and `.rela.data` sections](#relatext-and-reladata-sections)
  - [Linker steps](#linker-steps)
  - [Merging sections](#merging-sections)

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
__attribute__ ((constructor (101))) void foo()
// void foo()
{
        (void) printf("initializing: foo()\n");
}

__attribute__((destructor (101))) void bar()
// void bar()
{
        (void) printf("finalizing: bar()\n");
}

int a[10]={0,1,2,3,4,5,6,7,8,9};
int b[10];

int main(int argc, char* argv[]){
    int i;
    static int k = 3;

    for(i = 0; i < 10; i++) {
        printf("%d\n",a[i]);
        b[i] = k*a[i];
    }
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
There are **3 type of linking** libraries to binary:
- **static linking** at **compile time** as final step;
- **dynamic linking** at **loading time** (**by default**);
- **dynamic linking** at **runtime** (`dl` library is needed);

<br>

When the linker creates a **shared library** or **binary**, it **doesn't** know in advance where it might be loaded. Modern systems use **ASLR** feature (address space layout randomization) and load *shared libraries*/*binaries* at **different** locations on each program invocation. So, *shared libraries*/*binaries* are loaded into **non-deterministic** addresses and **absolute** addresses used inside them **must** be **updated** at load time.<br>

<br>

**PIC** (**Position Independent Code**) and **PIE** (**Position Independent Executables**) solve this problem:
- **PIC** for **shared libraries**;
- **PIE** for **executable**;

<br>

## Binaries
There are **two** types of binaries:
- **statically linked** binaries, they are **self-contained** and do not depend on any external libraries;
- **dynamically linked** binaries, they do not include a lot of functions, but rely on **system libraries** to provide a portion of the functionality;

The size of **statically linked** binary is greater than **dynamically linked** binary, as the library code is stored within the **executable** rather than in separate files.<br>
The **dynamically linked** binaries require **dynamic linker** to be loaded.<br>

<br>

**By default**, **executables** on modern systems are built as **Position Independent Executables** (**PIE**) and all addresses are resolved when the program is loaded into memory.<br>
To **disable** **PIE** for binary there is option `-no-pie`.<br>

<br>


## Libraries
There are **two** kind of libraries:
- **static** libraries (aka **statically linked** libraries, `.a` files);
- **shared** libraries (aka **dynamically linked** libraries, `.so` files);

**Shared libraries** have many names - **shared objects**, **dynamic shared objects**, **dynamically linked libraries** (**DLL**).<br>

<br>

### Static libraries
**Static library** is a **set** of object files that are copied into a target application by a linker producing a **stand-alone executable**.<br>

<br>

To create **static library** with name `NAME` there is command `ar`.<br>
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

### Shared libraries
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

# `dl` library: dynamic linking at runtime
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

# PIC/PIE
When an operating system loads an *executable* or a *shared library* (`.dll` or `.so`), it often places it at a **randomized memory location** for security (a feature called **ASLR**). If the code used absolute (fixed) addresses, the OS would have to patch every single memory-referencing instruction at startup.<br>

**PIE/PIC** code uses **RIP-relative addressing** (aka **PC-relative addressing**) and **GOT-based addressing** instead *absolute addressing*.<br>

To enable **PIE/PIC** there are `-fPIC` and `-fPIE` options for `gcc`.<br>

**Absolute addressing** is when the code refers to specific memory addresses **directly**.<br>
**Relative addressing**, on the other hand, is when the code refers to memory addresses relative to the current program counter (PC).<br>

The **PC** stands for **program counter**. The **PC** is a **special CPU register** that holds the **address** of the **next instruction** to be executed. **PC** and **RIP** refer to the exact same concept, but **PC** is the **generic** name, while **RIP** is the **specific** name in AMD64.<br>
When a program is running, the **PC** is **incremented** after each instruction is executed. This means that the **PC always contains the address of the next instruction** to be executed. This means that all addresses are relative to a known base address.<br>

**RIP-relative addressing** uses **signed 32-bit displacement** *from* **RIP** *to* **accessed value** and allows instructions to **access any data within ±2 GB from** the **current instruction**, which is more than enough for almost all applications.<br>

When **CPU** fetches instruction it will calculate *address* of **value** as `value_addr = RIP + disp32`.<br>
When **linker** relocates address it will calculate *disposal* as `disp32 = S + A − P`, where
- `P` is an address of **next instruction** that will reside in **RIP** at the moment of execution **current instruction**;
  - `P = address_of_current_instruction + length_of_current_instruction`;
- `A` is an **addend** in *relocation entry*;
- `S` is an **address of symbol**;

<br>

With *RIP-relative addressing*, the **distance** **between** the **code instruction** and the **address of symbol** remains **constant**, no matter where the segment is loaded in memory. This allows the code to be relocated to any memory address without having to be modified. So, *RIP-relative addressing* **works for any base address**.<br>

After linker has merged sections of many object file in resulting section the `disp32` **does not change**.<br>

<br>

For example:
- a **current instruction** sits at address `0x1000`;
- the length of this instruction is `7` bytes;
- if **symbol** is located at address `0x1020`
- the compiler will calculate and encode a **displacement** of `0x15`: `0x1020 - 0x1007 = 0x19`;
- while executing it
  - the CPU puts in **RIP** register an address of **next instruction** which is `0x1000 + 7 = 0x1007`;
  - the CPU calculates **address of data** to be accessed as `0x1007 + 0x19 = 0x1020`;

<br>

**GOT-based addressing** is used for **extern symbols** (variables and functions from other object files) and for some **CPU instruction that requires abdsolute addressing** and does not allow *RIP-relative*.<br>

<br>

The **GOT** (**Global Offset Table**) is an array of addresses of **extern symbols** (variables and functions from other object files). A **GOT** is simply a **table of addresses**, residing in the **data** section. The **runtime memory address**, also known as **absolute memory address** of variables and functions is **unknown** before the program is started and cannot be hardcoded during compilation. The **GOT** maps symbols to their corresponding **absolute memory addresses**.<br>

<br>

The **PLT** (**Procedure Linkage Table**) contains special **stub code** - one for each external function that the program calls through **PLT**.<br>

<br>

**GOT**/**PLT** uses *RIP-relative addressing* to access GOT entries. Since the **GOT** is in a **known** place in the data section, the **relative offset** to **GOT entry** is **known** and **fixed**. The **GOT entry**, in turn, will contain the **absolute address** of the variable.<br>

Suppose some instruction in the code section wants to refer to an **extern variable**. Instead of referring to it directly by absolute address, it refers to an entry in the **GOT**:<br>

![elf_got_code_data](/img/elf_got_code_data.png)

<br>

There **2 main scenarios** of using **GOT**:
- **call to an external function**
  - **with** *PLT*
  - **without** *PLT* (option `-fno-plt`);
- **access to external variables**

<br>

**Sections**:
- `.got` - *GOT* for **variables**;
- `.got.plt` - *GOT* for **functions**: `xxxxx@plt` (e.g., `printf@plt`, `malloc@plt`);
- `.plt` contains **small code stubs** - one for each external function that the program calls through **PLT** (e.g., `printf@plt`, `malloc@plt`);
`.rela.dyn` - all **relocation entries** that `ld.so` must process at **loading time**;
`.rela.plt` - **relocation entries** for *PLT*/*GOT* (specifically `R_X86_64_JUMP_SLOT`) that responsible for updating **GOT entries** during **lazy binding**;

<br>

**Compiler** generates following code to perform **GOT-based access** to **external symbols**:
```assembly
mov global_var@GOTPCREL(%rip), %rax   ; read address from GOT entry
mov (%rax), %eax                      ; load value from that address
```

<br>

Variants of **GOT-based access**:
- `name@GOT`: specifies the **offset** to the **GOT entry** for the symbol name from the **base of the GOT**;
- `name@GOTPLT`: specifies the **offset** to the **GOT entry** for the symbol name from the **base of the GOT**, implying that there is a **corresponding PLT entry**;
- `name@GOTOFF`: specifies the **offset** to the location of the symbol name from the **base of the GOT**;
- `name@GOTPCREL`: specifies the **offset** to the **GOT entry** for the symbol name from the **current code location**;
- `name@PLT`: specifies the **offset** to the **PLT entry** of symbol name from the **current code location**;
- `name@PLTOFF`: specifies the **offset** to the **PLT entry** of symbol name from the **base of the GOT**;

<br>

## How dynamic linker fills GOT?
The **dynamic linker** fills `.got` (*GOT* for variables) by addresses of variables at **loading time**.<br>

<br>

The **dynamic linker** can fill `.got.plt` in **2 modes**:
- **lazy binding**
  - **GOT entries** are relocated on-demand **when they are called**, in other words, the **GOT** is populated **dynamically** as the program is running;
- **immediate binding**
  - **all GOT entries** are relocated by the **dynamic linker** at **load-time**;

<br>

How **lazy binding** works:
- **First call to** `xxxxx()`:
  - `GOT[xxxxx]` initially holds the address of the second line (right after the jmp).
  - the `jmp [GOT]` instruction transfers control to the `push + jmp resolver_stub sequence`;
  - the `resolver_stub` in `ld.so`
    - reads the argument from the stack;
    - finds the corresponding relocation (usually of type `R_X86_64_JUMP_SLOT`);
    - computes the real address of the;function in the shared library;
  - it **overwrites** the `GOT[xxxxx]` **entry** with the real function address;
  - then it **calls the function**;
- **Second** and **subsequent calls to** `xxxxx()`:
- now `GOT[xxxxx]` holds the **real function address**;
- the `jmp [GOT]` instruction **jumps directly to the function**, bypassing the resolver;

<br>

Every time symbol `func` is called in code the compiler translates it to a call to `func@plt`, which is some **N-th** entry in the **PLT**. This entry contains jump to `GOT[n]` entry. When the shared library is first loaded, **all** entries in **GOT** point to **resolver routine**.<br>
So, this `GOT[n]` entry points to resolver routine. Every time resolver is involved it performs resolution of the actual address of called symbol `func`, places its **actual address** into `GOT[n]` and calls `func`.<br>

<br>

**Before** the first call to `func` the `GOT[n]` points to to **resolver routine**:<br>

![elf_plt_before](/img/elf_plt_before.png)

<br>

**After** the first call to `func` the `GOT[n]` points to `func`:<br>

![elf_plt_after](/img/elf_plt_after.png)

<br>

## Mitigations
**Lazy binding** can cause to **security issues** because it vulnarable to overwriting function pointers in `.got.plt`.<br>
There is a mitigation technique called **RELRO** (aka **RELocations Read-Only**).<br>

<br>

**RELRO** gives **two** levels of protection:
- **partial RELRO** is for **lazy binding**, it is enabled with `-Wl,-z,relro`:
  - it sets memory regions corresponing to `.got` section as `read-only`;
  - it sets memory regions corresponing to `.got.plt` section as `read-write`;
- **full RELRO** is for **immediate binding**, it is enabled with `-Wl,-z,relro,-z,now` and protects against overwriting function pointers in `.got.plt`:
  - it causes the **dynamic linker** to **immediately** look up every symbol in the **PLT** and update the addresses;
  - then, it sets memory regions corresponing to both `.got` and `.got.plt` sections as `read-only`;

<br>

# `.rela.text` and `.rela.data` sections
Wherever a final address of symbol is **unknown**, the **compiler** adds a **relocation entry** into `.rela.text` or `.rela.data` sections:
- `R_X86_64_PC32` for **PC‑relative relocations**:
  - `disp32 = symbol.st_value + A - P + instruction_size`
    - `A` is an addend from the relocation;
    - `P` is an address of the instruction being patched (i.e., `section_base_VA + offset`);
- `R_X86_64_64` for **absolute relocations**:
  - `address = symbol.st_value + addend`
- `R_X86_64_GOTPCREL` **GOT‑based relocations** for **GOT‑based access**:
  - `disp32 = (GOT_entry_address + A) - (P + instruction_size)`
    - `A` is an addend from the relocation;
    - `P` is an address of the instruction being patched (i.e., `section_base_VA + offset`);
    - `GOT_entry_address` is the address of the corresponding **GOT entry** in `.got` or `.got.plt`;
    - the result `disp32` is a **RIP‑relative offset** that points to the **GOT entry**;
    - the **actual symbol address** is filled later by the *dynamic linker*;

<br>

## Linker steps
Assume **linking** a **shared library** or a **PIE executable** with **dynamic symbols**. The **linker** (`ld`):
- **reads all** *object files* (`.o`, `.so`);
- **merges** their *sections* with the same name (`.text`, `.data`, *etc.*) and **symbol tables**:
  - `a.o .text` + `b.o .text` + `c.o .text` → `final .text`;
  - `a.o .data` + `b.o .data` + `c.o .data` → `final .data`;
  - `a.o .bss` + `b.o .bss` + `c.o .bss` → `final .bss`;
  - *etc.*;
- assigns final base **virtual addresses** (**VA**) for **result** *executable* or *shared object file*;
- assigns base **VA** for **each merged section**;
  - `.text` **base VA** = `0x401000`
  - `.data` **base VA** = `0x402000`
  - `.bss` **base VA** = `0x403000` (no space in file, only in memory)
  - these base addresses become the starting point for all symbols defined inside those sections;
- **resolves symbols**
  - for every symbol that is defined in the input object files, the linker computes the **final address** and assigns it to `st_value` as:
  - `st_value = section_base_VA + offset_in_section`
  - where:
    - `section_base_VA` is the base VA assigned to the merged section;
    - `offset_in_section` is the original value of `st_value` of symbol, it holds **offset** of the symbol in that section;
- **applies static relocations**
  - when the *linker* applies a relocation:
    - it uses the **final computed** `st_value`;
    - it calculates **new** `r_offset` for each *relocation entry* and use this **new** `r_offset`;
    - **applies relocations** in `.rela.text` or `.rela.data` for **all known** and **defined** **symbols** directly in `.text` / `.data`;
    - for extern symbols it **adds** *relocation entry* with new `r_offset` in `.rela.dyn`/`.rela.plt`;
    - **removes** *relocation entry* from `.rela.text` or `.rela.data`;
- creates the **GOT**:
  - allocates space for:
    - `.got`: **general** *GOT entries* for **data references**;
    - `.got.plt`: **special** *GOT entries* **used by the PLT** for function calls;
- creates the **PLT**:
  - for **each external function call** (`call xxxxx@plt`) that needs **lazy binding** the linker creates a **typical stub code** in `.plt`:

```
xxxxx@plt:
    jmp QWORD PTR [xxxxx@GOT]     ; jump to the address from GOT
    push OFFSET_FOR_RESOLVER      ; argument for the resolver
    jmp resolver_stub             ; call the common resolution routine
```

  - where:
    - `jmp QWORD PTR [xxxxx@GOT]` - a jump to the address stored in the GOT entry for the `xxxxx` symbol, i.e. `xxxxx` can be `printf` or `malloc`;
    - `push OFFSET_FOR_RESOLVER` and `jmp resolver_stub` are for resolver;

- emits *relocation entries* in `.rela.dyn` for all **unknown symbols**, that must be resolved at runtime:
  - these *relocation entries* tell the *dynamic loader* (`ld.so`) where and how to **patch** the *GOT entries* by **actual symbol address**;
    - `R_X86_64_GLOB_DAT` for **data symbols** resolved to their final address;
    - `R_X86_64_JUMP_SLOT` for **function calls** in the GOT used by PLT;
- generates **PLT‑specific** relocations in `.rela.plt`;
  - `.rela.plt` is used specifically for entries related to the PLT/GOT pair for functions:

<br>

## Merging sections
Suppose we have:
- `a.o` contains `.text` of size `0x20`, `foo()` defined at offset `0x10` inside it;
- `b.o` contains `.text` of size `0x40`, `bar()` defined at offset `0x30` inside it;

Linker decides to place them in order: `a.o` first, then `b.o`, with 16‑byte (`0x10`) alignment between sections.<br>

**Layout**:
- final **base VA** = `0x401000`;
- the `.text` of `a.o` starts at **VA** = `0x401000`;
- the `.text` of `b.o` starts at **VA** = `0x401000 + size_of(.text in a.o)`
- in the merged section the **final computed** `st_value` for `bar()` is: `0x401000 + size_of(.text in a.o) + 0x30`, where `0x30` is an **original offset** in `b.o .text`;
