# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [ELF](#elf)
- [ELF layout](#elf-layout)
  - [ELF Data Types](#elf-data-types)
    - [32-Bit Data Types](#32-bit-data-types)
    - [64-Bit Data Types](#64-bit-data-types)
  - [ELF header](#elf-header)
    - [`e_ident` field specification](#e_ident-field-specification)
  - [Program header table](#program-header-table)
- [Sections](#sections)
  - [Section types](#section-types)
  - [Most important sections](#most-important-sections)
- [ELF symbols](#elf-symbols)
  - [Symbol binding](#symbol-binding)
  - [Symbol types](#symbol-types)
- [ELF relocation entries](#elf-relocation-entries)
- [ELF segments](#elf-segments)
  - [Segment types](#segment-types)
  - [Segment Permissions](#segment-permissions)
- [RIP-relative addressing](#rip-relative-addressing)
- [Два основных сценария использования GOT](#два-основных-сценария-использования-got)
- [Два режима RELRO](#два-режима-relro)
- [](#)
- [disp32](#disp32)
- [disp32](#disp32-1)
- [новый\_r\_offset](#новый_r_offset)
- [Addend](#addend)
- [disp32](#disp32-2)
<!-- TOC -->

<br>

# ELF
The **ELF specification** version **4.1** was published in March **1997** as part (chapters **4** and **5**) of the **SVR4 gABI** document (aka *UNIX System V Release 4 gABI*).<br>
The **last version** of **ELF specification** is **4.3** and it was published in **2025** by **Xinuos, Inc**: https://gabi.xinuos.com/.<br>
Now **Xinuos, Inc** maintains the official specification for the **ELF gABI** (*generic Application Binary Interface*). **Xinuos, Inc** is the successor to the assets of **The SCO Group** and **UnixWare**.<br>

<br>

The **ELF** (**Executable and Linkable Format**) is a common standard file format for **object files**: *executable files*, *libraries*, *core dumps*.<br>
There are **3 main types** of *object files*:
- a **relocatable file** holds *code* and *data* **suitable for linking with other object files** to create an **executable** or a **shared object file**;
- an **executable file** holds a **program suitable for execution**;
- a **shared object file** holds *code* and *data* **suitable for linking in two contexts**:
  - *first*, the **linker** processes it with *other relocatable* and *shared object files* **to create another object file**;
  - *second*, the **dynamic linker** combines it with an *executable file* and *other shared objects* **to create a process image**;

<br>

# ELF layout
Files in ELF format include an **ELF header**, a **program header**, a **section header** and consist of multiple **segments** and **sections**.<br>
A **single segment** can be composed of **multiple sections**. *Segments* and *sections* are placed between *program header table* and *section header table*.<br>

<br>

The **ELF** file includes **3 headers** (as they follow in ELF):
- **ELF header**
  - it is located at the very beginning of ELF file;
  - it is represented by `Elfxx_Ehdr` structure (`xx` can be `32` or `64`);
  - if contains pointers to **program header table** and **section header table**;
- **program header table** (used by **loader**);
  - it describes **all segments** in an *object file*;
  - is an **array** of `Elfxx_Phdr` type (`xx` can be `32` or `64`);
  - `Elfxx_Phdr` contains segment **name**, segment **size** and so on;
  - **each instance** of `Elfxx_Phdr` is called **segment header**;
  - **each entry** in *program header table* is an instance of `Elfxx_Phdr`;
  - **every segment** in an *object file* has **exactly one segment header** describing it;
  - **program headers** are **meaningful** only for **executable** and **shared** *object files*;
  - each **segment** contains information the system needs to prepare the program for execution, in other words it specifies how the system (e.g., `exec()`) **creates a process image from executable file**;
- **section header table** (used by **linker**);
  - it describes **all sections** in an *object file*;
  - is an **array** of `Elfxx_Shdr` type (`xx` can be `32` or `64`);
  - `Elfxx_Shdr` contains section **name**, section **size** and so on;
  - **each instance** of `Elfxx_Shdr` is called **section header**;
  - **each entry** in *section header table* is an instance of `Elfxx_Shdr`;
  - **every section** in an *object file* has **exactly one section header** describing it;

<br>

**Object files** participate in both **linking** and **execution**. The ELF file provides **two separate views** on the data inside the ELF file:
- **section view** for linker;
- **segment view** for loader;

![elf_views](/img/elf_views.webp)

<br>
<br>

![elf_layout](/img/elf_layout.png)

<br>

## ELF Data Types
### 32-Bit Data Types
|Name|Size|Alignment|
|:---|:---|:--------|
|Elf32_Addr|4|4|
|Elf32_Off|4|4|
|Elf32_Half|2|2|
|Elf32_Word|4|4|
|Elf32_Sword|4|4|
|unsigned char|1|1|

<br>

### 64-Bit Data Types
|Name|Size|Alignment|
|:---|:---|:--------|
|Elf64_Addr|8|8|
|Elf64_Off|8|8|
|Elf64_Half|2|2|
|Elf64_Word|4|4|
|Elf64_Sword|4|4|
|Elf64_Xword|8|8|
|Elf64_Sxword|8|8|
|unsigned char|1|1|

<br>

## ELF header
The **ELF header** contains **general information** about the *object file*.<br>

<br>

There are 2 types of *ELF*:
- **ELF32** is a **32-bit version** of the *ELF*;
- **ELF64** is a **64-bit version** of the *ELF*;

<br>

The **ELF header** is denoted by an `Elfxx_Ehdr` structure, `xx` can be `32` or `64`:
- **Elf32_Ehdr**:
```c
typedef struct {
    unsigned char   e_ident[EI_NIDENT];
    Elf32_Half      e_type;
    Elf32_Half      e_machine;
    Elf32_Word      e_version;
    Elf32_Addr      e_entry;
    Elf32_Off       e_phoff;
    Elf32_Off       e_shoff;
    Elf32_Word      e_flags;
    Elf32_Half      e_ehsize;
    Elf32_Half      e_phentsize;
    Elf32_Half      e_phnum;
    Elf32_Half      e_shentsize;
    Elf32_Half      e_shnum;
    Elf32_Half      e_shstrndx;
} Elf32_Ehdr;
```
- **Elf64_Ehdr**
```c
typedef struct {
    unsigned char   e_ident[EI_NIDENT];
    Elf64_Half      e_type;
    Elf64_Half      e_machine;
    Elf64_Word      e_version;
    Elf64_Addr      e_entry;
    Elf64_Off       e_phoff;
    Elf64_Off       e_shoff;
    Elf64_Word      e_flags;
    Elf64_Half      e_ehsize;
    Elf64_Half      e_phentsize;
    Elf64_Half      e_phnum;
    Elf64_Half      e_shentsize;
    Elf64_Half      e_shnum;
    Elf64_Half      e_shstrndx;
} Elf64_Ehdr;
```

<br>

- `e_ident[16]` this field is called **ELF Identification**, it is an array of **16 bytes**;
- `e_type` identifies the **object file type**, values:
  - **1** - `ET_REL`: **relocatable** file, i.e. an *object file* **before** *linking*;
  - **2** - `ET_EXEC`: **executable** file;
  - **3** - `ET_DYN`: **shared object** file;
  - **4** - `ET_CORE`: **core** file;
- `e_machine` specifies target **instruction set architecture** (**ISA**);
  - `x86`
  - `AMD x86-64`
  - `AArch64`
  - ...
- `e_version` identifies the **object file version**;
- `e_entry` contains the **virtual address** of the program **entry point**, where **execution starts**, if the file has no associated entry point, this member holds zero;
- `e_phoff` holds the *program header table* **offset** in *bytes* from the beginning of the file;
- `e_phentsize` holds the **size** in *bytes* of a **single program header**;
  - each entry in the *program header table* is called **program header**;
  - **all entries** have the **same size**;
- `e_phnum` holds the **number of entries** in the *program header table*;
- `e_shoff` holds the *section header table* **offset** in *bytes* from the beginning of the file;
- `e_shentsize` holds the **size** in *bytes* of a **single section header**;
  - each entry in the *section header table* is called **section header**;
  - **all entries** have the **same size**;
- `e_shnum` holds the **number of entries** in the *section header table*;
- `e_flags` holds processor-specific flags associated with the file;
- `e_ehsize` holds the size of **ELF header** in *bytes*;
- `e_shstrndx` holds the *section header table* **index**;

<br>

`readelf` command:
- `readelf -h <file>` — displays ELF header details;
- `readelf -l <file>` — displays program headers and segment mapping;
- `readelf -S <file>` — lists all individual file Sections;

<br>

### `e_ident` field specification
The `e_ident` field **differs** for **ELF32** and **ELF64**: **ELF64** adds 2 bytes `EI_OSABI` and `EI_ABIVERSION` with padding `EI_PAD` at index **9**.<br>

|Byte name|Field|Index value for ELF32|Index value for ELF64|Purpose|
|:--------|:----|:-------|:----|:------|
|`EI_MAG0`|`e_ident[EI_MAG0]`|0|0|**File identification**: `0x7f`|
|`EI_MAG1`|`e_ident[EI_MAG1]`|1|1|**File identification**: `0x45` (`E`)|
|`EI_MAG2`|`e_ident[EI_MAG2]`|2|2|**File identification**: `0x4c` (`L`)|
|`EI_MAG3`|`e_ident[EI_MAG3]`|3|3|**File identification**: `0x46` (`F`)|
|`EI_CLASS`|`e_ident[EI_CLASS]`|4|4|**File class** determines **CPU architecture bit width**: **32**-bit (**ELF32**) or  **64**-bit (**ELF64**)|
|`EI_DATA`|`e_ident[EI_DATA]`|5|5|**CPU byte order** (**endianness**): **big-endian** or **little-endian**|
|`EI_VERSION`|`e_ident[EI_VERSION]`|6|6|**File version**|
|`EI_OSABI`|`e_ident[EI_OSABI]`|-|7|Identifies the target **Operating System ABI**|
|`EI_ABIVERSION`|`e_ident[EI_ABIVERSION]`|-|8|Specifies the **OS ABI version**|
|`EI_PAD`|`e_ident[EI_PAD]`|7|9|Reserved **padding bytes**. **Currently unused**. Should be **filled with zeros** and **ignored** when read|
|`EI_NIDENT`|`e_ident[EI_NIDENT]`|16|16|**Size of** `e_ident[]`|

<br>

A **CPU architecture bit width** refers to the size of the processor's data registers and the amount of data it can process in a single instruction cycle.<br>
An **instruction set architecture** (**ISA**) is an **abstract model** that defines the **programmable interface of the CPU**, defining how software interacts with hardware.<br>
A CPU that interprets instructions described by an ISA is an implementation of that ISA.<br>
In general, an ISA defines the **instructions**, **data types**, **registers**, and the programming interface for managing main memory such as **addressing modes**, **virtual memory**, and memory consistency mechanisms. The ISA also includes the **input/output model** of the programmable interface.<br>

<br>

An *ISA* is *distinguished from* a **microarchitecture**.<br>
A **microarchitecture**, sometimes abbreviated as **μarch** or **uarch**, is the **way** a **given ISA** is **implemented** in a particular CPU.<br>
A *given ISA* **may be implemented with different** *microarchitectures*.<br>

<br>

**Computer architecture** is the **combination** of *microarchitecture* and *ISA*.<br>

<br>

## Program header table
- **Elf32_Phdr**:
```c
typedef struct {
    Elf32_Word  p_type;
    Elf32_Off   p_offset;
    Elf32_Addr  p_vaddr;
    Elf32_Addr  p_paddr;
    Elf32_Word  p_filesz;
    Elf32_Word  p_memsz;
    Elf32_Word  p_flags;
    Elf32_Word  p_align;
} Elf32_Phdr;
```
- **Elf64_Phdr**:
```c
typedef struct {
    Elf64_Word  p_type;
    Elf64_Word  p_flags;
    Elf64_Off   p_offset;
    Elf64_Addr  p_vaddr;
    Elf64_Addr  p_paddr;
    Elf64_Xword p_filesz;
    Elf64_Xword p_memsz;
    Elf64_Xword p_align;
} Elf64_Phdr;
```

<br>

# Sections
**Sections** represent the smallest **indivisible units** that used by **linker**.<br>

**Section header table** describes **all sections** in an *object file*.<br>
**Section header table** is an **array** of `Elfxx_Shdr` type (`xx` can be `32` or `64`).<br>
**each instance** of `Elfxx_Shdr` is called **section header**.<br>
**Each entry** in *section header table* is an instance of `Elfxx_Shdr`.<br>
**Every section** in an *object file* has **exactly one section header** describing it.<br>

<br>

- **Elf32_Shdr**
```c
typedef struct {
    Elf32_Word  sh_name;
    Elf32_Word  sh_type;
    Elf32_Word  sh_flags;
    Elf32_Addr  sh_addr;
    Elf32_Off   sh_offset;
    Elf32_Word  sh_size;
    Elf32_Word  sh_link;
    Elf32_Word  sh_info;
    Elf32_Word  sh_addralign;
    Elf32_Word  sh_entsize;
} Elf32_Shdr;
```
- **Elf64_Shdr**
```c
typedef struct {
    Elf64_Word  sh_name;
    Elf64_Word  sh_type;
    Elf64_Xword sh_flags;
    Elf64_Addr  sh_addr;
    Elf64_Off   sh_offset;
    Elf64_Xword sh_size;
    Elf64_Word  sh_link;
    Elf64_Word  sh_info;
    Elf64_Xword sh_addralign;
    Elf64_Xword sh_entsize;
} Elf64_Shdr;
```

<br>

- `sh_name` holds an offset to a string in the `.shstrtab` section that represents the **name** of this section;
- `sh_type` holds **type of section** which defines the section’s contents and semantics;
- `sh_flags` holds the **attributes** of the section;
- `sh_addr` holds **virtual address** of the section **in memory**, for sections that are loaded;
- `sh_offset` holds the **offset** of the section in *bytes* from the beginning of the file;
- `sh_size` holds the **size of section** in *bytes*, i.e. number of bytes section occupies in the object file;
  - interpretation of `sh_size` depends on `sh_type`:
    - if section has `SHT_NOBITS` type it **occupies no space in the file** and `sh_size` means number of **0 bytes** to be allocated in memory;
    - for example, `.bss` segment has type `SHT_NOBITS`;
- `sh_info` this member holds extra information, whose interpretation depends on the section type. 
- `sh_entsize` some sections hold an **array of fixed-size entries**, such as a **symbol table**, for such a section, this field holds the **size** in *bytes* **of each entry**;

<br>

В Section Header для секции .bss есть поле sh_size — это и есть нужный размер в байтах. В выводе readelf -S ты видишь это как колонку Size:
[Nr] Name              Type            Address          Off      Size     EntSize
[ 8] .bss              NOBITS          0000000000000000 00001000 00000020 00000000
Здесь Size = 0x20 (32 байта) — столько надо выделить под .bss. Тип NOBITS означает: «в файле данных нет, но в памяти должно быть столько».

Это удобно для анализа, но не используется напрямую при загрузке процесса.

<br>

## Section types
- `SHT_SYMTAB` and `SHT_DYNSYM`
  - these sections hold a **symbol table**;
  - an object file may have **only one section of each type**;
  - `SHT_SYMTAB` holds **all** symbols for **linking**;
  - `SHT_DYNSYM` holds a **minimal set** of symbols for **dynamic linking**;
- `SHT_STRTAB`
  - section of this type holds a **string table**;
  - **string table** sections hold **null-terminated character sequences**, commonly called strings;
  - the object file uses these strings to represent **symbol** and **section names**;
  - an object file may have **multiple string** table sections;
- `SHT_REL`
  - section of this type holds **relocation entries without** explicit **addends**;
  - **relocation entries without** explicit **addends** represented by `Elfxx_Rel` structure (`xx` can be `32` or `64`);
- `SHT_RELA`
  - section of this type holds **relocation entries with** explicit **addends**;
  - **relocation entries with** explicit **addends** represented by `Elfxx_Rela` structure (`xx` can be `32` or `64`);
- `SHT_HASH`
  - section of this type holds a **symbol hash table**;
- `SHT_DYNAMIC`
  - section of this type holds **information for dynamic linking**;
- `SHT_NOBITS`
  - section of this type **occupies no space in the file**, but **can occupy some space in RAM**;
- `SHT_INIT_ARRAY`
  - this section contains an **array of pointers** to **initialization functions**;
- `SHT_FINI_ARRAY`
  - this section contains an **array of pointers** to **termination functions**;
- `SHT_PREINIT_ARRAY`
  - this section contains an **array of pointers** to **functions** that are **invoked before** all other initialization functions;

<br>

## Most important sections
- `.interp`
  - this section contains the **full path** to the appropriate **dynamic linker**, in other words every **dynamically linked** binary is linked against the **dynamic linker**;
- `.text`
  - this section holds the **executable instructions** for CPU;
- `.rodata` and `.rodata1`
  - this section holds **read-only data**, i.e. **constants** with values provided *at compile time*, like **hardcoded string**;
- `.data`
  - this section holds **initialized** data, i.e. **global variables** with values provided *at compile time*;
- `.bss`
  - for **uninitialized** data, i.e. **uninitialized global variables**;
  - the section **occupies no space in file**, but can have some size in RAM;
- `.note.ABI-tag`
  - this section specifies ABI details;
- `.rel.*`
  - **relocation** sections which contains *relocation entries* **without** explicit **addends**;
  - *relocation section* for `.text` has name `.rel.text`;
  - *relocation section* for `.data` has name `.rel.data`;
- `.rela.*`
  - **relocation** sections which contains *relocation entries* **with** explicit **addends**;
  - *relocation section* for `.text` has name `.rela.text`;
  - *relocation section* for `.data` has name `.rela.data`;
- `.dynamic`
  - this section holds dynamic linking information;
- `.dynsym`
  - this section holds the **symbol table** needed for **dynamic linking**;
- `.dynstr`
  - this section holds **string table** needed for `.dynsym`;
- `.symtab`
  - contains a **symbol table**;
- `.strtab`
  - contains the **string table** for `.symtab`;
- `.debug_info`
  - contains information generated by compilers to describe the source code while debugging by keeping symbols and its type, scope, file, line number, etc;
- `.debug_*`
  - additional sections for debug info;
- `.plt`
  - this section holds the **procedure linkage table** (**PLT**);
- `.got`
  - this section holds the **global offset table** (**GOT**);
- `.relr.dyn`
  - holds **relocation entries for dynamic linking**;
- `.fini`
  -  holds **executable instructions** that contribute to the **process termination code**;
- `.fini_array`
  - contains an **array of pointers** to **termination functions**;
- `.init`
  - holds **executable instructions** that contribute to the **process initialization code**;
- `.init_array`
  - contains an **array of pointers** to **initialization functions**;
- `.preinit_array`
  - contains an **array of pointers** to **termination functions**;

<br>

# ELF symbols
A **symbol table** of a program is a list containing **all** the program's **symbols** (function names, variables names, etc).<br>
The **debug symbols** is a special kind of symbols that attaches additional information needed for debugging.<br>
**DWARF** (*Debug With Arbitrary Record Format*) is a widely used, standardized **debugging data format**.<br>
**DWARF** uses a data structure called a **Debugging Information Entry** (**DIE**) to represent each variable, type, procedure, etc.<br>
In ELF file **DWARF** has been divided into different sections like `.debug_info`, `.debug_frame`, etc.<br>

<br>

A **symbol table entry** has the following format:
- **Elf32_Sym**
```c
typedef struct {
    Elf32_Word     st_name;
    Elf32_Addr     st_value;
    Elf32_Word     st_size;
    unsigned char  st_info;
    unsigned char  st_other;
    Elf32_Half     st_shndx;
} Elf32_Sym;
```
- **Elf64_Sym**
```c
typedef struct {
    Elf64_Word     st_name;
    unsigned char  st_info;
    unsigned char  st_other;
    Elf64_Half     st_shndx;
    Elf64_Addr     st_value;
    Elf64_Xword    st_size;
} Elf64_Sym;
```

<br>

- `st_name`
  - holds an **index into the object file’s symbol string table**, which holds the **character representations of the symbol names**;
- `st_value`
  - holds the **address of symbol**
    - in **relocatable** file it is an **offset from section** where symbol is defined;
    - in **executable** or **shared object** it is a final **virtual address of section** + **offset from section** where symbol is defined;
    - **linker updates** `st_value` for symbols **it was able to resolve** at linking stage;
- `st_size`
  - holds **sizes**;
- `st_info`
  - specifies the **symbol’s type** and **binding attributes**;
- `st_shndx`
  - **index** of *section header* where symbol is defined in *section header table* (`.text`, `.data`);

<br>

## Symbol binding
A symbol’s **binding** determines the **linkage visibility** and behavior:
- `STB_LOCAL`
  - **local symbols** are **not visible outside** the object file containing their definition;
  - local symbols of the same name **may exist in multiple files** without interfering with each other;
- `STB_GLOBAL`
  - **global symbols** are **visible to all** object files being combined;
- `STB_WEAK`
  - **weak symbols** resemble global symbols, but their definitions have lower precedence;

<br>

## Symbol types
- `STT_NOTYPE`
  - the symbol’s type is not specified;
- `STT_OBJECT`
  - the symbol is associated with a **data object**, such as a **variable**, an **array**, and so on;
- `STT_FUNC`
  - the symbol is associated with a **function** or other **executable code**;
- `STT_SECTION`
  - the symbol is associated with a **section**;
- `STT_FILE`
  - the symbol’s name gives the name of the source file associated with the object file;
  - a file symbol has `STB_LOCAL` binding;
- `STT_COMMON`
  - the symbol labels an uninitialized common block;
- `STT_TLS`
  - the symbol specifies a **Thread-Local Storage entity**;

<br>

# ELF relocation entries
There are **two** different **relocation entry structures**: `Elfxx_Rel` and `Elfxx_Rela`.<br>
The only **difference** between `Elfxx_Rel` and `Elfxx_Rela` is that `Elfxx_Rela` contains an explicit *addend* in relocation entry whereas `Elfxx_Rel` store an *addend* in the location to be modified and linker must read *addend* **before** in perform relocation action.<br>
On `x86_64` only `Elf64_Rela` is used.<br>

В секцию .data компилятор кладёт 8 байт: пока что там «заглушка» (например, ноль или адрес-заполнитель). Это и есть переменная func_ptr.
В секцию .rela.data он добавляет запись релокации: «по смещению N в .data запиши адрес символа external_func».

<br>

- **Elf32_Rel**
```c
typedef struct {
    Elf32_Addr   r_offset;
    Elf32_Word   r_info;
} Elf32_Rel;
```
- **Elf32_Rela**
```c
typedef struct {
    Elf32_Addr   r_offset;
    Elf32_Word   r_info;
    Elf32_Sword  r_addend;
} Elf32_Rela;
```
- **Elf64_Rel**
```c
typedef struct {
    Elf64_Addr   r_offset;
    Elf64_Xword  r_info;
} Elf64_Rel;
```
- **Elf64_Rela**
```c
typedef struct {
    Elf64_Addr   r_offset;
    Elf64_Xword  r_info;
    Elf64_Sxword r_addend;
} Elf64_Rela;
```

<br>

**Fields**:
- `r_offset`
  - holds the **location** at which to apply the relocation action;
    - for a **relocatable file**, the value is the **byte offset from the beginning of the section** to the storage unit affected by the relocation;
    - for an **executable file** or a **shared object**, the value is the **virtual address** of the storage unit affected by the relocation;
- `r_info`
  - holds the **symbol table index** and the **type of relocation** to apply;
- `r_addend`
  - specifies a **constant addend** used to compute the value to be stored into the relocatable field;

<br>

# ELF segments
An object file **segment** contains **one** or **more** *sections*.<br>
The **system loader** (e.g., `exec()`) and the **dynamic linker** cooperate to create the **process image** for the program.<br>
A **program header table** contains information about **all segments** in an *object file*.<br>
A **program header table** is an **array** of `Elfxx_Phdr` type (`xx` can be `32` or `64`).<br>
The `Elfxx_Phdr` contains segment **name**, segment **size** and so on.<br>
**Each instance** of `Elfxx_Phdr` is called **segment header**.<br>
**Each entry** in *program header table* is an instance of `Elfxx_Phdr`.<br>
**Every segment** in an *object file* has **exactly one segment header** describing it.<br>
*Program header table* is **meaningful** only for **executable** and **shared** *object files*. It specifies how the system (e.g., `exec()`) **creates a process image from executable file**..<br>
Each **segment header** defines **access permissions** for segment and other info.<br>

<br>

**Text segment** typically includes the following sections:
- `.text`
- `.rodata`
- `.hash`
- `.dynsym`
- `.dynstr`
- `.plt`
- `.rela.got`
- `.rela.plt`

<br>

**Data segment** typically includes the following sections:
- `.data`
- `.dynamic`
- `.got`
- `.bss`

<br>

- **Elf32_Phdr**
```c
typedef struct {
    Elf32_Word  p_type;
    Elf32_Off   p_offset;
    Elf32_Addr  p_vaddr;
    Elf32_Addr  p_paddr;
    Elf32_Word  p_filesz;
    Elf32_Word  p_memsz;
    Elf32_Word  p_flags;
    Elf32_Word  p_align;
} Elf32_Phdr;
```
- **Elf64_Phdr**
```c
typedef struct {
    Elf64_Word  p_type;
    Elf64_Word  p_flags;
    Elf64_Off   p_offset;
    Elf64_Addr  p_vaddr;
    Elf64_Addr  p_paddr;
    Elf64_Xword p_filesz;
    Elf64_Xword p_memsz;
    Elf64_Xword p_align;
} Elf64_Phdr;
```

<br>

- `p_type` holds **kind of segment**;
- `p_offset` holds the **byte offset** from the beginning of the file to the **first byte** of the segment;
- `p_vaddr`
  - holds **virtual address** at which the **first byte** of the segment resides in memory;
- `p_paddr`
  - holds **physical address** at which the **first byte** of the segment resides in memory;
  - meaningful on systems where physical address is relevant;
- `p_filesz`
  - holds **number of bytes** in the **file image** of the segment;
  - it may be zero;
- `p_memsz`
  - holds **number of bytes** in the **memory image** of the segment;
  - it may be zero;
- `p_flags`
  - each bit in `p_flags` field corresponds to some permission:
    - `PF_X` - **Execute**;
    - `PF_W` - **Write**;
    - `PF_R` - **Read**;
  - **combination** of such bits specifies **memory access permissions** for the segment;
  - so, `p_flags` holds **memory access permissions** for the segment;

<br>

## Segment types
Segment types, `p_type`:
- `PT_NULL`
  - means **unused egment**;
  - this type **lets** the *program header table* **have ignored entries**;
- `PT_LOAD`
  - specifies a **loadable segment**, i.e. a segment that **must be mapped directly** into the virtual memory space of the process;
  - described by the segment’s **memory size** (`p_memsz`) and the **file size** (`p_filesz`);
  - the bytes from the file are mapped to the beginning of the memory segment;
  - if the `p_memsz` is **larger** than the `p_filesz`, the **extra zero bytes** are added **after** the *segment’s initialized area*;
  - the `p_filesz` may not be larger than the `p_memsz`;
  - **loadable segment entries** in the *program header table* appear in **ascending order**, **sorted on** the `p_vaddr`;
- `PT_DYNAMIC`
  - holds **control data** required for runtime dynamic linking;
- `PT_INTERP`
  - specifies the **location** and **size** of a **null-terminated path** of the runtime **interpreter**/**dynamic linker**, e.g., `/lib64/ld-linux-x86-64.so.2`;
  - it may **not occur more than once** in a file;
  - if it is present, it **must precede** any *loadable segment* entry;
  - this segment type is *meaningful only for executable files*: when building an *executable file* that uses **dynamic linking**, the linker adds a segment of type `PT_INTERP` to an *executable file*, telling the system **to invoke the dynamic linker** as the program interpreter;
- `PT_NOTE`
  - contains **auxiliary** vendor/OS **metadata** (e.g., minimum kernel version);
- `PT_SHLIB`
  - this segment type is reserved but **has unspecified semantics**;
- `PT_PHDR`
  - the array element, if present, specifies the location and size of the *program header table* itself
  - if it is present, it **must precede** any *loadable segment* entry.
  - this segment type may not occur more than once in a file.
- `PT_TLS`
  - specifies the **thread-local storage** *segments*;
  - a **separate** `TLS` *segment* is created **for each thread**, i.e. **each thread has its own local storage**;

<br>

## Segment Permissions
When the system load segments, it gives **access permissions** as specified in the `p_flags` member.<br>
Although all flag combinations are valid, the system **may grant more access than requested**.<br>

The following table shows both the **exact** flag interpretation and the **allowable** flag interpretation:
|Flags|Value|Exact|Allowable|
|:----|:----|:----|:--------|
|none|0|All access **denied**|All access **denied**|
|`PF_X`|1|Execute only|Read, execute|
|`PF_X`|2|Write only|Read, write, execute|
|`PF_W`+`PF_X`|3|Write, execute|Read, write, execute|
|`PF_R`|4|Read only|Read, execute|
|`PF_R`+`PF_X`|5|Read, execute|Read, execute|
|`PF_R`+`PF_W`|6|Read, write|Read, write, execute|
|`PF_R`+`PF_W`+`PF_X`|7|Read, write, execute|Read, write, execute|

<br>

# RIP-relative addressing
Для позиционно‑зависимого кода (не‑PIE): линкер сразу подставляет в инструкции и данные абсолютные адреса для конкретного базового адреса (например, 0x400000). При загрузке ОС просто отображает сегмент по этому адресу (или близкому к нему). Никаких дополнительных релокаций при загрузке не нужно.


Для PIE/PIC: код специально написан так, чтобы не зависеть от абсолютного адреса; релокации либо не нужны, либо обрабатываются динамическим загрузчиком (для динамических библиотек) или заранее (для статического PIE).


Не‑PIE: код «зашит» под конкретный базовый адрес (например, .text должен начинаться с 0x400000). Если ОС загрузит его по другому адресу, абсолютные адреса в коде и данных станут неверными — программа сломается.
PIE: весь код и большинство обращений к данным устроены так, что работают при любом базовом адресе.

GOT и PLT для внешних символов
Когда нужен адрес символа из другой библиотеки (или другого объектного файла), который может оказаться где угодно, используют Global Offset Table (GOT) и Procedure Linkage Table (PLT).

Для функций:

Вызов printf() превращается в call printf@plt.
printf@plt — это короткая заглушка, которая через GOT получает реальный адрес функции и делает прыжок.
Адрес в GOT подставляется загрузчиком (динамическим линковщиком) при старте программы.
Для переменных:

Доступ к global_var делается через запись в GOT: mov global_var@GOTPCREL(%rip), %rax.
В GOT лежит реальный адрес переменной, который подставляет загрузчик.
Таким образом, в машинном коде нет абсолютных адресов внешних символов — только смещения и ссылки на GOT.

В PIE компилятор не сделает mov global_var, %eax (это абсолютный адрес). Вместо этого будет примерно так:
mov global_var@GOTPCREL(%rip), %rax  ; загружаем из GOT адрес переменной
mov (%rax), %eax                     ; читаем значение

Первая инструкция берёт из GOT адрес global_var, который подставит загрузчик.
Вторая инструкция читает значение по этому адресу.

Линкер (ld) делает следующее:

Создаёт записи в GOT для каждой внешней функции (например, printf@GOT). Изначально в них записаны адреса-заглушки — обычно это адрес первой инструкции в соответствующем PLT‑кадре.
Создаёт PLT‑кадры — небольшие блоки кода по одному на каждую внешнюю функцию.
Оставляет релокации (в секции .rela.plt или .rel.plt), чтобы динамический линковщик потом подставил туда реальные адреса.
На этом этапе в GOT ещё нет реальных адресов функций — только «временные» значения.

Шаг 3. Загрузка программы: роль динамического линковщика (ld.so)
Когда ты запускаешь программу, загрузчик ELF:

Загружает сегменты исполняемого файла и нужных .so (библиотек).
Применяет ленивую релокацию (lazy binding): не все адреса подставляются сразу, а только когда функция реально вызывается.
Для ленивой привязки использует специальный механизм с переходом через PLT.

Шаг 4. Первый вызов функции: «ленивая» привязка (lazy binding)
Допустим, программа впервые делает call printf@plt.

Что происходит внутри printf@plt
Типичный PLT‑кадр выглядит так (упрощённо, x86‑64):
Шаг 4. Первый вызов функции: «ленивая» привязка (lazy binding)
Допустим, программа впервые делает call printf@plt.

Что происходит внутри printf@plt
Типичный PLT‑кадр выглядит так (упрощённо, x86‑64):
printf@plt:
    jmp QWORD PTR [printf@GOT]      ; прыжок по адресу из GOT
    push QWORD PTR offset_to_resolver
    jmp resolver_stub
Первая инструкция — это и есть «точка входа»: она сразу прыгает по адресу, хранящемуся в printf@GOT.
На момент первого вызова в printf@GOT лежит не реальный адрес printf, а адрес второй части PLT (или адрес специального кода-резолвера).
Поэтому при первом вызове происходит прыжок не в printf, а в код, который должен «разрешить» адрес.

Как работает резолвер (resolver)
Резолвер — это внутренний код динамического линковщика. Он:

По номеру символа (из релокации) находит имя функции (printf).
Ищет её реальный адрес в загруженной библиотеке (например, в libc.so).
Записывает этот реальный адрес в ячейку printf@GOT (перезаписывая старое значение).
Выполняет прыжок на реальную функцию printf.
Возвращается из неё как обычно.
Важный момент: после этого в printf@GOT теперь лежит настоящий адрес printf.

Шаг 5. Второй и последующие вызовы: быстрый путь
Теперь, когда ты снова делаешь call printf@plt:

Выполняется jmp QWORD PTR [printf@GOT].
В printf@GOT уже лежит реальный адрес printf.
Происходит прямой прыжок на функцию.
Никакого резолвера и поиска больше нет.
Таким образом, первый вызов медленный (нужен поиск и обновление GOT), а все следующие — быстрые (просто косвенный прыжок).


Как устроены данные: GOT для переменных
Для глобальных переменных механизм похож, но без PLT:

Доступ к переменной идёт через GOT: mov global_var@GOTPCREL(%rip), %rax.
В ячейке GOT хранится реальный адрес переменной.
Если переменная находится в динамической библиотеке, её адрес подставляется динамическим линковщиком (может быть лениво или сразу, зависит от флагов и настроек).
PLT нужен только для функций, потому что для них удобно сделать «заглушку‑переходник». Для переменных достаточно просто косвенного доступа через GOT.


После того как динамический линковщик записал реальный адрес printf в ячейку printf@GOT, код PLT выглядит так (x86‑64, упрощённо):
printf@plt:
    jmp QWORD PTR [printf@GOT]   ; один прыжок по адресу из памяти

То есть:

Процессор читает одно значение из памяти — ячейку GOT.
Делает один прыжок на этот адрес (в реальную функцию printf).


call printf@plt — это вызов функции из твоего кода.
jmp QWORD PTR [printf@GOT] внутри PLT — это косвенный переход к реальному адресу функции.
Тут нет сохранения адреса возврата — это просто «перекинуть управление дальше».


Чтение GOT попадает в кэш (GOT обычно лежит рядом с кодом, либо в отдельном блоке, который быстро прогревается).
Современные процессоры хорошо справляются с косвенными переходами.

`.got` — основная часть: адреса глобальных переменных и иногда функций.
`.got.plt` — специальная часть для PLT: там лежат адреса функций, через которые идут вызовы вида call sym@plt.

Каждая ячейка GOT — это просто одно машинное слово (на x86‑64 это 8 байт) с адресом. Никаких заголовков и структур внутри нет: это плоский массив указателей.

Пример:

got[0] — часто служебное значение (например, адрес динамической секции).
got[1], got[2] и т. д. — адреса символов.

Если у тебя есть символ printf, то в .got.plt будет ячейка printf@GOT, и в неё загрузчик запишет реальный адрес printf из libc.so.


# Два основных сценария использования GOT
Для функций (через PLT)
Вызов printf() превращается в call printf@plt, а внутри printf@plt стоит:
Для функций (через PLT)
Вызов printf() превращается в call printf@plt, а внутри printf@plt стоит:
jmp QWORD PTR [printf@GOT]
То есть инструкция делает косвенный прыжок по адресу, лежащему в ячейке GOT.

На этапе первого вызова в этой ячейке лежит не реальный адрес, а адрес резолвера (чтобы запустить lazy binding). После резолвинга туда записывают настоящий адрес функции, и все следующие вызовы идут напрямую.

Для переменных
Доступ к глобальной переменной из другой библиотеки делается через GOT с PC‑relative адресацией:
mov global_var@GOTPCREL(%rip), %rax   ; читаем адрес переменной из GOT
mov (%rax), %eax                     ; читаем значение переменной
Здесь первая инструкция получает из GOT адрес переменной, вторая — её значение.


Как GOT заполняется: ленивая и немедленная привязка
Есть два режима:

Lazy binding (по умолчанию): адреса в GOT подставляются только при первом вызове функции. Для этого в ячейках GOT изначально лежат адреса-заглушки, которые ведут в резолвер. Резолвер находит реальный адрес, записывает его в GOT и сразу прыгает на функцию.
Immediate binding (-Wl,-z,now): динамический линковщик заполняет GOT до входа в main. Тогда при первом же вызове никакого резолвера не будет: в ячейке уже лежит правильный адрес.

Что происходит на каждом этапе
Компиляция: компилятор генерирует обращения через GOT и оставляет релокации (например, R_X86_64_GOTPCREL).
Линковка: линкер резервирует место под GOT и создаёт записи в секциях .got и .got.plt. Он не подставляет реальные адреса внешних символов — оставляет это для динамического линковщика.
Загрузка: динамический линковщик (ld.so) загружает библиотеки, находит реальные адреса символов и записывает их в ячейки GOT (сразу или лениво).
Исполнение: код читает адреса из GOT и использует их для вызовов функций и доступа к переменным.


Права доступа: GOT в рантайме должна быть записываемой, чтобы линковщик мог её обновлять. Это потенциальная точка атаки (GOT overwrite), поэтому есть защиты вроде RELRO.
RELRO:
Частичный RELRO: часть GOT делают только для чтения после инициализации.
Полный RELRO: всю GOT делают только для чтения до запуска кода, и тогда перезаписать её нельзя.

`.rel.text` и `.rel.data` существуют только в перемещаемых объектных файлах (.o), а не в финальных исполняемых файлах или shared-библиотеках. В готовом PIE-бинаре ты их не увидишь — там вместо них будут .rela.dyn и .rela.plt.
В объектном файле (main.o) линковщик ещё не знает финальных адресов символов (они станут известны только при сборке всего проекта). Поэтому ассемблер оставляет «дырки» в коде и данных и создаёт таблицы релокаций, чтобы линкер потом подставил туда правильные адреса.

.rel.text — хранит релокации для инструкций в секции .text (вызовы функций, относительные переходы, обращения к символам из кода).
.rel.data — хранит релокации для инициализированных данных в секции .data (например, когда глобальная переменная инициализируется адресом другой функции/переменной).
Их главная задача: сообщить линковщику, где именно в коде/данных нужно подставить адрес символа и по какой формуле.

Линкер:

Читает .rel.text, .rel.data (и другие .rel.*) из каждого .o.
Применяет статические релокации: подставляет адреса символов, которые уже известны внутри проекта.
Формирует динамические релокации в .rela.dyn и .rela.plt для тех символов, которые разрешаются при загрузке (например, printf из libc).
Не копирует старые .rel.text/.rel.data в финальный бинарь: они нужны только для этапа статической линковки.


.plt	Создаётся линкером. Содержит «кадры»‑заглушки для вызовов внешних функций (helper@plt).
.got	Резервируется линкером под адреса внешних переменных и служебных данных.
.got.plt	Резервируется под слоты для PLT‑вызовов (по одному 8‑байтному слоту на функцию).
.rela.plt	Таблица релокаций типа R_X86_64_JUMP_SLOT — говорит, какие ячейки .got.plt обновлять.
.rela.dyn	Все остальные динамические релокации: GOTPCREL, абсолютные адреса и т. п.


.got.plt — GOT специально для PLT
Это часть GOT, которая хранит адреса функций, вызываемых через .plt

Ключевая особенность: именно эти ячейки обновляются по релокациям типа `R_X86_64_JUMP_SLOT` в `.rela.plt`
`.rela.plt` — релокации для PLT (JUMP_SLOT)
Эти секции содержат список того, какие ячейки в .got.plt нужно обновить и каким символом. Тип релокации — обычно R_X86_64_JUMP_SLOT
Используется для Обновление ячеек в `.got.plt` при lazy binding
`.rela.dyn` — релокации для остальных данных (включая GOT для переменных)
Здесь перечислены релокации типа R_X86_64_GOTPCREL и другие, которые говорят, куда и какой адрес положить (в том числе в ячейки .got для переменных).
`.dynsym` — таблица динамических символов
В ней перечислены все внешние символы, которые программа использует (например, printf, fopen, global_var). Это справочник: по имени символа линковщик и загрузчик находят его тип, размер и т. д.
Роль в GOT/PLT: релокации ссылаются на индексы в .dynsym, чтобы понять, какой именно символ нужно положить в слот GOT.
`.dynamic` — динамические теги
Важные теги для GOT/PLT:

DT_PLTREL — тип релокаций для PLT (REL или RELA).
DT_PLTGOT — адрес таблицы GOT, которую использует PLT (часто это адрес начала .got.plt или объединённой GOT).
DT_JMPREL — смещение и размер секции релокаций PLT.
DT_SYMTAB, DT_STRTAB — ссылки на .dynsym и .dynstr.
DT_REL, DT_RELA — секции обычных динамических релокаций.

Роль: загрузчик (ld.so) читает эти теги, чтобы понять, где искать PLT, GOT, релокации и как их обрабатывать.


`.dynstr` — строки имён символов
Это просто строки (имена символов) для .dynsym. Без неё имена были бы непонятны человеку и инструментам.

Права доступа: секции .got и .got.plt в памяти должны быть записываемыми на этапе инициализации (чтобы линковщик мог их обновить). После этого может применяться RELRO, и тогда часть или вся GOT делается только для чтения.

RELRO (Relocation Read‑Only) — это механизм защиты ELF‑бинарников, который делает некоторые секции (в первую очередь GOT) доступными только для чтения после того, как динамический линковщик применил релокации. Главная цель — закрыть вектор атаки GOT overwrite: если злоумышленник получил возможность писать в память, он не сможет подменить адреса в GOT и перехватить вызовы функций.

# Два режима RELRO
Partial RELRO (частичный)
Включается по умолчанию во многих дистрибутивах флагом -Wl,-z,relro.

Что происходит:
Линкер помечает часть данных (обычно всё, кроме .got.plt) как read‑only после релокаций.
Секция .got.plt остаётся записываемой, чтобы работало ленивое связывание (lazy binding): при первом вызове функции динамический линковщик должен записать в неё реальный адрес.

.got.plt всё ещё можно перезаписать — атака возможна, если удастся эксплуатировать уязвимость до того, как символ уже резолвился.


Full RELRO (полный)
Включается парой флагов: -Wl,-z,relro -Wl,-z,now.

Что происходит:

-z,now заставляет динамический линковщик сразу при старте разрешить все символы (не лениво). То есть все ячейки GOT заполняются реальными адресами ещё до входа в main.

#
Как компилятор решает, какой вариант генерировать
Компилятор (GCC/Clang) на основе атрибутов символа и флагов выбирает:

Локальные/внутренние символы → прямой RIP‑relative.
Внешние/глобальные символы → чаще через GOTPCREL (чтобы поддержать переопределение символа в рантайме).
Функции → почти всегда через PLT (для lazy binding) или GOT (если отключён PLT).



disp32 = (адрес_символа) − (адрес_инструкции + размер_инструкции)
Почему + размер_инструкции? Потому что RIP в момент выполнения инструкции уже указывает на следующую инструкцию.

в момент выполнения инструкции регистр RIP уже указывает на следующую инструкцию. То есть «текущая позиция» для относительных расчётов — это адрес_инструкции + размер_инструкции.


RIP-относительная адресация (RIP-relative addressing) — это режим адресации в архитектуре x86-64, при котором адрес целевых данных в памяти вычисляется как смещение относительно текущего значения регистра RIP (Instruction Pointer). Иными словами, адрес равен: [RIP + смещение]

Этот механизм стал ключевым нововведением в x64 для создания позиционно-независимого кода (PIC — Position-Independent Code).
Позиционно-независимый код (PIC): Когда операционная система загружает исполняемый файл или динамическую библиотеку (.so или .dll), адрес загрузки в память может быть случайным (технология ASLR для безопасности). Если бы код использовал жестко прописанные абсолютные адреса, ОС приходилось бы пересчитывать и менять (патчить) каждую инструкцию при старте программы. С RIP-относительной адресацией расстояние между инструкцией кода и глобальной переменной в памяти всегда остается неизменным, куда бы ни загрузили программу.

RIP-относительный сдвиг (тоже 32-битный sign-extended) позволяет адресовать данные в диапазоне ±2 ГБ от текущей инструкции, чего практически всегда достаточно.

Важно помнить, что процессор считывает значение RIP как адрес СЛЕДУЮЩЕЙ инструкции, которая идет сразу за текущей.
Если инструкция mov находится по адресу 0x1000, а её длина составляет 7 байт, то во время её выполнения RIP будет равен 0x1007. Если переменная лежит по адресу 0x1020, то компилятор зашьет в тело инструкции смещение 0x15 (так как 0x1007 + 0x15 = 0x1020).



RIP-relative addressing is an addressing mode in the x86-64 architecture where the target memory address is calculated as an offset relative to the current value of the RIP register (Instruction Pointer).
Mathematically, the effective address is computed as: [RIP + signed 32-bit displacement].


offset = addr_xxx - (addr_call + len_call)
Это число — не абсолютный адрес, а разница. И вот самое важное: когда линкер сливает все .text в один большой .text, он просто приклеивает куски друг за другом. При этом разница между любыми двумя адресами внутри одного куска не меняется. 

Допустим, в b.o (где определена xxx):

инструкция call xxx находится по смещению 0x10;
функция xxx начинается по смещению 0x40;
длина инструкции call — 5 байт.

Тогда компилятор вычисляет:
delta = 0x40 - (0x10 + 5) = 0x2B

И кладёт 0x2B прямо в машинный код call.


Теперь линкер берёт:

.text из a.o длиной 0x300 байт;
.text из b.o и приклеивает его сразу после a.o.
Значит, весь код из b.o сдвигается на 0x300. Теперь:

реальный адрес вызова: base + 0x300 + 0x10;
реальный адрес xxx: base + 0x300 + 0x40.
Разница между ними:
(base + 0x300 + 0x40) - (base + 0x300 + 0x10 + 5) = 0x40 - 0x10 - 5 = 0x2B

Она точно такая же, как и посчитал компилятор. Поэтому линкеру не надо ничего менять в этой инструкции.


Где тогда нужны релокации?
Релокации требуются именно там, где нельзя заранее посчитать разницу, потому что один из адресов неизвестен на этапе компиляции. Например:

Вызов внешней функции (extern). Компилятор не знает, где будет foo, поэтому оставляет релокацию: «подставь сюда адрес foo позже».
Сохранение адреса в данных. Например, void (*f)() = xxx;. Здесь в памяти лежит не команда перехода, а само значение адреса. Это значение должно быть абсолютным (или через GOT/PLT), и его надо пересчитать с учётом финального базового адреса секции.
Инструкции, требующие абсолютного адреса. Некоторые архитектуры или режимы (например, загрузка адреса в регистр) не умеют делать PC‑relative и требуют явного адреса — для них тоже нужна релокация.

Why is it needed?

1. Position-Independent Code (PIC): When an operating system loads an executable or a shared library (.dll or .so), it often places it at a randomized memory location for security (a feature called ASLR). If the code used absolute (fixed) addresses, the OS would have to patch every single memory-referencing instruction at startup. With RIP-relative addressing, the distance between the code instruction and the global data remains constant, no matter where the program is loaded in memory.
2. Handling 64-bit Address Space: Full 64-bit pointers take up 8 bytes of space, which would make machine code instructions massive. To save space, x86-64 instructions use 32-bit displacements. An absolute 32-bit address can only reach the lowest 2 GB of memory. A RIP-relative 32-bit signed offset, however, allows instructions to access any data within ±2 GB from the current instruction, which is more than enough for almost all applications.
For example:
• An instruction sits at address `0x1000`.
• The length of this instruction is `7` bytes.
• While executing it, the CPU treats RIP as `0x1007`.
• If your variable is located at address `0x1020`, the compiler will calculate and encode a displacement of `0x15` (since `0x1007 + 0x15 = 0x1020`).



PIC works by using relative addressing instead of absolute addressing. Absolute addressing is when the code refers to specific memory addresses directly. Relative addressing, on the other hand, is when the code refers to memory addresses relative to the current program counter (PC).

The PC(program counter) is a register that contains the address of the next instruction to be executed(similar to rip; when debugging) by CPU. When a program is running, the PC is incremented after each instruction is executed. This means that the PC always contains the address of the next instruction to be executed, regardless of where the code is loaded in memory.

This means that all addresses in the PIC code are relative to a known base address.



Relative addressing: gcc uses relative addressing instead of absolute addressing whenever possible. This means that all addresses in the code are relative to the program counter (PC). This allows the code to be relocated to any memory address without having to be modified.


PIE (Position‑Independent Executable) и PIC (Position‑Independent Code): вместо абсолютных адресов часто используют PC‑relative адреса или таблицы GOT/PLT. Релокации всё равно есть, но формулы другие (например, «адрес символа минус текущий RIP»).
Динамическая линковка: часть релокаций откладывается до загрузки программы динамическим линковщиком (ld.so). Они тоже используют таблицы релокаций из ELF.


3. Позиционно‑независимый код (PIC/PIE)
В режиме -fPIC/-fPIE компилятор намеренно избегает абсолютных адресов. Вместо этого он адресует символы через GOT/PLT, и тогда создаются релокации типа R_X86_64_GOTPCREL и т.п., даже если в итоге символ окажется в той же секции.

-fPIC — это как раз гарантия, что компилятор будет использовать RIP‑relative и GOT‑доступ вместо абсолютных адресов.



Когда компилятор (например, gcc/clang) генерирует объектный файл (.o, формат ELF), он:

Разбивает код и данные на секции: .text, .data, .bss и т.д.
Внутри каждой секции символы получают относительные адреса — смещения от начала секции. Для секции это действительно выглядит как будто начало имеет адрес 0x00000000 внутри объектного файла.
Если код ссылается на символ из другого файла (например, вызов функции foo()), компилятор не знает его абсолютный адрес. Он оставляет «дырку» и создаёт запись релокации: «тут должен быть адрес символа foo, но он будет известен позже».
В ELF такие записи хранятся в секциях типа .rel.text / .rela.text. В них указано:

смещение внутри секции, где нужно подставить адрес;
тип релокации (как именно подставлять: просто адрес, адрес + смещение, PC-relative и т.п.);
на какой символ ссылаться (или какой раздел).



Объединяет секции с одинаковыми именами из разных объектных файлов в одну итоговую секцию. Например, все .text складываются в один большой .text в финальном ELF.
Назначает финальные адреса секциям согласно скрипту линковки (linker script) или по умолчанию. Например, секция .text может получить базовый адрес 0x400000.
Проходит по всем записям релокаций и вычисляет нужные значения:
Для внешних символов: берёт их финальный адрес из таблицы символов и применяет формулу, заданную типом релокации.
Для символов внутри той же секции: использует их смещение от начала объединённой секции плюс базовый адрес секции.




disp32 — это сокращение от displacement 32‑bit («смещение 32 бита»). В контексте x86‑64 и RIP‑relative адресации это 4‑байтное знаковое смещение,
disp32
=
𝑆
+
𝐴
−
𝑃
disp32=S+A−P
Где:

𝑆
S — финальный адрес символа (переменной/функции),
𝐴
A — addend из записи релокации (часто 0, но может быть и другим, например, при доступе к полю структуры),
𝑃
P — адрес, который будет в RIP в момент выполнения, то есть r_offset + размер_инструкции.

disp32 — это не абсолютный адрес. В инструкции нет адреса переменной, только разница между адресом переменной и тем, где будет RIP. Именно поэтому код остаётся позиционно‑независимым


через RIP‑relative к переменной из shared library обратиться нельзя. Если ты напишешь mov global_var(%rip), линковщик либо откажется это делать, либо (в некоторых конфигурациях) сделает доступ к символу в своей секции, а не в библиотеке — и это будет ошибка.

Вместо этого используют косвенный доступ через GOT (Global Offset Table). И именно так RIP‑relative адресация всё‑таки участвует — но она ведёт не на переменную, а на ячейку GOT.


`R_X86_64_PC32` (прямой RIP‑relative доступ)
  Статический линковщик выполняет релокацию
`R_X86_64_GOTPCREL` (доступ до GOT ячейки)
  Статический линковщик выполняет релокацию
  Используется для внешних переменных, чтобы поддержать переопределение символа:
  disp32=адрес_ячейки_в_GOT−(адрес_инструкции+размер)

mov    global_var@GOTPCREL(%rip), %rax   ; RIP‑relative доступ к ячейке GOT
mov    (%rax), %eax                      ; чтение значения переменной
`R_X86_64_64`
  динамический линковщик (в GOT)

R_X86_64_GOTPCREL + R_X86_64_64


`R_X86_64_GOTPCREL` — главный для доступа к внешним переменным
Что делает: вычисляет 32‑битное смещение до ячейки GOT, которая хранит адрес символа.
disp32=(адрес_ячейки_GOT + addend) − (r_offset+размер_инструкции)
Когда появляется: при обращении к extern‑переменным в PIC/PIE‑коде.
Кто применяет: Статический линковщик
Ключевой смысл
`R_X86_64_PC32` — прямой RIP‑relative доступ (локальные и внутренние символы)
Что делает: 32‑битное знаковое смещение напрямую до символа.
Кто применяет: Статический линковщик
disp32
=
(
𝑆
+
𝐴
)
−
𝑃
disp32=(S+A)−P
где 
𝑆
S — адрес символа, 
𝐴
A — addend, 
𝑃
P — адрес, который будет в RIP (r_offset + размер инструкции).

R_X86_64_64 — абсолютный 64‑битный адрес (для GOT и динамических релокаций)
Что делает: записывает полный 64‑битный адрес символа в указанное место.

Важно: это не про инструкции, а про ячейки в секциях .got, .got.plt, .data, .bss и т. д.

Типичный случай: в ячейку GOT нужно положить реальный адрес внешней переменной. Эту работу делает динамический линковщик (ld.so) при загрузке, используя релокации типа R_X86_64_64 из секции .rela.dyn.


R_X86_64_PLT32
для вызовов внешних функций (через PLT)
Что делает: RIP‑relative смещение до записи в PLT (Procedure Linkage Table).

R_X86_64_GOTOFF64
смещение от начала GOT до символа (абсолютное в пределах GOT). Используется, например, для доступа к своим (не внешним) данным относительно GOT.

R_X86_64_GOTPC32: смещение до самого GOT относительно RIP. Часто встречается в низкоуровневом коде, который «знает» про GOT.


R_X86_64_RELATIVE — «базовые» релокации для ASLR и загрузки
Что делает: просто добавляет базовый адрес загрузки образа к значению.

R_X86_64_JUMP_SLOT
динамические привязки для функций
Это тип релокации в динамической таблице (.rela.dyn), который говорит: «в эту ячейку (обычно в GOT) нужно записать адрес функции».


Сводная таблица: кто за что отвечает
|Тип релокации|Размер смещения|Где применяется|Кто применяет|Ключевой смысл|
|:-|:-|:-|:-|:-|
|R_X86_64_GOTPCREL|32‑бит (disp32)|Инструкции: доступ к внешним переменным|Статический линковщик|RIP‑relative до ячейки GOT|
|R_X86_64_PC32|32‑бит|Инструкции: локальные/внутренние символы|Статический линковщик|Прямой RIP‑relative|
|R_X86_64_64|64‑бит|Ячейки GOT, данные|Динамический линковщик (и иногда статический)|Абсолютный адрес символа|
|R_X86_64_PLT32|32‑бит|Вызовы функций через PLT|Статический линковщик|RIP‑relative до PLT|
|R_X86_64_GOTOFF64|64‑бит|Доступ к данным относительно GOT|Статический линковщик|Смещение внутри GOT|
|R_X86_64_GOTPC32|	32‑бит|Код, работающий с GOT напрямую|Статический линковщик|RIP‑relative до GOT|
|R_X86_64_RELATIVE|Зависит|Смещения, зависящие от базы образа|Динамический линковщик|Коррекция на базовый адрес|
|R_X86_64_JUMP_SLOT|Обычно 64‑бит|Записи GOT для функций|Динамический линковщик|Привязка адреса функции|



Есть 3 объектных файла a.o b.o c.o, в каждом есть .text и .rel.text. момле слияния секций воедино как линковщик поймет какие записи релокаций к каким секциям относятся?

Каждая запись релокации в ELF привязана к конкретной входной секции. В структуре Elf64_Rela нет явного имени секции, но линковщик при чтении объектного файла хранит для каждой секции:

её имя (.text из a.o),
её индекс в таблице секций,
её размер и выравнивание,
и самое главное: все релокации, относящиеся к этой секции, он хранит вместе с ней.

То есть внутренне у линковщика получается что-то вроде:

И фиксирует для каждой входной секции её смещение внутри итоговой секции:

Шаг 2: Слияние секций и вычисление новых смещений
Линковщик собирает итоговый .text так:
Итоговый .text = [a.o .text] + [b.o .text] + [c.o .text]
               = [0x000..0x100] + [0x100..0x200] + [0x200..0x300]

a.o .text → начинается с 0x000 (относительно начала итогового .text)
b.o .text → начинается с 0x100
c.o .text → начинается с 0x200
Это смещение линковщик хранит явно и использует для пересчёта r_offset.

Шаг 3: Пересчёт r_offset для каждой релокации
У каждой релокации есть r_offset — это смещение внутри входной секции. Чтобы применить релокацию в итоговом файле, линковщик делает:

новый_r_offset
=
смещение_входной_секции_в_итоге
+
старый_r_offset
новый_r_offset=смещение_входной_секции_в_итоге+старый_r_offset

Пример:

В b.o есть релокация с r_offset = 0x28 (внутри .text b.o).
Мы знаем, что .text из b.o начинается в итоговом .text с адреса 0x100.
Значит, в итоговом бинаре эту релокацию надо применять по адресу 0x128.
Так линковщик получает актуальные позиции для всех релокаций в итоговой секции.


Шаг 4: Применение релокаций или перенос в .rela.dyn
Дальше линковщик действует по типу символа:

Если символ разрешён (определён в одном из входных файлов)
Он:

Берёт итоговый адрес символа (из таблицы символов, уже посчитанный как base_of_section + offset).
Подставляет в формулу релокации (например, S + A - P для R_X86_64_PC32).
Записывает результат прямо в байты итогового .text по новому_r_offset.
Не копирует эту запись в выходной файл: она уже отработана.
Если символ внешний (например, printf из libc)
Тогда линковщик:

Пересчитывает r_offset как выше.
Оставляет запись релокации в новой таблице (обычно в .rela.dyn или .rela.plt).
Теперь эта запись ссылается на итоговую позицию в итоговом .text, а не на старую в b.o.


# Addend
Примеры, где addend ≠ 0
struct S {
    int a;
    int b;
};

extern struct S s;

int get_b(void) {
    return s.b;
}

Компилятор сгенерирует RIP‑relative доступ к s, но со смещением на 4 байта (потому что b — второе поле). Релокация будет типа R_X86_64_GOTPCREL, а addend = 4.

Формула линковщика:

disp32=(адрес_ячейки_GOT+4)−(адрес_RIP)
disp32=(адрес_ячейки_GOT+4)−(адрес_RIP)
То есть addend учитывает смещение внутри структуры.

[27.10.2026 00:24] Anton Romanov: Указатель с начальным смещением
extern int arr[10];
int *p = arr + 3;  // указатель на arr[3]
[27.10.2026 00:24] Anton Romanov: В .data будет лежать 8‑байтовое значение, которое должно стать адресом arr+3. Релокация R_X86_64_64 будет иметь addend = 12 (если int = 4 байта, то 3 * 4 = 12).

Динамический линковщик при загрузке запишет в эту ячейку: адрес_arr + 12.
[27.10.2026 00:25] Anton Romanov: Частый случай: addend = 0
Во многих простых обращениях addend равен нулю:
[27.10.2026 00:25] Anton Romanov: extern int x;
x += 1;
[27.10.2026 00:25] Anton Romanov: Релокация R_X86_64_GOTPCREL будет с addend = 0, потому что мы обращаемся к самому символу без смещения. Формула упрощается до:

disp32
=
адрес_GOT_ячейки
−
адрес_RIP
disp32=адрес_GOT_ячейки−адрес_RIP
Именно поэтому в выводе readelf часто видишь + 0.