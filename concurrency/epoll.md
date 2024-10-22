
# Table of contents
- [Table of contents](#table-of-contents)
- [mio](#mio)
- [`epoll`](#epoll)
  - [`epoll` API](#epoll-api)
    - [Syscall `epoll_create`](#syscall-epoll_create)
    - [Syscall `epoll_ctl`](#syscall-epoll_ctl)
    - [Syscall `epoll_wait`](#syscall-epoll_wait)
    - [Struct `epoll_event`](#struct-epoll_event)
  - [Level-triggered and edge-triggered](#level-triggered-and-edge-triggered)
    - [Level triggered example](#level-triggered-example)
    - [Edge triggered example](#edge-triggered-example)
  - [Thundering herd problem](#thundering-herd-problem)

<br>

# mio
The `mio` is a **low-level cross-platform I/O library** that creates abstraction over `epoll`, `kqueue` and `IOCP`, also mio support **iOS** and **Android**.<br>
The `mio` **doesn't** support `EPOLLONESHOT` and uses `epoll` in an **edge-triggered** mode.<br>

<br>

# `epoll`
The `epoll` is **not** part of POSIX and it's specific to **Linux**.<br>

<br>

## `epoll` API
### Syscall `epoll_create`
The `epoll_create` creates a new **epoll instance** (event queue inside kernel) and returns a **file descriptor** referring to that instance.<br>
On **success**, the `epoll_create` returns a file descriptor of new **epoll instance**.<br>

<br>

The **epoll instance** has 2 lists:
- the **interest list** (aka **epoll set**): the *set of file descriptors* (aka **sources of events**) that the process has registered to monitor;
- the **ready list**: the *set of file descriptors* that are **ready** for I/O (**read** or **write**);

<br>

**Declaration**:
```c
int epoll_create(int size);
```

<br>

**Arguments**:
- `size` this argument **must** have **any** value **larger** than `0`, but it will be ignored, this argument is there only for historical reasons;

<br>

### Syscall `epoll_ctl`
The `epoll_ctl` is used to **add**, **modify** or **delete** entries in the **interest list** of the **epoll instance**.<br>
On **success**, the `epoll_ctl` returns `0`.<br>

It supports 3 main operations: **add**, **modify**, **delete**.<br>

<br>

**Declaration**:
```c
int epoll_ctl(int epfd, int op, int fd, struct epoll_event *_Nullable event);
```

<br>

**Arguments**:
- `epfd`: the **file descriptor** that refers to **epoll instance**;
- `op`: specifies type of operation **add**, **modify** or **delete**, valid values for the op argument are:
  - `EPOLL_CTL_ADD`: **adds** new entry (`fd` argument) to the **interest list** of the **epoll instance**;
  - `EPOLL_CTL_MOD`: **changes** the settings of particular **file descriptor** (`fd` argument);
  - `EPOLL_CTL_DEL`: **deletes** (deregisters) **file descriptor** (`fd` argument) from the **interest list**;
- `fd`: some opened **file descriptor**;
- `epoll_event`: describes the object linked to the `fd`, it is required by `EPOLL_CTL_ADD` and `EPOLL_CTL_MOD`

<br>

### Syscall `epoll_wait`
The `epoll_create` **waits for I/O events**.<br>

<br>

A call to `epoll_wait` will **block until** either:
- a file descriptor delivers an event;
- the call is **interrupted** by a **signal handler**;
- the **requested timeout expires**;

On **success**, `epoll_wait` returns the **number** of **fd** ready for the requested I/O operation, or **0** if no file descriptor became ready during the **requested timeout** milliseconds.

<br>

**Declaration**:
```c
int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
```
<br>

### Struct `epoll_event`
Instance of `epoll_event` is passed to `epoll_ctl`.<br>

<br>

**Declaration**:
```c
struct epoll_event {
   uint32_t      events;  /* Epoll events */
   epoll_data_t  data;    /* User data variable */
};
```

<br>

**Fields**:
- the `events` **field** is a **bit mask** composed by ORing together zero or more **event types** and **input flags**.<br>
- the `data` **field** specifies data that the kernel should save and then return when this **fd** becomes ready.<br>


<br>

The **event types** indicates **what kind of events** we want to be notified of. When `epoll_wait` returns, the OS writes to `events` **field** kind of events avaliable on the appropriate **fd**.<br>
The **input flags** define **how** and **when** notify application. When `epoll_wait` returns, the OS **doesn't** return them back.<br>

<br>

The available **event types** are:
- `EPOLLIN`
  - in the context of `epoll_ctl`: application is interested in **read operations** on the `fd` argument;
  - in the context of `epoll_wait`: the associated **fd** is available for **read** operations;
- `EPOLLOUT`
  - in the context of `epoll_ctl`: application is interested in **write operations** on the `fd` argument;
  - in the context of `epoll_wait`: the associated **fd** is available for **write** operations;

<br>

The available **input flags** are:
- `EPOLLET`
  - in the context of `epoll_ctl`: enables **edge-triggered** mode for the `fd` argument;
- `EPOLLONESHOT`
  - in the context of `epoll_ctl`: requests **one-shot notification** for the associated **fd**;

<br>

## Level-triggered and edge-triggered
The `epoll` can notify events 2 modes:
- **level-triggered** mode (**by default**) it means that kernel will **notify** application **as long as data** in the **buffer** associated with the ****fd****;
- **edge-triggered** mode, it means that kernel will **notify** application **only** when the **buffer** has **changed** *from* **empty** *to* **non empty**, as long as there is data in the buffer **no new events will be reported**, in other words, you will **not** get **new** notification **untill** the buffer is **fully drained** and **then filled with new data**;

To enable **edge-triggered** mode set `EPOLLET` **flag** in the `events` field of `epoll_event`.<br>

When using epoll in **level-triggered** mode, we can get **multiple notification** on the **same event** since we haven't had time to drain buffer yet.<br>
To remedy this, there is a flag `EPOLLONESHOT` which requests **one-shot notification** for the associated **fd**. This means that after an event notified for the **fd** by `epoll_wait`, the **fd** is **disabled** in the **interest list** and **no** other events will be reported by the epoll interface. The application must call `epoll_ctl` with `EPOLL_CTL_MOD` to **reactivate** the **fd** with a new event mask.<br>

<br>

### Level triggered example
- `fd` was added to `epoll` with `EPOLLIN` flag;
- `epoll_wait()` is blocked until new data will be written to `fd` buffer;
- write to file 19 bytes;
- `epoll_wait()` is unblocked with `EPOLLIN`;
- do nothing with data;
- `epoll_wait()` is unblocked with `EPOLLIN` again;

<br>

### Edge triggered example
- `fd` was added to `epoll` with `EPOLLIN` flag;
- `epoll_wait()` is blocked until new data will be writeen to `fd` buffer;
- write to file 19 bytes;
- `epoll_wait()` is unblocked with `EPOLLIN`;
- do nothing with data;
- `epoll_wait()` is blocked until new data will be written to `fd` buffer;
- 
<br>

## Thundering herd problem
The `epoll` allows scenario when many threads call `epoll` on the same **epoll instance**. The `epoll` in **level-triggered** mode **wakes all** threads that are blocked on `epoll_wait`, causing all of them to try handle the event. This is called **thundering herd problem**. The `epoll` has flag `EPOLLEXCLUSIVE` that solve this problem.<br>
The **thundering herd problem** occurs when a large number of processes or threads waiting for an event are awoken when that event occurs, but only **one** process is able to handle the event.<br>

<br>

