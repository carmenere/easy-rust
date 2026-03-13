# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Stack trace vs. call stack](#stack-trace-vs-call-stack)
- [Stack frame](#stack-frame)
- [Stack unwinding](#stack-unwinding)
- [The concept of stack unwinding in Rust](#the-concept-of-stack-unwinding-in-rust)
  - [Unwind vs. Abort](#unwind-vs-abort)
<!-- TOC -->

<br>

# Stack trace vs. call stack
The fundamental difference is that the **call stack** is an active data structure used by a running program to manage function calls, while a **stack trace** is a **snapshot** of the *call stack* at a specific point in time, usually for debugging purposes.<br>

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
- **On callee side** or **inside called function**:
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

<br>

# Stack unwinding
Stack unwinding happens:
- **normally**, when a function returns;
- **during exception handling**, when an exception is thrown and control transfers to a matching catch block;

When an exception is not handled in the current function, the runtime:
- exits the function;
- destroys all local objects;
- moves up the call stack searching for a matching catch block;
- terminates the program if no handler is found;

All local (automatic) objects are destroyed during stack unwinding.
Destructors are called in reverse order of construction.
This guarantees cleanup of stack-based resources.

**Stack unwinding** is a fundamental mechanism in C++ exception handling, responsible for cleaning up resources and maintaining program stability in the presence of exceptions.
**Stack unwinding** refers to the process of deallocating resources and cleaning up the call stack when an exception is thrown but not caught within a function.

When an exception is thrown, the C++ runtime searches for a matching catch block to handle the exception. If no matching catch block is found within the function, the function’s stack frame is unwound, and control is transferred to the caller function. This process continues recursively until an appropriate catch block is found or until the top-level function is reached.

If during stack unwinding a destructor throws an exception and that exception is not handled, the terminate() function is called.

When an exception is thrown using the throw keyword, the program begins a search for a matching catch block to handle the exception.

`std::terminate()`: Function called when an exception is not caught.

In the C++ **exception mechanism**, control moves **from the throw** statement to the **first catch statement** that can handle the thrown type.
When the catch statement is reached, all of the variables that are in scope between the **throw** and **catch statements** are destroyed in a process that is known as **stack unwinding**.


<br>

# The concept of stack unwinding in Rust
there are two different error handling models in Rust these days.
The second way to indicate failure is a panic.
The way this works is by unwinding the stack slice by slice, invoking cleanup code at each level and finally terminate the task.

When you raise an exception you need to immediately bubble up stack frame by stack frame until you hit your exception handler.
as you blaze through the stack frames, Rust needs to execute all necessary cleanup code on each level so that no memory or resources leak.


Traditionally I think there are two problems with stack unwinding. The first one is that unlike function calling conventions, stack unwinding is not particularly standardized. This is especially a problem if you try to combine functions from different programing languages together.

Stack unwinding refers to the process of removing function call frames from the function call stack during runtime.

<br>

## Unwind vs. Abort
By default, Rust uses unwinding when a panic occurs. However, you can configure your program to use an alternative strategy: aborting.

**Abort** - Immediately terminates the process without cleaning up memory or resources.
Panics are not intended to be caught or handled.

By default, panicking **terminates** the current thread **by unwinding the stack**, executing all destructors as it goes. This means that the program can be left in a consistent state and the rest of the program can carry on executing.

When a panic occurs, a panic hook function is called. By default, this prints a message and possibly a backtrace to stderr, but it can be customised.
In a no-std crate, you'll need to set your own panic handler. Use the #[panic_handler] attribute and see the docs for core::panicking for more info.

panic strategies:
unwind vs abort.

When Rust panics, it has two main options for how to handle that failure:

**Unwind**:
Try to gracefully clean up — like C++ exceptions.
Rust walks back up the stack, running destructors (Drop traits) for all live objects until it finds a boundary that can handle the panic.
**Abort**:
Drop everything and crash the process immediately — no cleanup, no mercy.
This is faster but completely destructive.

You control this with a simple line in your Cargo.toml:

```toml
[profile.release]
panic = 'abort'
```
