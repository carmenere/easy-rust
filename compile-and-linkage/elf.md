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
<!-- TOC -->

<br>

# ELF
The **last version** of **ELF specification** is **Version 4.3 DRAFT** and it was published in **2025** by **Xinuos, Inc**: https://gabi.xinuos.com/.<br>
Now **Xinuos, Inc** maintains the official specification for the **ELF gABI** (*generic Application Binary Interface*). **Xinuos, Inc** is the successor to the assets of **The SCO Group** and **UnixWare**.<br>
The ELF specification was formerly part (chapters **4** and **5**) of the **SVR4 gABI** document (aka *UNIX System V Release 4 gABI*), both was published in March **1997**.<br>

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
- `e_machine` specifies the **required architecture** for an individual file;
- `e_version`  identifies the **object file version**;
- `e_entry` contains **entry point**, i.e. the **virtual address to which transfers control**, thus starting the process, if the file has no associated entry point, this member holds zero;
- `e_phoff` the **byte offset** from the beginning of the file to the *program header table*;
- `e_phentsize` holds the **size in bytes of one entry** in the *program header table*;
  - a **program header** is **one entry** in the *program header table*;
  - **all entries** are the **same size**;
- `e_phnum` holds the **number of entries** in the *program header table*;
- `e_shoff` the **byte offset** from the beginning of the file to the *section header table*;
- `e_shentsize` holds the **size in bytes of one entry** in the *section header table*;
  - a **section header** is **one entry** in the *section header table*;
  - **all entries** are the **same size**;
- `e_shnum`
  - holds the **number of entries** in the *section header table*;
- `e_flags` holds processor-specific flags associated with the file;
- `e_ehsize` holds the **ELF header's size** in bytes;
- `e_shstrndx` holds the *section header table index* of the entry associated with the **section name string table**;

<br>

### `e_ident` field specification
The `e_ident` field **differs** for **ELF32** and **ELF64**: **ELF64** adds 2 bytes `EI_OSABI` and `EI_ABIVERSION` with padding `EI_PAD` at index **9**.<br>

|Name|ELF32 index|ELF64 index|Purpose|
|:---|:-------|:----|:------|
|`EI_MAG0`|0|0|**File identification**: `0x7f`|
|`EI_MAG1`|1|1|**File identification**: `E`|
|`EI_MAG2`|2|2|**File identification**: `L`|
|`EI_MAG3`|3|3|**File identification**: `F`|
|`EI_CLASS`|4|4|**File class** determines **type of ELF**: **ELF32** or **ELF64**|
|`EI_DATA`|5|5|**Data encoding**: **LSB** or **MSB**|
|`EI_VERSION`|6|6|**File version**|
|`EI_OSABI`|-|7||
|`EI_ABIVERSION`|-|8||
|`EI_PAD`|7|9|This value marks the beginning of the unused bytes in `e_ident[]`|
|`EI_NIDENT`|16|16|**Size of** `e_ident[]`|

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

- `sh_name` this member specifies the **name of the section**;
- `sh_type` holds **type of section** which defines the section’s contents and semantics;
- `sh_offset` holds the **byte offset** from the beginning of the file to the first byte in the section;
- `sh_size` holds the **size of section** in bytes, i.e. number of bytes section occupies in the object file;
  - the section of `SHT_NOBITS` type **occupies no space in the file**, **but** it can have a **non-zero size in memory**, it is for `.bss` segment;
- `sh_info` this member holds extra information, whose interpretation depends on the section type. 
- `sh_entsize` some sections hold a **table of fixed-size entries**, such as a **symbol table**, for such a section, this member gives the **size in bytes of each entry**;

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
**DWARF** is a widely used, standardized **debugging data format**. **Version 5** of the **DWARF** format was published in February **2017**.<br>
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
  - holds the **value** of the associated symbol;
- `st_size`
  - holds **sizes**;
- `st_info`
  - specifies the **symbol’s type** and **binding attributes**;

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
  - the symbol is associated with a **section**. Symbol table entries of this type exist primarily for relocation and normally have STB_LOCAL binding.
- `STT_FILE`
  - the symbol’s name gives the name of the source file associated with the object file;
  - A file symbol has `STB_LOCAL` binding;
- `STT_COMMON`
  - the symbol labels an uninitialized common block. See below for details.
- `STT_TLS`
  - the symbol specifies a Thread-Local Storage entity

<br>

# ELF relocation entries
There are **two** different **relocation entry structures**: `Elfxx_Rel` and `Elfxx_Rela`.<br>
The only **difference** between `Elfxx_Rel` and `Elfxx_Rela` is that `Elfxx_Rela` contains an explicit *addend* in relocation entry whereas `Elfxx_Rel` store an *addend* in the location to be modified and linker must read *addend* **before** in perform relocation action.<br>
On `x86_64` only `Elf64_Rela` is used.<br>

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

**Type of addres**:
-	`R_386_32`
-	`R_386_PC32`

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
- `.rel.got`
- `.rel.plt`

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
- `p_offset` holds the **byte offset** from the beginning of the file to the first byte of the segment;
- `p_vaddr`
  - holds **virtual address** at which the first byte of the segment resides in memory;
- `p_filesz`
  - holds **number of bytes** in the **file image** of the segment;
  - it may be zero;
- `p_memsz`
  - holds **number of bytes** in the **memory image** of the segment;
  - it may be zero;
- `p_flags`
  - holds **flags** relevant to the segment. Defined flag values appear below;

<br>

## Segment types
Segment types, `p_type`:
- `PT_NULL`
  - means **unused egment**;
  - this type **lets** the *program header table* **have ignored entries**;
- `PT_LOAD`
  - specifies a **loadable segment**, described by the segment’s **memory size** (`p_memsz`) and the **file size** (`p_filesz`);
  - the bytes from the file are mapped to the beginning of the memory segment;
  - if the `p_memsz` is **larger** than the `p_filesz`, the **extra zero bytes** are added **after** the *segment’s initialized area*;
  - the `p_filesz` may not be larger than the `p_memsz`;
  - **loadable segment entries** in the *program header table* appear in **ascending order**, **sorted on** the `p_vaddr`;
- `PT_DYNAMIC`
  - specifies **dynamic linking information**;
- `PT_INTERP`
  - specifies the **location** and **size** of a **null-terminated path name** to invoke as an **interpreter**;
  - it may **not occur more than once** in a file;
  - if it is present, it **must precede** any loadable segment entry;
  - this segment type is *meaningful only for executable files*: when building an *executable file* that uses **dynamic linking**, the linker adds a segment of type `PT_INTERP` to an *executable file*, telling the system **to invoke the dynamic linker** as the program interpreter;
- `PT_SHLIB`
  - this segment type is reserved but **has unspecified semantics**;
- `PT_PHDR`
  - the array element, if present, specifies the location and size of the *program header table* itself
  - if it is present, it **must precede** any loadable segment entry.
  - this segment type may not occur more than once in a file.
- `PT_TLS`
  - specifies the **thread-local storage template**;
  - **thread-local storage segments** can be used to specify the **size** and **separate copy of initial data** allocated at compile-time for individual thread;

<br>

## Segment Permissions
Segment flag bits, `p_flags`:
- `PF_X` - **Execute**;
- `PF_W` - **Write**;
- `PF_R` - **Read**;

<br>

When the system creates loadable segments’ memory images, it gives **access permissions** as specified in the `p_flags` member.<br>
Although all flag combinations are valid, the system may grant more access than requested.<br>

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
