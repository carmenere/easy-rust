# Table of contents
- [Table of contents](#table-of-contents)
- [In a nutshell](#in-a-nutshell)
- [Adapters](#adapters)
- [Declarations](#declarations)
  - [`map()`](#map)
      - [Example](#example)
  - [`rev()`](#rev)
  - [`filter()`](#filter)
  - [`flatten()`](#flatten)
      - [Example](#example-1)
  - [`filter_map()`](#filter_map)
  - [`flat_map()`](#flat_map)
      - [Example](#example-2)
  - [`fuse()`](#fuse)
  - [`chain()`](#chain)
      - [Example](#example-3)
  - [`enumerate()`](#enumerate)
  - [`zip()`](#zip)
      - [Example for `zip()`](#example-for-zip)
      - [Example for `zip_longest()`:](#example-for-zip_longest)
  - [`by_ref()`](#by_ref)
  - [`cloned()`](#cloned)
  - [`copied()`](#copied)
  - [`take()`](#take)
  - [`take_while()`](#take_while)
  - [`skip()`](#skip)
  - [`skip_while()`](#skip_while)
  - [`fold()`](#fold)

<br>

# In a nutshell
`Iterator` trait provides **adapter methods**, or simply **adapters**.<br>

Calling **adapter** on an **iterator** returns a **new iterator** that **yields its own items** from the first iterator.<br>

Every **adapter** takes **iterator** **implicitly**, because of `self` argument.<br>

In a **chain of adapters**, the only way to get a result is to call `next()` or `collect()` on the **final iterator**.<br>

<br>

# Adapters
|Adapter|Description|
|:----|:------------|
|[**all**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)|Tests if **predicate** `p` is applied to **every** element returns `true`. On empty returns `true`.|
|[**any**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)|Tests if **predicate** `p` is applied to element returns `true` at least for 1 element. On empty returns `false`.|
|[**by_ref**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref)|Returns a **mutable reference** to the **iterator**.<br>Calling **adapter** on an **iterator** *transfer ownership* of the **underlying iterator** (because of `self` argument).<br>When you call **adapter** on a **mutable reference** to an **iterator**, this **adapter** takes ownership of the reference, not the **iterator** itself.<br>It is just a **borrow** that ends when the adapter goes out of scope and **original collection is valid**.<br>|
|[**chain(o)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)|**Connects 2 independent iterators** together, in a chain.<br>Argument `o` must implement `IntoIterator<Item = Self::Item>`.|
|[**cloned**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)|Takes an **iterator** that produces **references** and returns an **iterator** that produces **values** cloned from those references.<br>Semantic of `some_iter.cloned()` is equivalent to: `some_iter.map(\|item\| item.clone())`.|
|[**collect**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)||
|[**copied**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied)|Takes an **iterator** that produces **references** and returns an **iterator** that produces **values** copied from dereferenced values.<br>Semantic of `some_iter.copied()` is equivalent to: `some_iter.map(\|item\| *item)`.|
|[**count**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)||
|[**cycle**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle)|Instead of stopping at `None`, the iterator will starting again from the beginig.|
|[**filter_map**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)|Semantic of `filter_map(f)` is equivalent to: `map(f).filter()`.<br>It **yields** only items of type `B` for which closure `f` returns `Some(B)`.|
|[**filter(p)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)|Applies **predicate** `p` to every element in collection and **yields** only the elements for which the **predicate** returns `true`.|
|[**find**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)||
|[**fold(init, f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)|Recursively applies **closure** `f` to every element in collection and returns the **result value** of **accumulator**: `acc`. This operation is sometimes called **reduce**.<br>**Closure** `f` has 2 args: `acc` and element, result of closure is passed to `acc` in **next** *iteration*.<br>Argument `init` is the value that assigned to `acc` **before** *first call* of `f`.<br>Note, `init`, `acc` and **result value** must be of the **same type**.|
|[**for_each**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)||
|[**last**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last)||
|[**map(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)|Applies **closure** `f` to every element in collection. **Closure** `f` takes an `Self::Item` as input and returns the result of another type `B`.|
|[**max**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max)||
|[**min**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min)||
|[**nth**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)||
|[**partition**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition)||
|[**product**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)||
|[**reduce(f)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce)|Similar to `fold()`, but instead explicit `init` value uses **first element** of collection as `init`.|
|[**rev**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev)|**Reverses** an iterator’s direction.|
|[**scan**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan)||
|[**skip**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip)|Returns an **iterator** that **skips** the first `n` elements.<br>|
|[**step_by**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by)||
|[**sum**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)||
|[**take**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take)|Returns an **iterator** that **yields** the first `n` elements, or fewer if the underlying iterator ends sooner.|
|[**unzip**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip)||
|[**zip(i)**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)|Takes **2 iterators**: `self` and `i` and **returns 1 iterator** *of 2-tuples* which yields tuple `(Self::Item, other::Item)`.<br>The `zip()` method uses **shortest semantics**: the **result** will have **length** of the **shortest iterable**.<br>For **longest semantics** use the `zip_longest()` in `itertools` crate.|
|[**flatten**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)|Concatenates **iterator of iterables** into a **single collection of elements** and returns an **iterator** `Flatten` **over** the concatenated single collection of elements.<br>The `flatten()` method requires `Self::Item` to be **iterable**. It yields `Self::Item::Item`, where `Self::Item` is some collection of elements of type `Item`.|
|[**fuse**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse)|What returns `Iterator` after it has already returned `None`?.<br>Most **iterators** just return `None` **again**, but **not all**.<br>The `fuse()` takes any `Iterator` and produces new `Iterator` that will definitely continue to return `None` once it has done so the first time.|
|[**enumerate**](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)|**Yields** tuple `(usize, Self::Item)`, where element of `usize` type contains the **index** of the **value** of `Self::Item` type.|