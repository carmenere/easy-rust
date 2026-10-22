# Application runtime vs Runtime environment
**Application runtime** is the **period of time** during which a program or application is actually running on a system.<br>

<br>

A **platform** is a **hardware** and **operating system** (**OS**) that acts as a foundation upon which other applications are executed.
A **runtime environment** (**RTE**) is the environment in which a program or application is executed, i.e. the **RTE** is a piece of software that is designed to run other software.<br>
So, an **OS** is the **universal RTE** *for any other RTE*, which acts as the middleware between the application and the OS.<br>

<br>

We can look at software as a series of layers that sit on top of the *hardware*. Each layer provides services that will be used and required by the layer above it.<br>
For example, let's say your application is written in *JavaScript*. *JavaScript* is a high-level language whose high-level *runtime environment* is *Node.js*.
*Node* comes with fancy features like a callback queue, and event loop, and a thread pool. As a *runtime environment*, *Node* itself has its own *runtime environment*.
If you download the *Node* binary for Linux, you'll find that it's just another ELF executable to be run by the OS.<br>

<br>

So, *JavaScript*'s *runtime environment* is **Node**, and *Node*'s *runtime environment* is the **OS**.<br>

<br>

So, **OS** and **Programming language runtime** are both **RTE**, i.e. they are both provide some environment where app will be executed.<br>
