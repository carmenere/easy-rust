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
- [`readelf` command](#readelf-command)
- [`strip` command](#strip-command)

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
The **ELF** is specified in the **System V ABI**.<br>

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
|`.rodata`|This section holds **read-only data**.|
|`.data`|This section holds **initialized** data.|
|`.bss`|For **uninitialized** data. The section occupies no file space.|
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

<br>

# `readelf` command
A common tool to quickly parse ELF files is the `readelf` utility from **GNU binutils**.<br>
`readelf --file-header hello` shows **ELF headr**.<br>
`readelf --sections hello` prints out all **sections**.<br>
`readelf --segments hello` prints out all **segments** and section to segment **mapping**.<br>

<br>

**ELF Header**:
```bash
readelf --file-header hello

ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              DYN (Position-Independent Executable file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
  Entry point address:               0x1080
  Start of program headers:          64 (bytes into file)
  Start of section headers:          14208 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         13
  Size of section headers:           64 (bytes)
  Number of section headers:         31
  Section header string table index: 30
```

<br>

**Sections**:
```bash
readelf --sections hello

There are 31 section headers, starting at offset 0x3780:
Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [ 0]                   NULL             0000000000000000  00000000
       0000000000000000  0000000000000000           0     0     0
  [ 1] .interp           PROGBITS         0000000000000318  00000318
       000000000000001c  0000000000000000   A       0     0     1
  [ 2] .note.gnu.pr[...] NOTE             0000000000000338  00000338
       0000000000000030  0000000000000000   A       0     0     8
  [ 3] .note.gnu.bu[...] NOTE             0000000000000368  00000368
       0000000000000024  0000000000000000   A       0     0     4
  [ 4] .note.ABI-tag     NOTE             000000000000038c  0000038c
       0000000000000020  0000000000000000   A       0     0     4
  [ 5] .gnu.hash         GNU_HASH         00000000000003b0  000003b0
       0000000000000024  0000000000000000   A       6     0     8
  [ 6] .dynsym           DYNSYM           00000000000003d8  000003d8
       00000000000000c0  0000000000000018   A       7     1     8
  [ 7] .dynstr           STRTAB           0000000000000498  00000498
       000000000000009d  0000000000000000   A       0     0     1
  [ 8] .gnu.version      VERSYM           0000000000000536  00000536
       0000000000000010  0000000000000002   A       6     0     2
  [ 9] .gnu.version_r    VERNEED          0000000000000548  00000548
       0000000000000030  0000000000000000   A       7     1     8
  [10] .rela.dyn         RELA             0000000000000578  00000578
       00000000000000f0  0000000000000018   A       6     0     8
  [11] .rela.plt         RELA             0000000000000668  00000668
       0000000000000030  0000000000000018  AI       6    24     8
  [12] .init             PROGBITS         0000000000001000  00001000
       000000000000001b  0000000000000000  AX       0     0     4
  [13] .plt              PROGBITS         0000000000001020  00001020
       0000000000000030  0000000000000010  AX       0     0     16
  [14] .plt.got          PROGBITS         0000000000001050  00001050
       0000000000000010  0000000000000010  AX       0     0     16
  [15] .plt.sec          PROGBITS         0000000000001060  00001060
       0000000000000020  0000000000000010  AX       0     0     16
  [16] .text             PROGBITS         0000000000001080  00001080
       00000000000001af  0000000000000000  AX       0     0     16
  [17] .fini             PROGBITS         0000000000001230  00001230
       000000000000000d  0000000000000000  AX       0     0     4
  [18] .rodata           PROGBITS         0000000000002000  00002000
       000000000000002e  0000000000000000   A       0     0     4
  [19] .eh_frame_hdr     PROGBITS         0000000000002030  00002030
       0000000000000044  0000000000000000   A       0     0     4
  [20] .eh_frame         PROGBITS         0000000000002078  00002078
       00000000000000ec  0000000000000000   A       0     0     8
  [21] .init_array       INIT_ARRAY       0000000000003d90  00002d90
       0000000000000010  0000000000000008  WA       0     0     8
  [22] .fini_array       FINI_ARRAY       0000000000003da0  00002da0
       0000000000000010  0000000000000008  WA       0     0     8
  [23] .dynamic          DYNAMIC          0000000000003db0  00002db0
       0000000000000200  0000000000000010  WA       7     0     8
  [24] .got              PROGBITS         0000000000003fb0  00002fb0
       0000000000000050  0000000000000008  WA       0     0     8
  [25] .data             PROGBITS         0000000000004000  00003000
       000000000000004c  0000000000000000  WA       0     0     32
  [26] .bss              NOBITS           0000000000004060  0000304c
       0000000000000048  0000000000000000  WA       0     0     32
  [27] .comment          PROGBITS         0000000000000000  0000304c
       000000000000002b  0000000000000001  MS       0     0     1
  [28] .symtab           SYMTAB           0000000000000000  00003078
       00000000000003f0  0000000000000018          29    19     8
  [29] .strtab           STRTAB           0000000000000000  00003468
       00000000000001fc  0000000000000000           0     0     1
  [30] .shstrtab         STRTAB           0000000000000000  00003664
       000000000000011a  0000000000000000           0     0     1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  D (mbind), l (large), p (processor specific)
```

<br>

**Program Headers** and Section to Segment mapping:
```bash
readelf --segments hello

Elf file type is DYN (Position-Independent Executable file)
Entry point 0x1080
There are 13 program headers, starting at offset 64

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  PHDR           0x0000000000000040 0x0000000000000040 0x0000000000000040
                 0x00000000000002d8 0x00000000000002d8  R      0x8
  INTERP         0x0000000000000318 0x0000000000000318 0x0000000000000318
                 0x000000000000001c 0x000000000000001c  R      0x1
      [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
  LOAD           0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000698 0x0000000000000698  R      0x1000
  LOAD           0x0000000000001000 0x0000000000001000 0x0000000000001000
                 0x000000000000023d 0x000000000000023d  R E    0x1000
  LOAD           0x0000000000002000 0x0000000000002000 0x0000000000002000
                 0x0000000000000164 0x0000000000000164  R      0x1000
  LOAD           0x0000000000002d90 0x0000000000003d90 0x0000000000003d90
                 0x00000000000002bc 0x0000000000000318  RW     0x1000
  DYNAMIC        0x0000000000002db0 0x0000000000003db0 0x0000000000003db0
                 0x0000000000000200 0x0000000000000200  RW     0x8
  NOTE           0x0000000000000338 0x0000000000000338 0x0000000000000338
                 0x0000000000000030 0x0000000000000030  R      0x8
  NOTE           0x0000000000000368 0x0000000000000368 0x0000000000000368
                 0x0000000000000044 0x0000000000000044  R      0x4
  GNU_PROPERTY   0x0000000000000338 0x0000000000000338 0x0000000000000338
                 0x0000000000000030 0x0000000000000030  R      0x8
  GNU_EH_FRAME   0x0000000000002030 0x0000000000002030 0x0000000000002030
                 0x0000000000000044 0x0000000000000044  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000002d90 0x0000000000003d90 0x0000000000003d90
                 0x0000000000000270 0x0000000000000270  R      0x1

Section to Segment mapping:
  Segment Sections...
   00
   01     .interp
   02     .interp .note.gnu.property .note.gnu.build-id .note.ABI-tag .gnu.hash .dynsym .dynstr .gnu.version .gnu.version_r .rela.dyn .rela.plt
   03     .init .plt .plt.got .plt.sec .text .fini
   04     .rodata .eh_frame_hdr .eh_frame
   05     .init_array .fini_array .dynamic .got .data .bss
   06     .dynamic
   07     .note.gnu.property
   08     .note.gnu.build-id .note.ABI-tag
   09     .note.gnu.property
   10     .eh_frame_hdr
   11
   12     .init_array .fini_array .dynamic .got
```

<br>

> Note:<br>
> **Code segement** must have `RE` (**read** and **execute**) permission.<br>
> **Data segments** may have only `R` (**read**) permission or `RW` (**read** and **write**) permissions.<br>

<br>

# `strip` command
To delete **symbols** (*symbol table* or/and *debug symbols*) from object file there is special command: `strip`.<br>

**Example**:
```bash
strip -s hello
```

<br>

**Options**:
- `-s`, `--strip-all` remove **all** symbols;
- `-d`, `--strip-debug` remove **debugging symbols** only;
- `-o <file>` put the stripped output in file `<file>`, rather than replacing the existing file;
