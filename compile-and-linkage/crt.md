# Table of contents
- [Table of contents](#table-of-contents)
- [C standard library](#c-standard-library)
- [Linux ABI](#linux-abi)
- [C runtime (CRT)](#c-runtime-crt)
  - [Program entry point](#program-entry-point)
  - [Initialization and termination routines](#initialization-and-termination-routines)
- [Custom code inside .init](#custom-code-inside-init)
- [Custom entry point](#custom-entry-point)
- [In Rust](#in-rust)

<br>

# C standard library
The **C standard library** (aka **ISO C library** or **libc**) is the **standard library** for the **C programming language**, as specified in the **ISO C standard**.<br>
Some of the popular **implementations** of *C standard library*:
- **BSD libc**;
- **glibc** (GNU C Library);
- **musl** (**lightweight** implementation of **libc** for Linux systems);

<br>

# Linux ABI
**Flow of loading programm in linux**:<br>

![linux_binary_loading_flow](/img/linux_binary_loading_flow.png)

<br>

- `_start`:
  - *calls* `__libc_start_main`:
    - *calls* `__libc_csu_init`:
      - *calls* `_init`:
        - *calls* `__gmon_start__`;
        - then *calls* `frame_dummy`;
        - then *calls* `__do_global_ctors_aux`:
          - *calls* all **constructors** from `.init_array`;
    - then *calls* `exit(main)`
    - `exit`:
      - *receives* **code** from `main`;
      - then *calls* `__do_global_dtors_aux` which *calls* all **destructors** from `.fini_array` section;

<br>

[**More details here**](http://dbp-consulting.com/tutorials/debugging/linuxProgramStartup.html).<br>

<br>

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

The **CRT** is shipped as part of the **OS** and **compiler**.<br>

**CRT files** shipped with Linux:
  - `/usr/lib/x86_64-linux-gnu/crt1.o`;
  - `/usr/lib/x86_64-linux-gnu/Scrt1.o` (by default);
  - `/usr/lib/x86_64-linux-gnu/crti.o`;
  - `/usr/lib/x86_64-linux-gnu/crtn.o`;

<br>

**CRT files** shipped with `gcc`:
  - `/usr/lib/gcc/x86_64-linux-gnu/13/crtbegin.o`;
  - `/usr/lib/gcc/x86_64-linux-gnu/13/crtend.o`;

<br>

**Meaning**:
- `crt1.o` contains the `_start` function that **initializes** the process and calls `exit(main(argc, argv))`;
- `crti.o` contains **prologues** for `_init` and `_fini` functions;
- `crtn.o` contains **epilogues** for `_init` and `_fini` functions;
- `crtbegin.o` and `crtend.o` form **body** for `_init` and `_fini` functions:
  - `crtbegin.o` contains code that calls **constructors** from the `.init_array` section;
  - `crtend.o` contains code that calls **constructors** from the `.fini_array` section;

<br>

When **PIE** is enabled the `Scrt1.o` is used. When **PIE** is disabled (`-no-pie`) the `crt1.o` is used.<br>
The  `crtstuff.c` contains source code for `crtbegin.o` and `crtend.o`.<br>
The `crtbegin.o` and `crtend.o` files form **body** for `_init` and `_fini` functions.<br>
The `_init` is placed in `.init` section and calls **constructors** that are reside in `.init_array` section.<br>
The `_fini` is placed in `.init` section and calls **destructors** that are reside in `.fini_array` section.<br>

<br>

At link time, **linker** places:
- `crt1.o` **before** `crti.o`;
- `crti.o` **before** `crtbegin.o`;
- `crtbegin.o` **before** any other relocatable file;
- `crtend.o` **after** any other relocatable file;
- `crtn.o` **after** `crtend.o`;

<br>

The order of linking **CRT files** with **other** relocatable file as follows:<br>
```
crt1.o crti.o crtbegin.o foo.o bar.o crtend.o crtn.o
```

<br>

```bash
objdump -t /usr/lib/gcc/x86_64-linux-gnu/13/crtbegin.o

/usr/lib/gcc/x86_64-linux-gnu/13/crtbegin.o:     file format elf64-x86-64

SYMBOL TABLE:
0000000000000000 l    df *ABS*	0000000000000000 crtstuff.c
0000000000000000 l    d  .text	0000000000000000 .text
0000000000000000 l    d  .bss	0000000000000000 .bss
0000000000000000 l    d  .tm_clone_table	0000000000000000 .tm_clone_table
0000000000000000 l     O .tm_clone_table	0000000000000000 __TMC_LIST__
0000000000000000 l     F .text	0000000000000000 deregister_tm_clones
0000000000000030 l     F .text	0000000000000000 register_tm_clones
0000000000000070 l     F .text	0000000000000000 __do_global_dtors_aux
0000000000000000 l     O .bss	0000000000000001 completed.0
0000000000000000 l     O .fini_array	0000000000000000 __do_global_dtors_aux_fini_array_entry
00000000000000a0 l     F .text	0000000000000000 frame_dummy
0000000000000000 l     O .init_array	0000000000000000 __frame_dummy_init_array_entry
0000000000000000         *UND*	0000000000000000 .hidden __TMC_END__
0000000000000000  w      *UND*	0000000000000000 _ITM_deregisterTMCloneTable
0000000000000000  w      *UND*	0000000000000000 _ITM_registerTMCloneTable
0000000000000000 g     O .data	0000000000000000 .hidden __dso_handle
```

<br>

## Program entry point
When an executable is loaded in the memory, then OS calls **entrypoint** of the binary, which is the symbol `_start` in the GCC toolchain.<br>
This `_start` **entrypoint** runs some **setup code** required **before** calling the `main` function and some **cleanup** code required **after** `main` returns.<br>

The `_start` function is provided by the **C runtime library** (**CRT**) to perform some initialization (like filing the .bss segment by zeros) and call our `main` function.<br>

<br>

## Initialization and termination routines
The **initialization code** is executed **once** before `main` each time the library or binary is loaded in a process.<br>
The **termination code** is executed **once** each time the library or binary is unloaded from a process or at process termination.<br>

<br>

**Constructors** (aka **ctors**) are placed in `.init_array` section.<br>
**Destructors** (**dtors**) are placed in `.fini_array` section.<br>

<br>

There are 2 special attributes to mark functions:
- `__attribute__ ((constructor (priority)))` marks function as **constructor**;
- `__attribute__((destructor (priority)))` marks function as **destructor**;

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

# Custom code inside .init
It is possible to put **custom** code inside `.init` section:
```c
#include <stdio.h>

void my_init()
{
  (void) printf("my_init()\n");
}

__asm__(
        ".section .init\n"
        "call my_init"
);

int main(int argc, char* argv[]){
  printf("main()\n");
}
```

<br>

# Custom entry point
There is option `-nostdlib` to **disable** `libc`. It means we **no** longer have access to all functions from `libc`. It also means we will lose the `_start` function from **CRT**.<br>
It is possible to provide custom **entry point** with `-Wl,-entry=<function_name>`. The *entry point* **doesn't** return anything so it has return type `void`.<br>

For writing to **stdout** there is `write` **syscall**. When calling the **syscall** instruction, we pass the **syscall number** in the **rax** register and the **arguments** in registers **rdi**, **rsi**, and **rdx**. For example, the `write` syscall has number **0x01** and the `exit` syscall has number **0x3c**.<br>

<br>

**Custom entry point**:
```c
int main() {
  volatile const char message[] = "main()\n";
  volatile const unsigned long length = sizeof(message) - 1;

  // write(1, message, length)
  asm volatile("mov $1, %%rax\n"                // write syscall number (0x01)
               "mov $1, %%rdi\n"                // Stdout file descriptor (0x01)
               "mov %0, %%rsi\n"                // Message buffer
               "mov %1, %%rdx\n"                // Buffer length
               "syscall"                        // Make the syscall
               :                                // No output operands
               : "r"(message), "r"(length)      // Input operands
               : "%rax", "%rdi", "%rsi", "%rdx" // Clobbered registers
  );

  return 0;
}

void my_start()
{
  volatile unsigned long status = main();
  // exit(status)
  asm volatile("mov $0x3c, %%rax\n" // exit syscall number (0x3c)
               "mov %0, %%rdi\n"    // exit status
               "syscall"            // Make the syscall
               :                    // No output operands
               : "r"(status)        // Input operands
               : "%rax", "%rdi"     // Clobbered registers
  );
};
```

<br>

**Command to build**:
```bash
gcc -Wl,-entry=my_start -nostdlib -o hello hello.c
```

<br>

# In Rust
In a **typical** Rust **binary** that links the **standard library**, **execution starts** in a **CRT**. This creates a stack and places the arguments in the right registers.
Then **crt1** invokes the [**entry point**](https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73) of the **Rust runtime**.<br>
