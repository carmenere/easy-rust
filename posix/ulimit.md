# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [POSIX resource limits](#posix-resource-limits)
  - [Drawbacks](#drawbacks)
- [ulimit](#ulimit)
<!-- TOC -->

<br>

# POSIX resource limits
**POSIX resource limits** (aka **POSIX rlimits**) limit the consumption of **resources**. A **resource limit** is represented by an **rlimit** structure.<br>
Each **resource** has an associated **soft limit** and **hard limit**.<br>
**Any** process (*privileged* or *unprivileged*) may **set** its **soft limit** in the **range** from **0** up to the **hard limit**: `[0, hard limit]`.<br>
But **increasing** the **hard limit** is a **privileged action**. Only a **privileged process** (under Linux: one with the **CAP_SYS_RESOURCE** capability) may **set** a *hard limit* **larger** than the **current value** of the *hard limit*, more over a **privileged process** may **set** its **hard limit** to **arbitrary** value.<br>

<br>

There are 2 syscalls:
- `setrlimit()` to **set** *resource limit*;
- `getrlimit()` to **read** *resource limit*;

<br>

The **IEEE Std 1003.1** (aka **POSIX.1**) defines **7** *resource limits*, [see the documentation](https://pubs.opengroup.org/onlinepubs/009696699/functions/getrlimit.html):
- RLIMIT_CORE;
- RLIMIT_CPU;
- RLIMIT_DATA;
- RLIMIT_FSIZE;
- **RLIMIT_NOFILE**;
- RLIMIT_STACK;
- RLIMIT_AS;

<br>

**Linux** defines **additional** *resource limits*, [see the documentation](https://www.man7.org/linux/man-pages/man2/getrlimit.2.html):
- RLIMIT_MEMLOCK;
- RLIMIT_MSGQUEUE;
- RLIMIT_NICE;
- RLIMIT_NPROC;
- RLIMIT_SIGPENDING;
- RLIMIT_RTPRIO;
- RLIMIT_RTTIME;

<br>

## Drawbacks
The `setrlimit()` does **not** allow you to set a limit for a **group of processes**, for eample, a **memory leak** for an entire group of processes can **easily exceed** the limit set for a **single** process. This **drawback** of the rlimit mechanism was one of the reasons for the creation of **cgroups**.<br>

<br>

# ulimit
In many **interactive shells**, **resource limits** can be inspected or modified with the **ulimit** shell function.<br>
The `ulimit -a` list **all** avaliable **resource limits**:
Example:
```bash
ulimit -a
-t: cpu time (seconds)              unlimited
-f: file size (blocks)              unlimited
-d: data seg size (kbytes)          unlimited
-s: stack size (kbytes)             8176
-c: core file size (blocks)         0
-v: address space (kbytes)          unlimited
-l: locked-in-memory size (kbytes)  unlimited
-u: processes                       2666
-n: file descriptors                256
```