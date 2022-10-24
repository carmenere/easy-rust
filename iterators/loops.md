# Loop syntax
``for ... in ...`` syntax is just a syntactic sugar for an ``IntoIterator::into_iter()`` invocation, followed by repeated calling of ``Iterator::next()``.<br>

Contexts:
- The call ``(T).into_iter()`` returns an ``Iterator`` over ``T``;
- The call ``(&T).into_iter()`` returns an ``Iterator`` over ``&T``;
- The call ``(&mut T).into_iter()`` returns an ``Iterator`` over ``&mut T``.

<br>

<table>
    <tr>
        <th>Context</th>
        <th></th>
        <th>Real call</th>
        <th></th>
        <th>Real loop</th>
    </tr>
<tr></tr>
<tr>
<td>

```Rust
for x in v {
  // body
}
```

</td>


<td>

**=>**

</td>
<td>

```Rust
let mut iter = (v).into_iter();
```

</td>
<td rowspan="5">

**=>**

</td>
<td rowspan="5">

```Rust
loop {
    match iter.next() {
        Some(x) => {
          // body
        },
        None => break,
    }
}
```

</td>
</tr>

<tr></tr>
<tr>
<td>

```Rust
for x in &v {
  // body
}
```

</td>

<td>

**=>**

</td>
<td>

```Rust
let mut iter = (&v).into_iter();
```

</td>

</tr>

<tr></tr>
<tr>
<td>

```Rust
for x in &mut v {
  // body
}
```

</td>


<td>

**=>**

</td>
<td>

```Rust
let mut iter = (&mut v).into_iter();
```

</td>

</tr>

</table>
