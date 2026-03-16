# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Stack trace vs. call stack](#stack-trace-vs-call-stack)
- [Stack frame](#stack-frame)
<!-- TOC -->

<br>

# Stack trace vs. call stack
The fundamental difference is that the **call stack** is an active data structure used by a running program to manage function calls, while a **stack trace** (aka **backtrace**) is a **snapshot** of the *call stack* at a specific point in time, usually for debugging purposes.<br>

<br>

# Stack frame
The **stack** is primarily used for a few things:
- storing **function arguments**;
- storing **local variables**;
- storing processor state between function calls;

<br>

A **stack frame** is an **execution context** of a function: it stores its **arguments**, **local variables** and **return address**.<br>
A **return address** is an **address** of the **next instruction**, it is stored in the **IP** register.<br>

Each time a function is called, a **new** *stack frame* is created, and when the function finishes, the reverse process occurs, restoring the **stack frame** of caller.<br>

There are 3 registers **BP** or **FP** (**Base Pointer** or **Frame Pointer**), **SP** (**Stack Pointer**) and **IP** (**Instruction Pointer**) that are crucial for managing the **stack frame**:
- the **SP** register always points to the **current** *top of the stack* (the lowest memory address currently used on the stack). Its value **changes dynamically** as data is *pushed onto* or *poped off* the stack;
- the **BP**/**FP** register serves as a **fixed base address** within the **current stack frame** and allows consistent access to function *parameters* and *local variables* using offsets from this *base address* during function is exeduted:
  - *local variables* are accessed using **negative offsets** from **BP** (e.g., `[bp-4]`, `[bp-8]`) because the stack grows downwards;
  - *function arguments* are accessed using **positive offsets** from **BP** (e.g., `[bp+8]`, `[bp+12]`) because they reside in the memory area **above** the saved **BP** value;
- the **IP** register always points to the **next** *instruction to be executed*;
  - the **IP** is **always** the **last thing to be saved** on the stack frame of a given function, before it transfer control to another function;

<br>

When a function is called a specific actions are performed on the **caller side** and **inside callee** according to a specific **calling convention**, like `cdecl`.<br>

The **calling convention** specifies:
- how **arguments** are passing to function;
- how function **returns** its result;
- how **registers** are used;
- how **stack** is set up;
- **prologue** and **epilogue** logic for functions;

<br>

The **function prologue** is a set of instructions at the **beginning** of a **called** function that sets up the new **stack frame**.<br>
The **function epilogue** is a set of instructions at the **end** of a **called** function that restores the stack and registers to their state before the function was called.<br>

<br>

When the number of parameters passed to a function **exceeds** the capacity of registers designated for parameter passing in the calling convention, additional parameters are passed on the stack.<br>

<br>

According `cdecl`:
- **On caller side** (caller responsibility):
  - **before** the **callee** (function `<name>`) is called the **caller**:
    - puts the first **6** parameters in the registers designated for parameter passing in the calling convention;
    - then, puts any additional parameters onto the stack;
    - then, the **callee** is called: the **caller** runs `call <name>` instruction
    - the `call <name>` instruction:
      - pushes the **return address** onto the stack;
      - then **transfers control** to function `<name>`;
  - **after** the **callee** ends and returns to the **caller**:
    - the **caller** removes from stack all parameters passed to callee
    - save the **return value** to a local variable
- **On callee side** (**inside called function**):
  - **function prologue**:
    - `push bp`: saves the caller's *base pointer* (**BP**) onto the stack;
    - `mov bp, sp`: sets the **current** *stack pointer* (**SP**) as the **new** *base pointer* (**BP**) for the **current frame** (copies **SP** to **BP**);
    - `sub sp, N`: **allocates** space for **local variables** by decrementing the stack pointer by a certain number of `N` bytes;
  - **function epilogue**:
    - `mov sp, bp`: **deallocates** space for **local variables** by moving **BP** to **SP**;
    - `pop bp`: **restores** the caller's original *base pointer* value from the stack to the **BP** register;
    - `ret`:
      - **pops** the **return address** from the stack into the **IP** (instruction pointer);
      - then **transfers control** back to the caller;

<br>

**Illustration**:<br>
![stack_frame](/img/stack_frame.png)
