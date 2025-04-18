# Compiler pipeline
**Compiler pipeline**:<br>
`Source code` -> `AST` -> `HIR` -> `MIR` -> `LLVM IR` -> `Machine code`.

<br>

![HIR_MIR](/img/HIR-MIR.png)

<br>

**Transformations**:
- the **HIR** is constructed from **AST**;
- the **MIR** is constructed from **HIR**;
- the **LLVM IR** is constructed from **MIR**;

<br>

The **HIR** (**high-level IR**) is used for **type inference** and **type checking**.<br>
The **MIR** (**mid-level IR**) is used for **borrow checking** and **optimizations** use.<br>

Internally **MIR** is represented as **CFG**.
The MIR CFG consists of a set of basic blocks.
_Each_ **basic block** in _MIR_ has a **series of statements** and a **terminator** ((**goto**/**return** statement)

**Statements** fall into following categories:
- **Assignments** like `x = y`, the **RHS** of such an assignment is called an **rvalue**, the **LHS** of such an assignment is called an **lvalue**;
  - note, that there are no compound **rvalues**;
  - for example, the Rust expression `a = b + c + d;` would be compiled into two _MIR_ instructions, like `tmp0 = b + c;` and `a = tmp0 + d;`;
- `drop(lvalue)` **deallocates** an **lvalue**;
- `StorageLive(x)`;
- `StorageDead(x)` **deallocates the stack storage** for `x`;
