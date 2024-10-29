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
A fully working **async runtime** in Rust consists of 2 parts:
- **reactor** (aka **event loop**): it **tracks events** we are waiting for and **notifies** about **I/O events**, in other words it **dispatches events to an executor**;
- **executor** (aka **scheduler**): it **polls top-level futures** (aka **tasks**) that are ready, in other words it calls `poll()` method on the **tasks**;

<br>

The **executer** provides:
- special API called **spawner**: spawner produces **new tasks** and puts them into *executor's* **task queue**;
- the **runtime** that
  - iterates over its **task queue** and calls `poll()` on `Futures` until `Futures` return the `Ready` state;
  - or sleep if **task queue** is empty;

<br>

The **reactor** notifies the **executor** which task is ready to continue executing.<br>

The **reactor** can track following **I/O events**:
- IO events;
- IPC;
- timers;
- interrupts;

<br>

So:
- *executor* **schedules** `Futures` that are **ready to be polled**;
- *reactor* **tracks IO events** and **wakes** `Futures` that are **bound** to events when events happen;
- 
<br>

The **loop** that **pools tasks** in the `main` function takes the role of the **executor**.<br>
The **executor** calls `poll()` method on the **top level futures**, which in turn call the `poll()` method on its **child future**.<br>
When some **future** is polled, it polls all its **child future** until it reaches a **leaf future**.<br>
The **leaf future** represents something we are actually waiting on, other words, it polls an actual **event source** that is either **ready** or **not**.<br>
If **leaf future** returns `Pending`, it is propagated up the chain of calls immediately.<br>
The **future** will **not** return `Ready` **until** all its **child** futures have returned `Ready`.<br>

<br>

**Chain of futures**:
![Chain of futures](/img/chain_of_futures.png)

<br>

To avoid continuous polling of **top-level futures** in `Pending` state we must exclude them from scheduling until they become ready.<br>
We can reach this by using **mio**'s **registry** and **poll** abstractions. For example, *leaf future* register *top-level future*'s **id** in mio's registry.<br>
Every time `block_on` function wakes up it iterates over events and **polls** *top level futures* that are **ready**.<br>

<br>

**Prototype** of **executor-reactor**:
![Chain of futures](/img/prototype_of_executor_reactor.png)

<br>

But in above implementation *reactor* and *executor* are **tightly coupled** because both *executor* and *reactor* knows about `mio::Registry` and `mio::Poll`.<br>
We acheive a **loose coupling** between the *reactor* and *executor* we need an interface to signal the *executor* that it should wake up and poll futures when appropriate events have occurred.<br>

<br>


# `Future` life cycle
Every `Future` transits through different phases during its life cycle.<br>

### Spawning
**Spawning** is registering a **top-level** `Future` at the **executor**.<br>

### Polling
The **executor** fetches `Future` from its **task queue** and call `poll(cx)` method on it where `cx` is `Context`.<br>
`Context` is **wrapper** for `Waker` and just contains a reference to a `Waker`.<br>
The result of the `poll(cx)` method represents the the state of the `Future`.<br>

### Waiting
When the **executor** calls `poll()` on a `Future`, that `Future` will return either `Ready` or `Pending`:
- If `Future` returns `Ready(T)` then the `.await` will return `T` and the **executor** removes it from the **task queue**.
- If `Future` returns `Pending` then the **executor** removes it from the **task queue**, but **Reactor** will notify **executor** when particular `Future` will become ready to be polled again. This is where the **Waker API** comes in.

### Waking
The **reactor** stores a **copy** of the `Waker` that the **executor** passed to the future when it polled it.<br>
The **reactor** tracks events on I/O source.<br>
When the **reactor** gets a notification that an **event has happened** on one of the **tracked source**, it locates the `Waker` associated with that source and calls `Waker::wake` on it.<br>
This in turn puts `Future` that is bound to this event into *executor's* **task queue**.<br>

<br>

# Waker API
The **Waker API** connects *executor* and *Reactor*.<br>
Every time *executor* calls `poll(cx)` method it passes a `Context` to it. `Context` provides access to a `Waker`, i.e., it wraps `Waker`.<br>
The reason `poll()` takes `Context` instead `Waker` is to has ability add other things to `Context` in future.<br>

Requirements to `Waker` type:
- the `Waker` type cannot be Generic because it is need to be passed through arbitrary `Futures`;
- the `Waker` type must implement `.wake()` method;
- the `Waker` type must implement `Clone` trait;

<br>

`Futures` can be **nested** and `Waker` object is passed along chain of nested `Futures` until it reaches the **source of event** (**reactor**), then `Waker` is being registered in **Reactor**.<br>

If `Future` returns `Poll::Pending` then `Waker`, that was passed inside `Context`, is registered in **Reactor** and bound to **event id** (e.g. **file descriptor**).<br>
When event occurs **Reactor** calls `wake()`.

To poll `Futures` it is necessary to create a `Waker`. `Waker` is responsible for scheduling a task to be polled again once `wake()` is called.<br>
The easiest way to create a new `Waker` is by implementing the `ArcWake` trait and then using the `waker_ref()` or `into_waker()` functions to turn an `Arc<impl ArcWake>` into a `Waker`.<br>

<br>

## Ways to implement `wake()`
### Using task id
In this approach the `Waker` is **Task id** and the *executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
Also *executor* stores set of Tasks as `HashMap<Task_id, Task>`.<br>
When event occurs, **Reactor** calls `wake()` and it appends **Task** id to *executor’s* **task queue**.<br>

### Using reference counter
In this approach the `Waker` is `Arc<Task>` and the *executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
When event occurs, **Reactor** calls `wake()` and it push `Arc<Task>` to *executor’s* **task queue**.<br>
