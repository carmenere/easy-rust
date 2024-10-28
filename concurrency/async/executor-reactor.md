# Table of contents
- [Table of contents](#table-of-contents)
- [Executor/Reactor pattern](#executorreactor-pattern)
- [`Future` life cycle](#future-life-cycle)
    - [Spawning](#spawning)
    - [Polling](#polling)
    - [Waiting](#waiting)
    - [Waking](#waking)
- [Waker API](#waker-api)
  - [Ways to implement `wake()`](#ways-to-implement-wake)
    - [Using task id](#using-task-id)
    - [Using reference counter](#using-reference-counter)

<br>

# Executor/Reactor pattern
At the **top** of the program is the **Executor**. The **Executor** is just a **scheduling algorithm** that executes the `Futures` by calling `poll()` on them.<br>
- **Executer** provides special API called **spawner**: spawner produces new tasks and puts them into *Executor's* **task queue**;
- **Executor** provides the runtime that iterates over its **task queue** and calls `poll()` on `Futures` until `Futures` return the `Ready` state;

<br>

At the **bottom** of the program is **Reactor** (aka **source of system IO events**).<br>
The **Reactor** notifies the **Executor** which task is ready to continue executing.<br>
**Reactor** is an **interface** between **Executor** and **OS**.<br>

**Reactor** provides **subscription API** for **external events**:
- IO events;
- IPC;
- timers;
- interrupts;

<br>

In async runtime, **subscribers** are `Futures` requesting **low level IO operations**, i.e., **read from socket**, **write to socket** and so on.<br>

<br>

So:
- *Executor* **schedules** `Futures` that are **ready to be polled**.
- *Reactor* **waits** **IO events** and **wakes** `Futures` that are bound to events **when events happen**.
- **Event loop** = **Executor** + **Reactor**.

<br>

# `Future` life cycle
Every `Future` transits through different phases during its life cycle.<br>

### Spawning
**Spawning** is registering a **top-level** `Future` at the **Executor**.<br>

### Polling
**Executor** fetches `Future` from its **task queue** and call `poll(cx)` method on it where `cx` is `Context`.<br>
`Context` is **wrapper** for `Waker` and just contains a reference to a `Waker`.<br>
The result of the `poll(cx)` method represents the the state of the `Future`.<br>

### Waiting
When the **Executor** calls `poll()` on a `Future`, that `Future` will return either `Ready` or `Pending`:
- If `Future` returns `Ready(T)` then the `.await` will return `T` and the **Executor** removes it from the **task queue**.
- If `Future` returns `Pending` then the **Executor** removes it from the **task queue**, but **Reactor** will notify **Executor** when particular `Future` will become ready to be polled again. This is where the **Waker API** comes in.

### Waking
The **reactor** stores a **copy** of the `Waker` that the **executor** passed to the future when it polled it.<br>
The **reactor** tracks events on I/O source.<br>
When the **reactor** gets a notification that an **event has happened** on one of the **tracked source**, it locates the `Waker` associated with that source and calls `Waker::wake` on it.<br>
This in turn puts `Future` that is bound to this event into *Executor's* **task queue**.<br>

<br>

# Waker API
The **Waker API** connects *Executor* and *Reactor*.<br>
Every time **Executor** calls `poll(cx)` method it passes a `Context` to it. `Context` provides access to a `Waker`, i.e., it wraps `Waker`.<br>
The reason `poll()` takes `Context` instead `Waker` is to has ability add other things to `Context` in future.<br>

Requirements to `Waker` type:
- the `Waker` type cannot be Generic because it is need to be passed through arbitrary `Futures`;
- the `Waker` type must implement `.wake()` method;
- the `Waker` type must implement `Clone` trait;

<br>

`Futures` can be **nested** and `Waker` object is passed along chain of nested `Futures` until it reaches the **source of event** (**Reactor**), then `Waker` is being registered in **Reactor**.<br>

If `Future` returns `Poll::Pending` then `Waker`, that was passed inside `Context`, is registered in **Reactor** and bound to **event id** (e.g. **file descriptor**).<br>
When event occurs **Reactor** calls `wake()`.

To poll `Futures` it is necessary to create a `Waker`. `Waker` is responsible for scheduling a task to be polled again once `wake()` is called.<br>
The easiest way to create a new `Waker` is by implementing the `ArcWake` trait and then using the `waker_ref()` or `into_waker()` functions to turn an `Arc<impl ArcWake>` into a `Waker`.<br>

<br>

## Ways to implement `wake()`
### Using task id
In this approach the `Waker` is **Task id** and the *Executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
Also Executor stores set of Tasks as `HashMap<Task_id, Task>`.<br>
When event occurs, **Reactor** calls `wake()` and it appends **Task** id to *Executor’s* **task queue**.<br>

### Using reference counter
In this approach the `Waker` is `Arc<Task>` and the *Executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
When event occurs, **Reactor** calls `wake()` and it push `Arc<Task>` to *Executor’s* **task queue**.<br>
