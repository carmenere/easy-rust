# ``unimplemented!`` vs ``todo!``

- ``unimplemented!()`` macros indicates **unimplemented** code by panicking with a message of not implemented.<br>
- ``todo!()`` indicates **unfinished** code by panicking with a message of not **yet** implemented.<br>

The difference between ``unimplemented!`` and ``todo!`` is that ``todo!`` conveys an intent of **implementing the functionality later**, while ``unimplemented!`` makes no such claims. Its message is "**not implemented**".
