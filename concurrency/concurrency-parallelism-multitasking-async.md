# Table of contents
- [Table of contents](#table-of-contents)
- [Multiprocessor](#multiprocessor)
- [Multitasking](#multitasking)
  - [Task level parallelism](#task-level-parallelism)
  - [Concurrency](#concurrency)
    - [Use cases for concurrency](#use-cases-for-concurrency)
- [Multitasking in OS](#multitasking-in-os)
  - [Process vs. thread](#process-vs-thread)
    - [Thread's properties](#threads-properties)
    - [POSIX threads (Pthreads)](#posix-threads-pthreads)
- [Scheduling](#scheduling)
  - [Cooperative multitasking](#cooperative-multitasking)
  - [Preemtive multitasking](#preemtive-multitasking)
  - [Preemptive scheduling](#preemptive-scheduling)
    - [Advantages](#advantages)
    - [Disadvantages](#disadvantages)
  - [Cooperative scheduling](#cooperative-scheduling)
    - [Advantages](#advantages-1)
    - [Disadvantages](#disadvantages-1)
- [Threading models](#threading-models)
- [Async](#async)

<br>

# Multiprocessor
**Multiprocessor** can refer to machine with
- **1** multi**core** CPU;
- **N>1** single**core** CPUs;
- **N>1** multi**core** CPUs;

<br>

# Multitasking
**Multitasking** is a making of **progress** of **multiple tasks**. There are **2 ways** to achieve this:
- run tasks **concurrently**, but **not** at the same time;
- run tasks **simultaneously** at the same time (in **parallel**) on **independent** CPUs or cores;

<br>

## Task level parallelism
**Task level parallelism** (aka **parallelism**) is a form of *parallel computing* based on the concept of **task**.<br>
A **task** is a **set of operations** that *requires* some *resources* *to progress*. In other words, a **task** is an abstraction of a **unit of work**.<br>
In **task level parallelism** different tasks are executed **simultaneously** on **different** CPUs.<br>
**Task level parallelism** is possible on machines with **MIMD** only and it is **not** possible on other achitectures.<br>

<br>

## Concurrency
In contrast to *task level parallelism*, **multitasking** is possible even on **SIMD**, but **multitasking** on **SIMD** is **pseudo-parallelism**. So,
- in **MIMD** *multitasking* becomes a **task level parallelism**;
- in **SIMD** *multitasking* becomes a **pseudo-parallelism**;

<br>

**Parallelism** (aka **parallel computing**) means **simultaneity** at the **physical level**.<br>
**Concurrency** (aka **concurrency computing**) means **simultaneity** at the **logical level**. If two tasks are running **concurrently**, but they are **not** running *in parallel*, they **must** be able to **stop** and **resume** their **progress**. It means *tasks* must be **interruptible**.<br>

<br>

### Use cases for concurrency
- instead of waiting response when performing I/O switch to another tasks;
- prevent tasks from waiting too long by interrupting and resuming them;

<br>

# Multitasking in OS
In modern OS there are 2 approaches to achive multitasking: 
1.	**Multiprocessing** (aka process level multitasking) – multitasking at OS **processes** level, i.e., os schedules its **processes**.
2.	**Multithreading** (aka thread level multitasking) – multitasking at OS **threads** level, i.e., os schedules its **threads**.

<br>

## Process vs. thread
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

# Scheduling
## Cooperative multitasking
In **cooperative multitasking** the **programmer** is **reponsible** to **yield control** to the OS.<br>
This approach is error-prone, because a small mistake in a program's code could halt or crash the entire system.<br>

<br>

## Preemtive multitasking
In **preemtive multitasking** the **OS** is responsible for scheduling tasks and does this by switching contexts on the CPU.<br>

<br>

**Scheduling** is the action of assigning resources (workers) to perform tasks.<br>
**Scheduler** is a component of execution environment that dispatches tasks to workers, e.g., OS is execution environment and CPU/cores are workers. <br>

<br>

There 2 different types of scheduling:
1.	**Preemptive scheduling**.
2.	**Cooperative scheduling**.

<br>

## Preemptive scheduling
**Preemptive scheduling** involves the use of an **interrupt mechanism** which suspends the currently executing process or thread and invokes a scheduler to determine which process should execute next.<br>
Therefore, all processes/threads will get some amount of CPU time (**quantum**) at any given time.<br>
The **time slice** or **quantum** is the period of time for which a process or thread is allowed to run in a preemptive multitasking system.<br>

All modern OS use **preemptive scheduling**.

<br>

### Advantages
It has fine grained control over tasks in contrast to cooperative scheduler and allows evenly distribute them among workers.

<br>

### Disadvantages
It has **context switch** overhead.

<br>

## Cooperative scheduling
**Cooperative scheduling** is a multitasking in which the OS never initiates a context switch, instead, processes or threads voluntarily **yield control** back to **scheduler**.

<br>

### Advantages
Scheduling is **lightweight** because it doesn't require *context switching* of process/thread, it just **switches coroutines**.

<br>

### Disadvantages
There is risk that **poorly** designed program can consume all of the CPU time for itself.

<br>

# Threading models
There are 2 types of threads from programming point of view:
- **kernel-level threads** are scheduled by OS **kernel scheduler**.
- **user-level threads** (aka **coroutines**/**green threads**) are scheduled by **application scheduler** (aka **event loop**).

<br>

So there can be 3 threading models:
- **Kernel-level threading**: all threads inside process are **kernel threads**.
- **User-level threading**: all threads inside process are **green threads**.
- **Hybrid threading**: some of threads inside process are **kernel threads** and some of threads inside process  **green threads**.

<br>

# Async
*Multitasking*, *concurrency*, *parallelism* and *async programming* are **closely related** to each other.<br>