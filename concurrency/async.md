# Table of contents
- [Table of contents](#table-of-contents)
- [Async](#async)
- [I/O models in different OS](#io-models-in-different-os)
  - [Blocking I/O](#blocking-io)
  - [Non-blocking I/O](#non-blocking-io)
  - [Event queues (aka I/O multiplexing)](#event-queues-aka-io-multiplexing)
    - [Typycal using event queue kernel API](#typycal-using-event-queue-kernel-api)
- [Example: enable non-blocking mode in linux](#example-enable-non-blocking-mode-in-linux)

<br>

# Async
*Multitasking*, *concurrency*, *parallelism* and *async programming* are **closely related** to each other.<br>

What is **computation**? It is a proccess of **transformig** some **input** into some **result** where *result* can be **success** or **failure**.<br>
What is **async** *computation*? It is a **promise** to provide the **result** of computation in the **future**.<br>

The common approach of async programming is following: **async operation** instead of result returns an **object** that **will store result** of async operation and which can be **accessed later**. For example, **non-blocking socket** is an example of such object: it holds data and application can read them at any time.<br>

<br>

# I/O models in different OS
Modern OS provide several ways of performing **I/O**, both **blocking** and **non-blocking**. I/O operations need to go through the OS since they are dependent on resources that our OS abstracts over.

<br>

## Blocking I/O
When a thread asks the OS to perform a **blocking** operation, OS will **suspend** this thread. When operation is completed OS will wake up our thread again.<br>
By default, **all file descriptors** on Unix systems start out in **blocking mode**. That means that system calls like `read`, `write`, or `connect` will be **blocked**.

<br>

## Non-blocking I/O
When a thread asks the OS to perform a **non-blocking** operation, OS will **not** suspend this thread. But instead OS provides mechanism that thread can use to ask OS if the operation **ready** or **not**. The process of querying for status is called **polling**.
When operation is completed OS will wake up our thread again.<br>
**Don't block** if request **cannot** be completed **immediately**, returns error `EWOULDBLOCK` instead.

<br>

## Event queues (aka I/O multiplexing)
Instead of polling every handle individually we can add all handles to single **event queue**. Implementations of **event queues** in OS can serve thousands handles with very little overhead.<br>

Event queue kernel APIs in different OS:
|OS|Event queue kernel API|Type|
|:-|:---------------------|:---|
|Linux|`epoll`|**readiness-based**|
|macOS|`kqueue`|**readiness-based**|
|Windows|`IOCP`|**completion-based**|
|Linux|`io_ring`|**completion-based**|

<br>

- `epoll` and `kqueue` are known as **readiness-based** *event queues*. This type of queue notifies you when an **event** is **ready** to be performed. For example, socket that is ready for reading.
- `iocp` is known as **completion-based** event queue, **iocp** means **input output completeion port**. This type of queue notifies you when events are **completed**.

<br>

There are 2 variants to interact with the event queue kernel API:
- **query the queue with regular intervals** to check if any of the events we added changed statuses;
- **make blocking call to the queue**, telling the OS wake us when at least one event in our queue has changed status;

<br>

### Typycal using event queue kernel API
1. We create **event queue** by calling the syscall `epoll_create()` or `kqueue()`.
2. We create new networking socket and get its `fd`.
3. Through another syscall, we **add** `fd` from *step 2* to the **event queue** we created in *step 1* and also we specify interest in `read` events on this `fd`.
4. Next, we call `epoll_wait()` or `kevent()` to wait for an event. In **blocking** mode or **non-blocking** manner.

<br>

# Example: enable non-blocking mode in linux
To put `fd` into **non-blocking mode** it is needed add `O_NONBLOCK` to the set of `fcntl` **flags**:
```c
/* set O_NONBLOCK on fd */
int flags = fcntl(fd, F_GETFL, 0);
fcntl(fd, F_SETFL, flags | O_NONBLOCK);
```