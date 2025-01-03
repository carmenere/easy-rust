# Table of contents
- [Table of contents](#table-of-contents)
- [Executor/Reactor pattern](#executorreactor-pattern)
  - [Work stealing](#work-stealing)
  - [Chain of futures](#chain-of-futures)
- [Without Waker API](#without-waker-api)
- [Waker API](#waker-api)
  - [Ways to implement `wake()`](#ways-to-implement-wake)
    - [Using `task id`](#using-task-id)
    - [Using shared reference to the task itself](#using-shared-reference-to-the-task-itself)

<br>

# Executor/Reactor pattern
A fully working **async runtime** in Rust consists of 2 parts:
- **reactor** (aka **event loop**): it **tracks events** we are waiting for and **notifies** about **I/O events**, in other words it **dispatches events to an executor**;
- **executor** (aka **scheduler**): it **polls top-level futures** (aka **tasks**) that are **ready**, in other words it calls `poll()` method on the **tasks** that are **ready**;

<br>

The **executer** provides:
- **spawner** method;
- the **loop** that:
  - *iterates* over its **ready queue** and calls `poll()` on `Futures`;
    - if `Future` returns `Pending` executor puts it into its **pending queue**;
    - if `Future` returns `Ready<T>` executor puts its **result** `T` its **result queue** and remove future from scheduling;
  - *sends* **current thread** to **sleep** if the **ready queue** is **empty**;

<br>

**Spawning** is a registering **task** (aka **top-level future**) in the *executor's* **ready queue** to guaranty that **task** is polled **at least once**<br>

<br>

The **reactor tracks IO events** and notifies the **executor** which **task** is **ready** to continue executing.<br>
The **reactor** can track following **I/O events**:
- fd read/write/close events;
- IPC;
- timers;
- interrupts;

<br>

## Work stealing
There are 3 ways to implement executor:
1. **One** executor in single thread;
2. **Many** executors each in separate thread:
   1. When *all executors* share the **global** task queue and can **steal work** from each other;
   2. When *every executor* has its **thread local** task queue in this case executors **can't** *steal work* from each other (**no work-stealing**);

<br>

The design of **work-stealing executor** is **more complex**, because everything must be `Send` + `Sync`.<br>

<br>

## Chain of futures
The **loop** that **pools tasks** in the `main` function takes the role of the **executor**.<br>
The **executor** calls `poll()` method on the **top level futures**, which in turn call the `poll()` method on its **child future**.<br>
When some **future** is polled, it polls all its **child future** until it reaches a **leaf future**.<br>
The **leaf future** represents something we are actually waiting on, other words, it polls an actual **source of event** that is either **ready** or **not**.<br>
If **leaf future** returns `Pending`, it is propagated up the chain of calls immediately.<br>
The **future** will **not** return `Ready` **until** all its **child** futures have returned `Ready`.<br>

<br>

**Chain of futures**:
![Chain of futures](/img/chain_of_futures.png)

<br>

# Without Waker API
To avoid continuous polling of **top-level futures** in `Pending` state we must exclude them from scheduling until they become ready.<br>
We can reach this by using `mio::Registry` and `mio::Poll` abstractions. For example, *leaf future* register *top-level future*'s **id** in mio's registry.<br>
Every time `rt.run()` (see example `concurrency/examples/executor-reactor/no-waker`) wakes up then it iterates over events and **polls** *top level futures* that are **ready**.<br>

<br>

**Executor-reactor** without Waker API:
![Executor-reactor without Waker API](/img/prototype_of_executor_reactor.png)

<br>

But in above implementation *reactor* and *executor* are **tightly coupled** because both *executor* and *reactor* knows about `mio::Registry` and `mio::Poll`.<br>

<br>

# Waker API
We can acheive a **loose coupling** between the *reactor* and *executor* if we add additional layer of abstraction between them. This level of abstraction is called **Waker API**.<br>

Loosely coupled reactor and executor reactor and executor:<br>
![Loosely coupled reactor and executor](/img/loosely_coupled_runtime.png)

<br>

The **Waker API** connects *executor* and *reactor*:
- **executor** pass **waker** in top call of `poll()` and **waker** it spreads further **down** until **leaf future**;
- if **leaf future** returns `Poll::Pending` then it registers **waker**, that was passed inside `Context`, in a **reactor** and bound to **event id** (e.g. **file descriptor**);
- when appropriate events have occurred a **reactor** calls `waker.wake()` on appropriate **waker**;
- `waker.wake()` puts **task** (top-level future) from **pending queue** to **ready queue**;

<br>

**Executor-reactor** using **Waker API**:
![Executor-reactor using Waker API](/img/waker_api.png)

<br>

Every time *executor* calls `poll(cx)` method it passes instance of `Context`. `Context` provides access to a `Waker`, i.e., it wraps `Waker`.<br>
The reason `poll()` takes `Context` instead `Waker` is to has ability add other things to `Context` in future.<br>

<br>

Requirements to `Waker` type:
- the `Waker` type cannot be Generic because it is need to be passed through arbitrary `Futures`;
- the `Waker` type must implement `.wake()` method;
- the `Waker` type must implement `Clone` trait;

<br>

## Ways to implement `wake()`
To poll `Futures` it is necessary to create a `Waker`. `Waker` is responsible for scheduling a task to be polled again once `wake()` is called.<br>
The easiest way to create a new `Waker` is by implementing the `ArcWake` trait and then using the `waker_ref()` or `into_waker()` functions to turn an `Arc<impl ArcWake>` into a `Waker`.<br>

<br>

### Using `task id`
In this approach the `Waker` stores the **task id** and the reference to *executor’s* **ready queue** is `Vec<Arc<Task>>`.<br>
Also *executor* stores set of Tasks as `HashMap<Task_id, Task>`.<br>
When event occurs, **reactor** calls `wake()` and it appends **task id** to *executor’s* **ready queue**.<br>

<br>

### Using shared reference to the task itself
In this approach the `Waker` is `Arc<Task>` and the *executor’s* **ready queue** is `Vec<Arc<Task>>`.<br>
When event occurs, **reactor** calls `wake()` and it push `Arc<Task>` to *executor’s* **ready queue**.<br>
