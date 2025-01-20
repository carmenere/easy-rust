# Table of contents
- [Table of contents](#table-of-contents)
- [GNU Binutils](#gnu-binutils)
- [`readelf`](#readelf)
- [`strip`](#strip)
- [`ldd`](#ldd)
- [`objdump`](#objdump)

<br>

# GNU Binutils
[**GNU Binutils**](https://sourceware.org/binutils/)<br>

<br>

# `readelf`
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
readelf --sections -W hello
There are 31 section headers, starting at offset 0x3780:

Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .interp           PROGBITS        0000000000000318 000318 00001c 00   A  0   0  1
  [ 2] .note.gnu.property NOTE            0000000000000338 000338 000030 00   A  0   0  8
  [ 3] .note.gnu.build-id NOTE            0000000000000368 000368 000024 00   A  0   0  4
  [ 4] .note.ABI-tag     NOTE            000000000000038c 00038c 000020 00   A  0   0  4
  [ 5] .gnu.hash         GNU_HASH        00000000000003b0 0003b0 000024 00   A  6   0  8
  [ 6] .dynsym           DYNSYM          00000000000003d8 0003d8 0000c0 18   A  7   1  8
  [ 7] .dynstr           STRTAB          0000000000000498 000498 00009d 00   A  0   0  1
  [ 8] .gnu.version      VERSYM          0000000000000536 000536 000010 02   A  6   0  2
  [ 9] .gnu.version_r    VERNEED         0000000000000548 000548 000030 00   A  7   1  8
  [10] .rela.dyn         RELA            0000000000000578 000578 0000f0 18   A  6   0  8
  [11] .rela.plt         RELA            0000000000000668 000668 000030 18  AI  6  24  8
  [12] .init             PROGBITS        0000000000001000 001000 00001b 00  AX  0   0  4
  [13] .plt              PROGBITS        0000000000001020 001020 000030 10  AX  0   0 16
  [14] .plt.got          PROGBITS        0000000000001050 001050 000010 10  AX  0   0 16
  [15] .plt.sec          PROGBITS        0000000000001060 001060 000020 10  AX  0   0 16
  [16] .text             PROGBITS        0000000000001080 001080 0001af 00  AX  0   0 16
  [17] .fini             PROGBITS        0000000000001230 001230 00000d 00  AX  0   0  4
  [18] .rodata           PROGBITS        0000000000002000 002000 00002e 00   A  0   0  4
  [19] .eh_frame_hdr     PROGBITS        0000000000002030 002030 000044 00   A  0   0  4
  [20] .eh_frame         PROGBITS        0000000000002078 002078 0000ec 00   A  0   0  8
  [21] .init_array       INIT_ARRAY      0000000000003d90 002d90 000010 08  WA  0   0  8
  [22] .fini_array       FINI_ARRAY      0000000000003da0 002da0 000010 08  WA  0   0  8
  [23] .dynamic          DYNAMIC         0000000000003db0 002db0 000200 10  WA  7   0  8
  [24] .got              PROGBITS        0000000000003fb0 002fb0 000050 08  WA  0   0  8
  [25] .data             PROGBITS        0000000000004000 003000 00004c 00  WA  0   0 32
  [26] .bss              NOBITS          0000000000004060 00304c 000048 00  WA  0   0 32
  [27] .comment          PROGBITS        0000000000000000 00304c 00002b 01  MS  0   0  1
  [28] .symtab           SYMTAB          0000000000000000 003078 0003f0 18     29  19  8
  [29] .strtab           STRTAB          0000000000000000 003468 0001fc 00      0   0  1
  [30] .shstrtab         STRTAB          0000000000000000 003664 00011a 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  D (mbind), l (large), p (processor specific)
```

<br>

**Program Headers** and Section to Segment mapping:
```bash
readelf --segments -W hello

Elf file type is DYN (Position-Independent Executable file)
Entry point 0x1080
There are 13 program headers, starting at offset 64

Program Headers:
  Type           Offset   VirtAddr           PhysAddr           FileSiz  MemSiz   Flg Align
  PHDR           0x000040 0x0000000000000040 0x0000000000000040 0x0002d8 0x0002d8 R   0x8
  INTERP         0x000318 0x0000000000000318 0x0000000000000318 0x00001c 0x00001c R   0x1
      [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
  LOAD           0x000000 0x0000000000000000 0x0000000000000000 0x000698 0x000698 R   0x1000
  LOAD           0x001000 0x0000000000001000 0x0000000000001000 0x00023d 0x00023d R E 0x1000
  LOAD           0x002000 0x0000000000002000 0x0000000000002000 0x000164 0x000164 R   0x1000
  LOAD           0x002d90 0x0000000000003d90 0x0000000000003d90 0x0002bc 0x000318 RW  0x1000
  DYNAMIC        0x002db0 0x0000000000003db0 0x0000000000003db0 0x000200 0x000200 RW  0x8
  NOTE           0x000338 0x0000000000000338 0x0000000000000338 0x000030 0x000030 R   0x8
  NOTE           0x000368 0x0000000000000368 0x0000000000000368 0x000044 0x000044 R   0x4
  GNU_PROPERTY   0x000338 0x0000000000000338 0x0000000000000338 0x000030 0x000030 R   0x8
  GNU_EH_FRAME   0x002030 0x0000000000002030 0x0000000000002030 0x000044 0x000044 R   0x4
  GNU_STACK      0x000000 0x0000000000000000 0x0000000000000000 0x000000 0x000000 RW  0x10
  GNU_RELRO      0x002d90 0x0000000000003d90 0x0000000000003d90 0x000270 0x000270 R   0x1

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

# `strip`
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

<br>

# `ldd`
`ldd` and `otool` prints all **shared libraries** required by binary.<br>

In MacOS: `otool -L <file>`.<br>
In Linux: `ldd <file>`.<br>

<br>

Each entry in output has 3 value:
•	**optional**: `libNAME.so.X` where `X` is a version of lib, `NAME` - name of lib;
•	**abs path** to library file;
•	**address** at which it is loaded ;

<br>

**Example**:
```bash
ldd /bin/cat
    linux-vdso.so.1 (0x00007ffd2d6d9000)
    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f6c5b1fb000)
    /lib64/ld-linux-x86-64.so.2 (0x00007f6c5b7f5000)
```

<br>

The **first enry** `linux-vdso.so.1` **doesn't** have corresponding file, it is the **vDSO** (virtual dynamic shared object).<br>
The **last enry** contain only **abs path** and **address**.<br>

<br>

# `objdump`
Mapping **symbols** to **sections**:
```bash
objdump -t hello

hello:     file format elf64-x86-64

SYMBOL TABLE:
0000000000000000 l    df *ABS*	0000000000000000              Scrt1.o
000000000000038c l     O .note.ABI-tag	0000000000000020              __abi_tag
0000000000000000 l    df *ABS*	0000000000000000              hello2.c
0000000000004048 l     O .data	0000000000000004              k.0
0000000000000000 l    df *ABS*	0000000000000000              crtstuff.c
00000000000010b0 l     F .text	0000000000000000              deregister_tm_clones
00000000000010e0 l     F .text	0000000000000000              register_tm_clones
0000000000001120 l     F .text	0000000000000000              __do_global_dtors_aux
0000000000004060 l     O .bss	0000000000000001              completed.0
0000000000003da8 l     O .fini_array	0000000000000000              __do_global_dtors_aux_fini_array_entry
0000000000001160 l     F .text	0000000000000000              frame_dummy
0000000000003d98 l     O .init_array	0000000000000000              __frame_dummy_init_array_entry
0000000000000000 l    df *ABS*	0000000000000000              crtstuff.c
00000000000021c0 l     O .eh_frame	0000000000000000              __FRAME_END__
0000000000000000 l    df *ABS*	0000000000000000
0000000000003db0 l     O .dynamic	0000000000000000              _DYNAMIC
0000000000002044 l       .eh_frame_hdr	0000000000000000              __GNU_EH_FRAME_HDR
0000000000003fb0 l     O .got	0000000000000000              _GLOBAL_OFFSET_TABLE_
0000000000001266 g     F .text	000000000000001a              my_init
0000000000000000       F *UND*	0000000000000000              __libc_start_main@GLIBC_2.34
0000000000000000  w      *UND*	0000000000000000              _ITM_deregisterTMCloneTable
0000000000004000  w      .data	0000000000000000              data_start
0000000000000000       F *UND*	0000000000000000              puts@GLIBC_2.2.5
0000000000004080 g     O .bss	0000000000000028              b
000000000000404c g       .data	0000000000000000              _edata
0000000000001183 g     F .text	000000000000001a              bar
0000000000001280 g     F .fini	0000000000000000              .hidden _fini
0000000000000000       F *UND*	0000000000000000              printf@GLIBC_2.2.5
0000000000001228 g     F .text	000000000000003e              my_start
0000000000004000 g       .data	0000000000000000              __data_start
0000000000000000  w      *UND*	0000000000000000              __gmon_start__
0000000000004008 g     O .data	0000000000000000              .hidden __dso_handle
0000000000002000 g     O .rodata	0000000000000004              _IO_stdin_used
0000000000001169 g     F .text	000000000000001a              foo
00000000000040a8 g       .bss	0000000000000000              _end
0000000000001080 g     F .text	0000000000000026              _start
0000000000004020 g     O .data	0000000000000028              a
0000000000004060 g       .bss	0000000000000000              __bss_start
000000000000119d g     F .text	000000000000008b              main
0000000000004050 g     O .data	0000000000000000              .hidden __TMC_END__
0000000000000000  w      *UND*	0000000000000000              _ITM_registerTMCloneTable
0000000000000000  w    F *UND*	0000000000000000              __cxa_finalize@GLIBC_2.2.5
0000000000001000 g     F .init	0000000000000000              .hidden _init
```
