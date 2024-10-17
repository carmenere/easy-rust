# Table of contents
- [Table of contents](#table-of-contents)
- [Multiprocessor](#multiprocessor)
- [Multitasking](#multitasking)
  - [Multitasking in OS](#multitasking-in-os)
- [Scheduling](#scheduling)
  - [Preemptive scheduling](#preemptive-scheduling)
  - [Cooperative scheduling](#cooperative-scheduling)
- [Threading models](#threading-models)
  - [Kernel-level threading](#kernel-level-threading)
    - [Process vs. thread](#process-vs-thread)
    - [Thread's properties](#threads-properties)
    - [POSIX threads (Pthreads)](#posix-threads-pthreads)
  - [User-level threading](#user-level-threading)

<br>

# Multiprocessor
**Multiprocessor** can refer to machine with
- **1** multi**core** CPU;
- **N>1** single**core** CPUs;
- **N>1** multi**core** CPUs;

<br>

# Multitasking
There are 2 fundamentally different types of computing:
- **parallelism** (aka **parallel computing**) means **simultaneity** at the **physical level**;
- **concurrency** (aka **concurrency computing**) means **simultaneity** at the **logical level**;

<br>

**Multitasking** is a form of **concurrency** based on the concept of **task**.<br>

A **task** is a **set of operations** that *requires* some *resources* *to progress*. There are **2 ways** to make progress for tasks:
- **concurrency**: different **tasks** are executed **concurrently**, but **not** at the same time;
- **task level parallelism**: different **tasks** are executed **simultaneously** at the same time on **different** CPUs;

Tasks **must** be able to **stop** and **resume** their **progress**. It means *tasks* must be **interruptible**.<br>

Use cases for *concurrency*:
- to **prevent** CPU **idle** time by **I/O-bound tasks**: switch to another tasks to make some progress instead of waiting response when performing I/O;
- to **prevent consume** all of the CPU time by **CPU-bound tasks**: scheduler periodically **interrupts** and **resumes** tasks;

<br>

## Multitasking in OS
In modern OS there are 2 approaches to achive multitasking: 
1.	**Multiprocessing** (aka process level multitasking) – multitasking at OS **processes** level, i.e., os schedules its **processes**.
2.	**Multithreading** (aka thread level multitasking) – multitasking at OS **threads** level, i.e., os schedules its **threads**.

<br>

# Scheduling
**Scheduler** is a component of execution environment that **dispatches tasks to workers**, e.g., OS is execution environment and CPU/cores are workers. <br>

**Scheduling** is the action of **dispatching tasks to workers**.<br>

**Dispatch** is the act of sending something somewhere, e.g.,
- dispatch an event to a listener;
- dispatch an interrupt to a handler;
- dispatch a process to the CPU;

<br>

There 2 different types of scheduling:
1.	**Preemptive scheduling**.
2.	**Cooperative scheduling**.

<br>

## Preemptive scheduling
All modern OS use **preemptive scheduling**.<br>

In **preemtive scheduling** the **OS** is responsible for scheduling tasks (threads) and does this by switching contexts on the CPU.<br>

**Preemptive scheduling** involves the use of an **interrupt mechanism** which suspends the currently executing process or thread and invokes a scheduler to determine which process should execute next. Therefore, all processes/threads will get some amount of CPU time (**quantum**) at any given time.<br>

The **time slice** or **quantum** is the period of time for which a process or thread is allowed to run.<br>

<br>

**Advantages**:
- it has fine grained control over tasks and allows evenly **distribute resources** among tasks;

**Disadvantages**:
- it has **context switch** overhead: it is take some time to **store** and **restore** the register state;

<br>

## Cooperative scheduling
In **cooperative scheduling** the **developper** is **reponsible** to **yield control** back to **scheduler**.<br>

<br>

**Advantages**:
- scheduling is **lightweight** because it **doesn't** require *context switching* of process/thread, it just **switches coroutines**;

**Disadvantages**:
- there is risk that **poorly** designed program can **consume** all of the CPU time for itself and **halt** the entire system;

<br>

# Threading models
There are 2 types of threads:
- **kernel-level threads** (aka **OS threads**) are scheduled by OS **kernel scheduler**;
- **user-level threads** are scheduled by **application scheduler** (aka **event loop**), OS doesn't know anything about them;

<br>

There 3 threading models:
- **kernel-level threading**: an application spawns **OS threads** for each concurrent operation;
- **user-level threading**: an application spawns **user-level threads** for each concurrent operation;
- **hybrid threading**: an application spawns some **OS threads** and then inside OS threads it spawns **user-level threads**;

<br>

## Kernel-level threading
### Process vs. thread
A **process** is an executable code located in an **isolated address space**. In other words, a process is a running instance of a program, while in the general case, one running program can spawn N processes.<br>
A **thread** is an independent flow of instructions that runs inside the **address space of a process**.<br>

<br>

### Thread's properties
- every process can be single threaded or multi-threaded;
- every process spawns **primary thread**;
- all threads of some process are executed in **the same** address space;
- every thread is scheduled by OS kernel like processes.

<br>

### POSIX threads (Pthreads)
**POSIX Threads** (aka **pthreads**) defines types, functions and constants to control threads.

<br>

## User-level threading
There are 3 kinds of **user-level threads**:
- **stackful coroutines**:
  - **fibers**;
  - **green threads**;
- **callback** based approach - each task consists of a **chain of callbacks**;
- **stackless coroutines**:
  - **promises** (in js);
  - **futures** and **async/await** (in Rust): **async** transform function to *future* and **await** on *future* **yields control** to the runtime and task is suspended until the *future* has **finished**;

<br>

**Differences**:
- *Stackful coroutines* **can** *suspend* execution at any point. They use the same mechanisms as an OS, setting up a stack for each task (thread), saving and restoring CPU's registers at every context switch.<br>
- *Stackless coroutines* **cannot** *suspend* execution at any point. *Stackless coroutines* have to **yield** *at specific points* and there is **no way** to **suspend execution** in the **middle** of a *stack frame*.<br>
