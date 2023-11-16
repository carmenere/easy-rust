# Iterators
Consider example:
```Rust
for item in collection {
    ...
}
```

In this example, after `for` loop *collection* `collection` is become **invalid**.<br>

Access to **collections** in loops uses `move semantics` by default.

<br>

To make the `collection` **reusable after loop** use `immutable reference` to access to the `collection`:
```Rust
for item in &collection {
    ...
}
```

<br>

To **modify item** *during* the loop use `mutable reference` to access to the `collection`:
```Rust
for item in &mut collection {
    ...
}
```

Iterator syntax variants:
<table>
<tr>
<td><b>Shorthand</b></td>
<td><b>Equivalent</b></td>
<tr>
<tr></tr>
<tr>
<td>

```Rust
for item in collection
```
</td>
<td>

```Rust
for item in IntoIterator::into_iter(collection)
```
</td>
</tr>
<tr></tr>
<tr>
<td>
        
```Rust
for item in &collection
```
</td>
        <td>

```Rust
for item in collection.iter()
```
</td>
</tr>
<tr></tr>
<tr>
<td>

```Rust
for item in &mut collection
```
</td>
<td>

```Rust
for item in collection.iter_mut()
```
</td>
    </tr>
</table>
