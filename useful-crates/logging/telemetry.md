# Observability vs. Monitoring
`Inputs` -> `System` -> `Outputs`.<br>

**Observability** measures how easily can we infer **internal state** of system using only the obtained **telemetry** (metrics, logs, traces, profiling).<br>
**Observability** helps to determine the behavior of the entire system from the system's outputs.<br>
**Monitoring** is an **action**, but **observability** is a **property** of system.<br>
**Monitoring** shows the **fact** of problem, **observability** help to reveal the **root causes** of problem.<br>

| Monitoring                                 | Observability                 |
|:-------------------------------------------|:------------------------------|
| **Is** it broken?                          | **Why** it is broken?         |
| Helps to **react** on incident **quickly** | Helps to **prevent** incident |

_Observability_ **complements** _Monitoring_.<br>
_Observability_ provides means to **look deeper** into complex systems.<br>
_Observability_ allows to map/**correlate telemetry data** from various systems and to **enrich** them with context.<br>

To improve observability, software engineers use a wide range of techniques and tools to gather **telemetry data** and analyze it.<br>
Observability is foundational to site reliability engineering (**SRE**).<br>

_Observability_ relies on 3 main types of **telemetry data**: **metrics**, **logs** and **traces**. Those are often referred to as **pillars of observability**.<br>

<br>

# Telemetry
**Telemetry** is the **automated process** of **measuring** and **collecting**, **transmitting**, **processing** and **analyzing** data from **remote** systems. Once the data is **collected**, it needs to be transmitted to a **central** system that **store**, **process**, and **analyze** it.<br>
**Telemetry** provides **constant feedback**, ensuring you always have the data you need to make informed decisions.<br>
**Telemetry** helps you understand what’s happening within your system. Software developers and IT administrators use **telemetry** to remotely **monitor** the **health**, **security** and **performance** of applications in real time.<br>

**Telemetry data** (or just **telemetry**) refers to the information gathered from **remote** systems and **transmitted** to a **central** system for analysis.<br>
**Telemetry** includes different types of data, such as **logs**, **metrics**, and **traces**.<br>

A **metric** is a **scalar** value that represents some system **state** at a specific moment of time.<br>
**Event logging** is the act of keeping information about some **event** that occur in a system. A **log entry** (or just a **log**) is recorded for each such event.<br>
**Logs** typically include a **timestamp**, **severity level** and **message** about event. In the simplest case, _logs_ are written to a file, called a **log file**.<br>

<br>

# Instrumentation
**Instrumenting** a software application means **integrating logic** into its code that will **produce and collect telemetry** _at runtime_.<br>
Note that **instrumenting** a program can cause **performance penalty**, and may in some cases lead to **inaccurate results**.<br>

There are several **tools** and **techniques** to **instrument an application**. Specifically, there are **two** types of instrumentation:
- **source instrumentation**: involves modifying the **source code** of a program to add **instrumentation logic**;
- **binary instrumentation**: involves modifying the compiled **executable** to add **instrumentation logic**;

<br>

Software instrumentation comes with some challenges you must take into account. These include:
- **Performance overhead**: Instrumentation code can increase CPU and bandwidth usage, which can negatively impact the application's performance.
- **More complex code**: The logic required to implement instrumentation can make the codebase more complex and difficult to read and maintain.
- **Privacy concerns**: The data collected through instrumentation may include sensitive information, such as user behavior, preferences, and habits. This info should be handled with respect to privacy regulations and mustn't be exposed to unauthorized parties.
- **Stability issues**: Instrumenting an application can introduce new bugs or issues that weren't present in the original code.

<br>

# Tracing
**Tracing** in software engineering refers to the **process of capturing** information about the execution of a software program.<br>
It involves recording information about the program’s execution, such as **function calls**, **variable values**, or even the **entire call stack**. All such collected information is called **trace data** or just **trace**.<br>
_Tracing_ is achieved by **instrumenting** the source code.<br>

<br>

## Tracing vs. Logging
- **Event logging**:
    - Consumed primarily by **system administrators**;
    - Contains **high level** information;
    - Must **not** be too noisy;
- **Tracing**:
    - Consumed primarily by **developers**;
    - Logs **low level** information;
    - **Can** be noisy;

<br>

# Distributed traces
A **distributed trace** contains details that illustrate how the request moves through various services within a distributed system.<br>
A **trace** represents the **complete** journey of a request through a distributed system.<br>
A **trace** consists of multiple **spans**.<br>

**Spans** are fundamental building blocks of **distributed trace**.<br>
Each **span** represents a single **unit of work** during processing a user request.<br>

_Traces_ provide an **end-to-end overview** of the processing the whole request.<br>
_Spans_ provide **detailed information** about **individual** operations or steps.<br>

<br>

**Instrumenting** an application **with traces** means sending span information to a **tracing backend**.<br>
The tracing backend correlates the received spans to generate presentable traces.<br>
To be able to follow a request as it traverses multiple services, spans are labeled with ID that enable constructing a parent-child relationship between spans.<br>

<br>

A **parent span** (aka **root span**) encapsulates the **end-to-end latency** of an entire request.<br>
A **child span** is triggered by a _parent span_ and can be a **function call**, **DB call**, **call to another service**, etc.<br>

A **span attributes** are key-value pairs that can be used to provide additional context on a span about the specific operation it tracks.<br>

A **span context** uniquely identifies the request to which span belongs. A **span context** is propagated to all child spans.<br>
_Span context_ consists of three core components:
- **Trace ID**: all spans within the trace share the same **Trace ID**;
- **Span ID**: a unique identifier for each span within the trace;
- **Timestamps**: for example, duration of span;

<br>

# Profiling
**Profiling** is a form of dynamic program analysis that **measures** the **memory** or **time** complexity of a program, the **frequency** and **duration** of function calls and so on.<br>
**Profiling** is achieved by **instrumenting** either the **source code** or binary executable using a tool called a **profiler**.<br>
