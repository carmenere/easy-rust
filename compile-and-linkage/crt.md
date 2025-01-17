# Table of contents
- [Table of contents](#table-of-contents)
- [C standard library](#c-standard-library)
- [C runtime (CRT)](#c-runtime-crt)
  - [Program entry point](#program-entry-point)
  - [Initialization and termination routines](#initialization-and-termination-routines)
- [In Rust](#in-rust)

<br>

# C standard library
The **C standard library** (aka **ISO C library** or **libc**) is the **standard library** for the **C programming language**, as specified in the **ISO C standard**.<br>
Some of the popular **implementations** of *C standard library*:
- **BSD libc**;
- **glibc** (GNU C Library);
- **musl** (**lightweight** implementation of **libc** for Linux systems);

<br>

# C runtime (CRT)
**CRT** stands for **C runtime**.<br>

There is a very important difference between **C standard library** and **CRT**:
- the **C standard library** defines functions that are available to the programmer;
- **CRT** is a thin layer of code compiled in binary that contains **startup routine** and **error handling** code;

<br>

The **CRT** is an **object file**, **not** library. For example, you could write a program which **does not** use the **C standard library** but you always need the **CRT** because otherwise, your program could **not** be executed. So, **CRT** is automatically linked into binary by the compiler.<br>

<br>

The work performed by **CRT** depends on the **language**, **compiler**, OS and **C standard library** implementation. Ususally **CRT** performs following work:
- **before** calling the `main`:
  - initializes the **stack**;
  - pushes `argc` and `argv` onto the **stack**;
  - initializes the `.bss` section to zero;
  - initializes the `heap`;
  - **wraps** the `main()` by `exit()`: `exit(main(argc, argv, envp));`;
- **after** `main` returns:
  - pops `argc` and `argv` from the **stack**;
  - stores the **return code** from `main()` in `eax`;
  - calls `exit()`;

<br>

The **CRT** is shipped as part of the **OS** and **compiler**, for example, in Linux:
- **OS**:
  - `/usr/lib/x86_64-linux-gnu/crt1.o`
  - `/usr/lib/x86_64-linux-gnu/crti.o`
  - `/usr/lib/x86_64-linux-gnu/crtn.o`
- **gcc**:
  - `/usr/lib/gcc/x86_64-linux-gnu/13/crtbegin.o`
  - `/usr/lib/gcc/x86_64-linux-gnu/13/crtend.o`

<br>

## Program entry point
When an executable is loaded in the memory, then OS calls **entrypoint** of the binary, which is the symbol `_start` in the GCC toolchain.<br>
This `_start` **entrypoint** runs some **setup code** required **before** calling the `main` function and some **cleanup** code required **after** `main` returns.<br>

The `crt0.o` object file will contain the `_start` function that **initializes** the process and calls `exit(main(argc, argv))`.<br>

<br>

## Initialization and termination routines
The **initialization code** is executed **once** before `main` each time the library or binary is loaded in a process.<br>
The **termination code** is executed **once** each time the library or binary is unloaded from a process or at process termination.<br>

There are 2 special attributes to mark functions:
- `__attribute__ ((constructor (priority)))`;
- `__attribute__((destructor (priority)))`;

<br>

**Constructors** are placed in `.init_array` section.<br>
**Destructors** are placed in `.fini_array` section.<br>

<br>

The `priority` controls the **order** in which **constructors** and **destructors** functions are called.<br>
Priorities from `0` to `100` are **reserved**. Applications can use `101` to `65535`.<br>
A constructor with a **smaller** `priority` value runs **before** a constructor with a **larger** `priority` value; the **opposite** relationship holds for **destructors**.<br>

<br>

**Example**:
```c
__attribute__ ((constructor (101))) void foo()
{
        (void) printf("initializing: foo()\n");
}

__attribute__((destructor (101))) void bar()
{
        (void) printf("finalizing: bar()\n");
}
```

<br>

There are 2 symbols `_init` and `_fini` that call of such **constructors** and **destructors** functions.<br>
The `_init` is placed in `.init` section and `_fini` is placed in `.fini` section.<br>

<br>

The `_init` and `_fini` are assembled by the linker from number of files `crti.o`, `crtn.o`, `crtbegin.o` and `crtend.o`. The idea is that files are linked in this **order**: `crt0.o` `crti.o` `crtbegin.o` `foo.o` `bar.o` `crtend.o` `crtn.o`.<br>

- `crti.o` contains **prologues** for `_init` and `_fini` functions;
- `crtn.o` contains **epilogues** for `_init` and `_fini` functions;
- `crtbegin.o` contains the necessary instructions that call **constructors** from the `.init_array` section;
- `crtend.o` contains the necessary instructions that call **constructors** from the `.fini_array` section;

<br>

The `crtbegin.o` and `crtend.o` files form **body** for `_init` and `_fini` functions.<br>

<br>

**Example** of implementation of `crti.s` for x86_64:
```bash
_init:
   push %rbp
   movq %rsp, %rbp
   /* gcc will nicely put the contents of crtbegin.o .init section here. */

_fini:
   push %rbp
   movq %rsp, %rbp
   /* gcc will nicely put the contents of crtbegin.o .fini section here. */
```

<br>

**Example** of implementation of `crtn.s` for x86_64:
```bash
   /* gcc will nicely put the contents of crtend.o .init section here. */
   popq %rbp
   ret

.section .fini
   /* gcc will nicely put the contents of crtend.o .fini section here. */
   popq %rbp
   ret
```

<br>

# In Rust
In a **typical** Rust **binary** that links the **standard library**, **execution starts** in a **CRT**. This creates a stack and places the arguments in the right registers.
Then **crt0** invokes the [**entry point**](https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73) of the **Rust runtime**.<br>
