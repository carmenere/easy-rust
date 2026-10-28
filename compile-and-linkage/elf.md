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
  - [Addends](#addends)
- [ELF segments](#elf-segments)
  - [Segment types](#segment-types)
  - [Segment Permissions](#segment-permissions)
- [Relocation](#relocation)
  - [Шаг 2: Слияние секций и вычисление новых смещений](#шаг-2-слияние-секций-и-вычисление-новых-смещений)
  - [Шаг 3: Пересчёт r\_offset для каждой релокации](#шаг-3-пересчёт-r_offset-для-каждой-релокации)
- [новый\_r\_offset](#новый_r_offset)
  - [Шаг 4: Применение релокаций или перенос в .rela.dyn](#шаг-4-применение-релокаций-или-перенос-в-reladyn)
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
  - this section holds the **symbol table** with symbols needed for **dynamic linking**;
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

## Addends
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
То есть addend учитывает смещение внутри структуры.

Указатель с начальным смещением
extern int arr[10];
int *p = arr + 3;  // указатель на arr[3]
В .data будет лежать 8‑байтовое значение, которое должно стать адресом arr+3. Релокация R_X86_64_64 будет иметь addend = 12 (если int = 4 байта, то 3 * 4 = 12).

Динамический линковщик при загрузке запишет в эту ячейку: адрес_arr + 12.
Частый случай: addend = 0
Во многих простых обращениях addend равен нулю:
extern int x;
x += 1;

Релокация R_X86_64_GOTPCREL будет с addend = 0, потому что мы обращаемся к самому символу без смещения. Формула упрощается до:

disp32 = адрес_GOT_ячейки − адрес_RIP
Именно поэтому в выводе readelf часто видишь + 0.

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

# Relocation
`.rel.text` и `.rel.data` существуют только в перемещаемых объектных файлах (.o), а не в финальных исполняемых файлах или shared-библиотеках. В готовом PIE-бинаре ты их не увидишь — там вместо них будут `.rela.dyn` и `.rela.plt`.
В объектном файле (main.o) линковщик ещё не знает финальных адресов символов (они станут известны только при сборке всего проекта). Поэтому ассемблер оставляет «дырки» в коде и данных и создаёт таблицы релокаций, чтобы линкер потом подставил туда правильные адреса.

`.rel.text` — хранит релокации для инструкций в секции .text (вызовы функций, относительные переходы, обращения к символам из кода).
`.rel.data` — хранит релокации для инициализированных данных в секции .data (например, когда глобальная переменная инициализируется адресом другой функции/переменной).
Их главная задача: сообщить линковщику, где именно в коде/данных нужно подставить адрес символа и по какой формуле.

Линкер:
Читает `.rel.text`, `.rel.data` (и другие .rel.*) из каждого .o.
Применяет статические релокации: подставляет адреса символов, которые уже известны внутри проекта.
Формирует динамические релокации в `.rela.dyn` и `.rela.plt` для тех символов, которые разрешаются при загрузке (например, printf из libc).
Не копирует старые `.rel.text`/`.rel.data` в финальный бинарь: они нужны только для этапа статической линковки.

Есть 3 объектных файла `a.o` `b.o` `c.o`, в каждом есть `.text` и `.rel.text`. момле слияния секций воедино как линковщик поймет какие записи релокаций к каким секциям относятся?

Каждая запись релокации в ELF привязана к конкретной входной секции. В структуре Elf64_Rela нет явного имени секции, но линковщик при чтении объектного файла хранит для каждой секции:

её имя (`.text` из `a.o`),
её индекс в таблице секций,
её размер и выравнивание,
и самое главное: все релокации, относящиеся к этой секции, он хранит вместе с ней.

То есть внутренне у линковщика получается что-то вроде:

И фиксирует для каждой входной секции её смещение внутри итоговой секции:

## Шаг 2: Слияние секций и вычисление новых смещений
Линковщик собирает итоговый .text так:
Итоговый .text = [a.o .text] + [b.o .text] + [c.o .text]
               = [0x000..0x100] + [0x100..0x200] + [0x200..0x300]

`a.o .text` → начинается с `0x000` (относительно начала итогового .text)
`b.o .text` → начинается с `0x100`
`c.o .text` → начинается с `0x200`
Это смещение линковщик хранит явно и использует для пересчёта `r_offset`.

## Шаг 3: Пересчёт r_offset для каждой релокации
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


## Шаг 4: Применение релокаций или перенос в .rela.dyn
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

<br>
