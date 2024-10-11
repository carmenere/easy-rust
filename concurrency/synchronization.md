# Synchronization
In general **shared resources** are
- data in RAM/HDD;
- peripheral devices.

<br>

**Concurrent access** to **shared resources** can arise in several problems:
- deadlock;
- race condition;
- resource starvation.

<br>

**Deadlock** is any situation in which each thread make no progress because it waits for another thread to release a lock.<br>
**Race condition** is any situation in which threads/processes simultaneously changes some data and the final result is depend on the sequence of operations.<br>

<br>

The main difficulty in designing concurrency applications is **coordinating** the **access** of different threads/processes to **shared resources**.<br>

<br>

# Сoncurrency control
The alghorithm or application is **not thread-safe** if it allows deadlock and/or race condition and/or resource starvation.<br>
**Concurrency control** *prevents* any problems of concurrent access to shared resources.<br>

There are 2 main types of **concurrency control** to achive **thread-safety**:
- **shared memory** communication;
- **message passing** communication.

<br>

Every type uses its own сoncurrency control methods for achiving **thread-safety**.

<br>

## Shared memory communication
**Shared memory** communication uses following synchronization primitives:
- Spinlock;
- Memory barrier;
- Mutex (requires syscall);
- Futex (in Linux);
- Semaphore;

<br>

The main difference between **mutex** and **spinlock** is that if a thread **fails** to acquire a **mutex**, it goes to **sleep**.<br>

<br>

### Spinlock
The main idea: 
1. Before accessing any shared resource, thread must check some flag (e.g., boolean var). 
2. If the flag is **reset**, then the thread sets the flag and **continues** executing the thread. 
3. If the flag is **set** (**locked**), the threads would **keep spinning** in a loop checking if the flag is set or not.
4. Once acquired, spinlocks will usually be held until they are explicitly released.

<br>

**Spinlocks** are time wasting primitives and they efficient if threads are likely to be blocked for only **short** periods. Spinlock can use **exponential backoff** delay before re-checking the flag.

<br>

**Spinlock** requires **hardware support** of **atomic operations**. But **Peterson's algorithm** alows to implement spinlock **without hardware support**. It is used in Linux - [link](https://elixir.bootlin.com/linux/v5.6.19/source/arch/arm/mach-tegra/sleep-tegra20.S#L126).<br>
**Atomicity** means bus is locked until atomic instruction is done, this guarantees other cores/CPUs will not access to RAM during atomic instruction is being executed.<br>

<br>

There 2 most commonly used atomic instructions in CPU: **TAS** и **CAS**.

<br>

#### TAS based spinlock
**Test-and-set** (**TAS**) is an atomic instruction that **writes** (set) `1` to some variable and returns its **previous** (**old**) value:
- if resource is **busy** (**locked**) the previous value is `1`;
- if resource is **free** the previous value is `0`;

<br>

The calling process/thread obtains the lock if the **old** value was `0`, otherwise the while-loop spins waiting to acquire the lock.

<br>

```c
void lock() {
  while (test_and_set(&isLocked, 1) == 0);
}
```

<br>

#### CAS based spinlock
**Compare-and-swap** (**CAS**) is an atomic instruction that compares the contents of some variable with a given value and, only if they are **the same**, modifies the contents of that memory location to a new given value.

<br>

```c
void lock() {
  // we expect current value is 0 and try to write 1
  while (!compare_and_swap(&isLocked, 0, 1))
}
```

<br>

## Message passing communication
**Message passing** communication uses various math model to achive thread-safety, for example:
- Communicating Sequential Processes (CSP);
- Calculus of Communicating Systems (CCS);
- Actor model
