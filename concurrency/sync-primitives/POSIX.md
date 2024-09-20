# POSIX Threads
**POSIX Threads extensions** (aka **pthreads**) specifies functions for **thread management** and **synchronization primitives**.<br>

<br>

## Functions for thread management
- `pthread_create()` for **spawning** threads;
- `pthread_join()` for **joining** threads;
- `pthread_exit()`;

<br>

## Synchronization primitives
- **mutex**: `pthread_mutex_t`;
- **rwlock**: `pthread_rwlock_t`;
- **cond var**: `pthread_cond_t`;
- **spinlock**: `pthread_spinlock_t`;
- **barrier**: `pthread_barrier_t`;

<br>

### `pthread_mutex_t` functions
- **init**: `pthread_mutex_init()`;
- **destroy**: `pthread_mutex_destroy()`;
- **lock**:
  - `pthread_mutex_lock()`;
  - `pthread_mutex_trylock()`;
- **unlock**: `pthread_mutex_unlock()`;

<br>

### `pthread_rwlock_t` functions
- **init**: `pthread_rwlock_init()`;
- **destroy**: `pthread_rwlock_destroy()`;
- **lock**:
  - lock for **readers**: `pthread_rwlock_rdlock()` ;
  - lock for **writers**: `pthread_rwlock_wrlock()`;
- **unlock**: `pthread_rwlock_unlock()` for unlocking a lock of either kind;

<br>

### `pthread_cond_t` functions
- **init**: `pthread_cond_init()`;
- **destroy**: `pthread_cond_destroy()`;
- **wait**:
  - `pthread_cond_wait()`;
  - `pthread_cond_timedwait()`;
- **notify**:
  - `pthread_cond_signal()`;
  - `pthread_cond_broadcast()`;

<br>

# Linux
On Linux systems, the *pthread synchronization primitives* are **all** implemented using the `futex` **syscall**.<br>
The two main operations are `FUTEX_EAIT` and `FUTEX_WAKE`. The **wait** operation puts thread to **sleep**, and a **wake** operation on the the same atomic variable **wakes** the thread.<br>