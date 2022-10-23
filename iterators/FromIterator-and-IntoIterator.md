# Trait std::iter::FromIterator
pub trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

By implementing FromIterator for a type, you define how it will be created from an iterator. 

FromIterator::from_iter() is rarely called explicitly, and is instead used through Iterator::collect() method.

Trait std::iter::FromIterator
1.0.0 · source · [−]

pub trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

Conversion from an Iterator.

By implementing FromIterator for a type, you define how it will be created from an iterator. This is common for types which describe a collection of some kind.

If you want to create a collection from the contents of an iterator, the Iterator::collect() method is preferred. However, when you need to specify the container type, FromIterator::from_iter() can be more readable than using a turbofish (e.g. ::<Vec<_>>()). See the Iterator::collect() documentation for more examples of its use.

See also: IntoIterator.




Trait std::iter::IntoIterator
1.0.0 · source · [−]

pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator
    where
        <Self::IntoIter as Iterator>::Item == Self::Item;

    fn into_iter(self) -> Self::IntoIter;
}

Conversion into an Iterator.

By implementing IntoIterator for a type, you define how it will be converted to an iterator. This is common for types which describe a collection of some kind.

One benefit of implementing IntoIterator is that your type will work with Rust’s for loop syntax.

See also: FromIterator.