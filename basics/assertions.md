# Assertions
An **assertion** is a statement that enables you to test your assumptions about your program.

If assertion is `false`, the program **crashes**.

<table>
    <tr>
        <td><b>Assertion macros</b></td>
        <td><b>Description</b></td>
    <tr>
<tr>
<td>

```Rust
assert!(expr)
```

</td>
<td>If <code>expr</code> is <b>false</b> then <code>panic!</code> is called.</td>
<tr></tr>
<tr>
<td>

```Rust
assert_eq!(left, right)
```

</td>
<td>If <code>left</code> is <b>not equal</b> <code>right</code> then <code>panic!</code> is called.</td>
</tr>
</table>
