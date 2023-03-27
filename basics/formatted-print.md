# Formatted print
There are series of macros defined in ``std::fmt`` for printing:
|Macros|Description|
|:-----|:----------|
|``print!``|Writes text to the **standard output**: ``io::stdout``.|
|``println!``|Same as ``print!`` but **appends newline** ``\n``.|
|``eprint!``|Writes text to the **standard error**: ``io::stderr``.|
|``eprintln!``|Same as ``eprint!`` but **appends newline** ``\n``.|

<br>

# Variants to pass arguments to ``println!``
<table>
<tr>
<td> <b>Variant</b> </td> <td> <b>Example</b> </td>
</tr>
<tr></tr>
<tr>
<td> <b>Positional args</b> </td> 
<td>

```Rust
println!("{0} and {1}.", a, b);
```

</td>
</tr>
<tr></tr>
<tr>
<td><b>Named args</b></td>
<td>

```Rust
println!("{a} and {b}.", a="x", b="y");
```

</td>
</tr>
<tr></tr>
<tr>
<td><b>Expressions</b></td>
<td>

```Rust
println!("a + b = {}.", 50 + 40);
```

</td>
</tr>
<tr></tr>
<tr>
<td><b>Explicit values</b></td>
<td>

```Rust
println!("{} and {}.", "x", 22);
```

</td>
</tr>
<tr></tr>
<tr>
<td><b>Formatting traits</b>.

<br>
Mapping formatting sign to formatting trait:

<br>
<ul>
<li><i>nothing</i> ⇒ <code>Display</code> trait</li>
<li><code>:?</code> ⇒ <code>Debug</code> trait</li>
<li><code>:x?</code> ⇒ <code>Debug</code> trait with lower-case hexadecimal integers</li>
<li><code>:X?</code> ⇒ <code>Debug</code> trait with upper-case hexadecimal integers</li>
<li><code>:o</code> ⇒ <code>Octal</code> trait</li>
<li><code>:x</code> ⇒ <code>LowerHex</code> trait</li>
<li><code>:X</code> ⇒ <code>UpperHex</code> trait</li>
<li><code>:p</code> ⇒ <code>Pointer</code> trait</li>
<li><code>:b</code> ⇒ <code>Binary</code> trait</li>
<li><code>:e</code> ⇒ <code>LowerExp</code> trait</li>
<li><code>:E</code> ⇒ <code>UpperExp</code> trait</li>
</ul>
</td>
<td>

```Rust
println!("binary: {:b}.", 12);
```

```Rust
println!("debug vector: {:?}.", vec![1, 2]);
```

```Rust
println!("binary: {:x}.", 12);
```

</td>
</tr>
</table>


