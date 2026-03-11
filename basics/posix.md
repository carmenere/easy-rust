# POSIX.1-2008
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
