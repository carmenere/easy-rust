# Table of contents
- [Table of contents](#table-of-contents)
- [Dynamic linker](#dynamic-linker)
  - [rpath](#rpath)
  - [LD\_LIBRARY\_PATH](#ld_library_path)
  - [/etc/ld.so.cache](#etcldsocache)
- [`ldd`](#ldd)

<br>

# Dynamic linker
**Dynamic linker** finds and load the **shared objects** (**shared libraries**) needed by a binary, prepare the program to run, and then run it.<br>

[**man**](https://man7.org/linux/man-pages/man8/ld.so.8.html).<br>

The *dynamic linker* of the GNU C Library searches for shared libraries in the following locations in order:
- the colon-separated paths in the `DT_RPATH` attribute of `.dynamic` section of the binary if present and the `DT_RUNPATH` attribute does **not** exist;
- in directories listed in `LD_LIBRARY_PATH`;
- the colon-separated paths in the `DT_RUNPATH` attribute of `.dynamic` section of the binary if present;
- in `/etc/ld.so.cache`;
- in **default** directories
  - `/lib` and `/usr/lib` for **32–bit** application;
  - `/lib/64` and `/usr/lib/64` for **64–bit** applications;

At **run time**, if the *dynamic linker* finds a `DT_RUNPATH` attribute, it **ignores** the value of the `DT_RPATH` attribute.

<br>

## rpath
The **rpath** (aka **run paths**) stands for **run-time search paths** hard-coded in an **executable** file or **library**.<br>

It is possible to add additional search path in object file directly. To record a runpath in an executable or shared object:
`gcc hello.o -Wl,-rpath=. -o hello`<br>

It is possible to set a **run path** for **executable** file or **library**.<br>
The **run paths** are accessible through `DT_RPATH` and `DT_RUNPATH` attributes in the `.dynamic` section.<br>
The difference between the two attributes is **when** they are used during the search for dependencies.<br>

The `DT_RPATH` value is used **first**, before any other path. This is problematic since it does not allow the user to overwrite the value. Therefore `DT_RPATH` is **deprecated**.
The `DT_RUNPATH` attribute must be used instead.<br>

<br>

Specialized objects can be built with the `-z nodefaultlib` option to suppress any search in **default** directories. Use of this option implies that all the dependencies of an object can be located using its runpaths. Without this option, no matter how you augment the runtime linker's search path, the last search paths used are always the default locations.

<br>

## LD_LIBRARY_PATH
`LD_LIBRARY_PATH` is a list of directories in which to search for **shared libraries** at **execution** time.<br>

Example:
```bash
LD_LIBRARY_PATH=/root:${LD_LIBRARY_PATH}
export LD_LIBRARY_PATH
```

<br>

## /etc/ld.so.cache
The command `ldconfig -p` lists `/etc/ld.so.cache`.<br>
The command `ldconfig` rebuilds `/etc/ld.so.cache` using `/etc/ld.so.conf.d` and `/etc/ld.so.conf`:
```bash
rm /etc/ld.so.cache
ldconfig
```

Content of `/etc/ld.so.conf.d`:
```bash
ls -hal /etc/ld.so.conf.d
-rw-r--r--   1 root root   38 Jan 21  2024 fakeroot-x86_64-linux-gnu.conf
-rw-r--r--   1 root root   44 Aug  2  2022 libc.conf
-rw-r--r--   1 root root  100 Mar 30  2024 x86_64-linux-gnu.conf
```

<br>

```bash
cat /etc/ld.so.conf.d/fakeroot-x86_64-linux-gnu.conf 
/usr/lib/x86_64-linux-gnu/libfakeroot
```

<br>

```bash
cat /etc/ld.so.conf.d/libc.conf 
/usr/local/lib
```

<br>

```bash
cat /etc/ld.so.conf.d/x86_64-linux-gnu.conf
/usr/local/lib/x86_64-linux-gnu
/lib/x86_64-linux-gnu
/usr/lib/x86_64-linux-gnu
```

<br>

# `ldd`
`ldd` and `otool` prints all **shared libraries** required by binary.<br>

In MacOS: `otool -L <file>`.<br>
In Linux: `ldd <file>`.<br>

<br>

Each entry in output has 3 value:
•	**optional**: `libNAME.so.X` where `X` is a version of lib, `NAME` - name of lib;
•	**abs path** to library file;
•	**address** at which it is loaded ;

<br>

**Example**:
```bash
ldd /bin/cat
    linux-vdso.so.1 (0x00007ffd2d6d9000)
    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f6c5b1fb000)
    /lib64/ld-linux-x86-64.so.2 (0x00007f6c5b7f5000)
```

<br>

The **first enry** `linux-vdso.so.1` **doesn't** have corresponding file, it is the **vDSO** (virtual dynamic shared object).<br>
The **last enry** contain only **abs path** and **address**.<br>

<br>

