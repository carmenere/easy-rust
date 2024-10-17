# Alignment
**Alignment** refers to the arrangement of data in memory.<br>
From the software's point of view, **memory** is just **array of bytes**.<br>
However, CPU **does not** accesses memory **byte by byte**. Instead, it accesses memory by **units** and **unit** is **N bytes** long, **N** can be **1**, **2**, **4**, **8**.<br>
From the CPU's point of view, **memory** is just **array of units** and therefore CPU has its own **memory access granularity**.<br>

The **size** of *unit* **depends** on CPU **arch**:
- **16-bit** CPU reads memory by **words** (1 WORD = **2 bytes**);
- **32-bit** CPU reads memory by **doublewords** (1 DWORD = **4 bytes**);
- **64-bit** CPU reads memory by **quadwords** (1 QWORD = **8 bytes**);

<br>

## N-byte aligned address 
If the **address** is **not** *evenly divisible* by the number of **N** (**N** is the size of CPU's **memory access granularity**) it is called an **unaligned address**.<br>
If the **address** is *evenly divisible* by the number of **N** (**N** is the size of CPU's **memory access granularity**) it is called an **N-byte aligned address**.<br>

<br>

The **N-byte aligned address** means that the **data** this address points to is **aligned** on **N-byte boundary**.<br>

<br>

Examples:
|CPU's **memory access granularity**|Address (hex)|Result of division|Aligned or not|
|:----------------------------------|:------------|:-----------------|:-------------|
|N=**2** (WORD)|`0x4`|`0x4`/**2**=**2**|**Aligned**|
|N=**2** (WORD)|`0x3`|`0x3`/**2**=**1.5**|**Unaligned**|
|N=**4** (DWORD)|`0x18`|`0x18`/**4**=**6**|**Aligned**|

<br>

## Generic rule
**Generic rule**: if CPU's has **N** bytes **memory access granularity**, then **aligned address** must have *at least* $`log_2(N)`$ zeros at the end (least-significant zeros):
- N=**2**, so **aligned address** must have *at least* $`log_2(2) = 1`$ zero in the end;
- N=**4**, so **aligned address** must have *at least* $`log_2(4) = 2`$ zeros in the end;
- N=**8**, so **aligned address** must have *at least* $`log_2(8) = 3`$ zeros in the end;

<br>

## Drawbacks
If the microprocessor *supports* **unaligned access**, then *unaligned access* usually negatively impacts performance, as extra memory access is required to perform it.<br>
If the microprocessor **does not** *support* **unaligned access**, then *unaligned access* causes an **exception** on CPU.<br>
