# Dynamic linker
`ld.so`, `ld-linux.so` are dynamic linkers.<br>
The programs `ld.so` and `ld-linux.so*` find and load the **shared objects** (**shared libraries**) needed by a program, prepare the program to run, and then run it.<br>

[**man**](https://man7.org/linux/man-pages/man8/ld.so.8.html).<br>

The dynamic linker of the GNU C Library searches for shared libraries in the following locations in order:
- the (colon-separated) paths in the `DT_RPATH` attribute of `.dynamic` section of the binary if present and the `DT_RUNPATH` attribute does **not** exist;
- in directories listed in `LD_LIBRARY_PATH`;
- the (colon-separated) paths in the `DT_RUNPATH` attribute of `.dynamic` section of the binary if present;
- in `/etc/ld.so.cache`;
- in **default** directories `/lib` и `/usr/lib`;

At run time, if the dynamic linker finds a `DT_RUNPATH` attribute, it **ignores** the value of the `DT_RPATH` attribute.

<br>

## rpath
The **rpath** stands for the **run-time search path** hard-coded in an executable file or library.<br>

It is possible to set a **run path** for executable file or library.<br>
The **run paths** are accessible through `DT_RPATH` and `DT_RUNPATH` attributes in the `.dynamic` section.<br>
The difference between the two attributes is **when** they are used during the search for dependencies.<br>
The `DT_RPATH` value is used **first**, before any other path. This is problematic since it does not allow the user to overwrite the value. Therefore `DT_RPATH` is **deprecated**.
The `DT_RUNPATH` attribute must be used instead.<br>

<br>

## LD_LIBRARY_PATH
`LD_LIBRARY_PATH` is a list of directories in which to search for ELF libraries at execution time.<br>

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

# Shared libraries
In MacOS: `otool -L <file>`.<br>
In Linux: `ldd <file>`.<br>

`ldd` prints all **shared libraries** required by each program or shared object specified on the command line.<br>

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