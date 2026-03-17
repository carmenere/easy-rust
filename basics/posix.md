# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [POSIX](#posix)
  - [Syscalls](#syscalls)
    - [`sigaction()`](#sigaction)
  - [Errors](#errors)
    - [EINTR](#eintr)
- [POSIX signal disposition](#posix-signal-disposition)
<!-- TOC -->

<br>

# POSIX
## Syscalls
### `sigaction()`
[**sigaction**](https://pubs.opengroup.org/onlinepubs/9699919799/functions/sigaction.html).<br>

The `sigaction()` function allows the calling process to specify the action to be associated with a specific signal. The **flags** can be used to modify the behavior of the specified signal.<br>

Flags:
- `SA_RESTART`
    - if **set**, and an *interruptible function* is **interrupted** by this signal, the function **shall restart** and **shall not fail** with `EINTR` unless otherwise specified;
    - if the flag is **not** *set*, and an *interruptible function* is **interrupted** by this signal, the function **shall fail** with **errno** set to `EINTR`;

<br>

## Errors
### EINTR
**Interruptible functions** may return `EINTR` error.<br>
In POSIX, an **interruptible function** is a *system call* or *library function* that, when **interrupted by a signal** (**without** `SA_RESTART` flag), **returns prematurely** with `-1` and **sets** the global variable **errno** with an *error code* of `EINTR` instead of completing its task. This behavior allows programs to handle asynchronous events (signals) while a function is blocked waiting for an event (e.g., I/O or a timer).<br>

<br>

# POSIX signal disposition
[**man page**](https://man7.org/linux/man-pages/man7/signal.7.html).<br>

The *disposition of a signal* defines how a process **behaves** when it **receives** a specific **signal**.<br>
Each signal has **default disposition** (aka **default action**).<br>

Thare 5 **default actions**:
- `Term` **terminates the process**;
- `Ign` **ignores the signal**;
- `Core` **terminates the process** and **dumps core**;
- `Stop` **stops the process**;
- `Cont` **continues the process** *if* it is currently **stopped**;

Processes can use system calls `sigaction()` to change the *default disposition* of most signals to either **ignore** them, or **catch** them with a **custom signal handler** function.<br>

**Note**, the **signals** `SIGKILL` and `SIGSTOP` **cannot** be *caught*, *blocked*, or *ignored*.<br>

<br>

**POSIX signal dispositions**:
|Signal|Action|Comment|Standard|
|:-----|:-------|:-----|:------|
|`SIGABRT`|`Core`|Abort signal from `abort(3)`|POSIX.1-1990|
|`SIGALRM`|`Term`|Timer signal from `alarm(2)`|POSIX.1-1990|
|`SIGBUS `|`Core`|Bus error (bad memory access)|POSIX.1-2001|
|`SIGCHLD`|`Ign`|Child stopped, terminated, or continued|POSIX.1-2001|
|`SIGCLD`|`Ign`|A synonym for ]| - |
|`SIGCONT`|`Cont`|Continue if stopped|POSIX.1-1990|
|`SIGEMT`|`Term`|Emulator trap| - |
|`SIGFPE `|`Core`|Erroneous arithmetic operation|POSIX.1-1990|
|`SIGHUP `|`Term`|Hangup detected on controlling terminal or death of controlling process|POSIX.1-1990|
|`SIGILL `|`Core`|Illegal Instruction|POSIX.1-1990|
|`SIGINFO`|`Term`|A synonym for ]| - |
|`SIGINT `|`Term`|Interrupt from keyboard|POSIX.1-1990|
|`SIGIO `|`Term`|I/O now possible| - |
|`SIGIOT`|`Core`|IOT trap.  A synonym for ]| - |
|`SIGKILL`|`Term`|Kill signal|POSIX.1-1990|
|`SIGLOST`|`Term`|File lock lost (unused)| - |
|`SIGPIPE`|`Term`|Broken pipe: write to pipe with no readers|POSIX.1-1990|
|`SIGPOLL`|`Term`|Pollable event|POSIX.1-2001|
|`SIGPROF`|`Term`|Profiling timer expired|POSIX.1-2001|
|`SIGPWR`|`Term`|Power failure| - |
|`SIGQUIT`|`Core`|Quit from keyboard|POSIX.1-1990|
|`SIGSEGV`|`Core`|Invalid memory reference|POSIX.1-1990|
|`SIGSTKFLT`|`Term`|Stack fault on coprocessor (unused)| - |
|`SIGSTOP`|`Stop`|Stop process|POSIX.1-1990|
|`SIGTSTP`|`Stop`|Stop typed at terminal|POSIX.1-1990|
|`SIGSYS `|`Core`|Bad system call|POSIX.1-2001|
|`SIGTERM`|`Term`|Termination signal|POSIX.1-1990|
|`SIGTRAP`|`Core`|Trace/breakpoint trap|POSIX.1-2001|
|`SIGTTIN`|`Stop`|Terminal input for background process|POSIX.1-1990|
|`SIGTTOU`|`Stop`|Terminal output for background process|POSIX.1-1990|
|`SIGURG `|`Ign`|Urgent condition on socket|POSIX.1-2001|
|`SIGUSR1`|`Term`|User-defined signal 1|POSIX.1-1990|
|`SIGUSR2`|`Term`|User-defined signal 2|POSIX.1-1990|
|`SIGVTALRM`|`Term`|Virtual alarm clock|POSIX.1-2001|
|`SIGXCPU`|`Core`|CPU time limit exceeded;|POSIX.1-2001|
|`SIGXFSZ`|`Core`|File size limit exceeded;|POSIX.1-2001|
|`SIGWINCH`|`Ign`|Window resize signal|POSIX.1-2024|
