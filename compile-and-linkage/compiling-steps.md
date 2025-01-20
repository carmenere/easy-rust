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
- [PIC](#pic)
  - [Mitigations](#mitigations)

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
There are 3 type of binding libraries to binary:
- **static linking**;
- **dynamic linking** at **loading time** (**by default**);
- **dynamic linking** at **runtime** (`dl` library is needed);

<br>

When the linker creates a **shared library**, it **doesn't** know in advance where it might be loaded. Modern systems use **ASLR** feature (address space layout randomization) and load libraries at **different** locations on each program invocation. So, **shared libraries** are loaded into **non-deterministic** addresses and **absolute** addresses used inside them **must** be **updated** at load time.<br>

<br>

There are 3 main approaches to solve this problem in Linux ELF shared libraries:
- **Load-time relocation**;
- **Position Independent Code** (**PIC**) for **shared libraries**;
- **Position Independent Executables** (**PIE**) for **executable**;

<br>

**Load-time relocation** has a couple of problems:
  - it requires to change the `.text` section and which makes using of the library **non-shareable**;
  - it takes time to perform, because **dynamic linker** performs **all** relocations **before** calling `_start` entrypoint;

<br>

**PIC** and **PIE** are recommended solutions.<br>

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

# PIC
**PIC** uses the **Procedure Linkage Table** (**PLT**) and the **Global Offset Table** (**GOT**).<br>

The **GOT** holds entries of addresses of **global variables** and **external functions**. A **GOT** is simply a **table of addresses**, residing in the **data** section.<br>
The **PLT** consists of short entries of instructions (aka **trampolines**), used to reach **external** functions.<br>

<br>

The **runtime memory address**, also known as **absolute memory address** of variables and functions is **unknown** before the program is started and cannot be hardcoded during compilation. The **GOT** maps symbols to their corresponding **absolute memory addresses**.<br>

<br>

There are 2 approaches of relocating **GOT** entries:
- **early binding**, when **GOT** entries are relocated by the **dynamic linker** at **load-time**;
- **lazy linking**, when **GOT** entries are relocated on-demand **when they are called**, in other words, the **GOT** is populated **dynamically** as the program is running;

<br>

One of the key insights on which **PIC** relies is the **offset** between the **text** and **data** sections, **known** to the linker at **link-time**. When the linker combines several object files together, it collects their sections. Therefore, the linker knows both about the sizes of the sections and about their relative locations:<br>

![elf_got_code_data_offset](/img/elf_got_code_data_offset.png)

<br>

Suppose some instruction in the code section wants to refer to a **variable**. Instead of referring to it directly by absolute address, it refers to an entry in the **GOT**. Since the **GOT** is in a **known** place in the data section, this reference is **relative** and **known** to the linker. The **GOT** entry, in turn, will contain the **absolute address** of the variable.<br>

![elf_got_code_data](/img/elf_got_code_data.png)

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

The `.got.plt` section is used in conjunction with `.plt` in the **lazy** binding process:
- `.got.plt` is a **runtime-writable** part of **GOT**: linker updates entires corresponding `.got.plt` section as they are accessed;
- `.plt` is a part of **PLT** for **runtime-writable** part of **GOT**;

<br>

The `.got` section is used in conjunction with `.plt.got` in the **non-lazy** binding process:
- `.got` is a **read-only** part of **GOT**: linker updates all entires corresponding `.got` section at **load-time**;
- `.plt.got` is a part of **PLT** for **read-only** part of **GOT**;

<br>

## Mitigations
**Lazy linking** can cause to security issues because it vulnarable to overwriting function pointers in `.got.plt`.<br>
There is a mitigation technique called **RELRO** (aka **RELocations Read-Only**).<br>

<br>

**RELRO** gives **two** levels of protection:
- **partial RELRO** enabled with `-Wl,-z,relro`:
  - it sets memory regions corresponing to `.got` section as `read-only`;
- **full RELRO** enabled with `-Wl,-z,relro,-z,now` and protects against overwriting function pointers in `.got.plt`:
  - it causes the **dynamic linker** to **immediately** look up every symbol in the **PLT** and update the addresses;
  - then it sets memory regions corresponing to `.got.plt` section as `read-only`;
