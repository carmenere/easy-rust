# Cache coherence
**Cache coherence** ensures that all CPU or/and cores maintain a consistent view of memory.<br>

<br>

Hardware-based solutions for **cache coherence**:
- **snooping-based** coherence implementations;
- **directory-based** coherence implementations;

<br>

Every CPU/core has **cache controller** which tracks the status of each **cache line** in its cache. Each cache controller can send and receives messages from other cache controllers.<br>
In a **snooping system**, all the _cache controllers_ **monitor** (or **snoop**) the memory bus transactions and react accordingly, to maintain memory coherence.<br>

<br>

The most common **snooping protocols**:
1. **MESI protocol**.
2. **MESIF protocol**.
2. **MOESI protocol**.

<br>

Every _cache line_ in cache has **state**. The letters in the acronyms **MESI** represent **states** of **cache lines**:
- Modified (**M**)
- Exclusive (**E**)
- Shared (**S**)
- Invalid (**I**)

<br>

A **write** may only be performed freely if the **cache line** is in the **Modified** or **Exclusive** state.<br>
If it is in the **Shared** state, all other cached copies must be **invalidated** first. This is typically done by a broadcast operation known as **Request For Ownership** (**RFO**).<br>
A cache that holds a line in the **Modified** state must **snoop** (**intercept**) all attempted reads (from all the other caches in the system) of the corresponding main memory location and insert the data that it holds.<br>

<br>

When **different threads** access **the same cache line simultaneously** it causes to **performance penalty** because of _cache line_ **invalidation**.<br>