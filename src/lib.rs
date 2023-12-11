//! # MapOk
//!
//! This crate provides the [`MapOk`] trait that allows mapping [`Ok`] variants in an iterator to a different type.
//!
//! # Examples
//!
//! ```
//! use std::num::ParseIntError;
//! use std::str::FromStr;
//! use map_ok::MapOk;
//!
//! struct Person {
//!     age: u8,
//! }
//!
//! impl Person {
//!     fn new(age: u8) -> Self {
//!         Person { age }
//!     }
//! }
//!
//! impl FromStr for Person {
//!     type Err = ParseIntError;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         let age = u8::from_str(s)?;
//!         Ok(Person::new(age))
//!     }
//! }
//!
//! let input = vec!["10", "20", "x", "30"];
//! let mut iterator = input.iter()
//!     .map(|s| s.parse::<Person>())
//!     .map_ok(|p| p.age);
//!
//! assert_eq!(iterator.next(), Some(Ok(10)));
//! assert_eq!(iterator.next(), Some(Ok(20)));
//! assert!(iterator.next().unwrap().is_err());
//! assert_eq!(iterator.next(), Some(Ok(30)));
//! assert_eq!(iterator.next(), None);
//! ```

use std::marker::PhantomData;

/// Represents an iterator that maps the Ok values to another type using the given function.
///
/// This trait is implemented for iterators over `Result<T, E>`, allowing them to transform
/// the Ok values using a closure.
///
/// # Example
///
/// ```
/// use std::iter::Iterator;
/// use map_ok::MapOkIter;
///
/// pub trait MapOk<T, E>: Sized {
///     fn map_ok<U, F>(self, f: F) -> MapOkIter<Self, T, E, U, F>
///     where
///         F: Fn(T) -> U;
/// }
/// ```
///
/// # Implementations
///
/// Implementations of this trait must provide an implementation for the `map_ok` function, which receives
/// a closure `f` that takes an Ok value of type `T` and returns a value of type `U`. It returns a `MapOk`
/// iterator, which will apply the closure to each Ok value encountered during iteration.
pub trait MapOk<T, E, F, U>: Sized
where
    F: Fn(T) -> U,
{
    type Iter: Iterator<Item = Result<U, E>>;

    fn map_ok(self, f: F) -> Self::Iter;
}

impl<I, T, E, U, F> MapOk<T, E, F, U> for I
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(T) -> U,
{
    type Iter = MapOkIter<Self, T, E, U, F>;

    fn map_ok(self, f: F) -> Self::Iter {
        MapOkIter {
            iter: self,
            f,
            _phantom: PhantomData,
        }
    }
}

/// A special iterator adapter that applies a function to the elements of an underlying iterator,
/// similar to `Iterator::map`, but returns `Ok` variant of the result.
///
/// # Type arguments
/// * `I` - The iterator itself.
/// * `T` - The type of [`Ok`] variant of the iterated item.
/// * `E` - The type of the [`Err`] variant of the iterated item.
/// * `U` - The mapped type.
/// * `F` - A [`Fn`] that maps from `T` to `U`.
///
/// # Examples
///
/// ```
/// use std::num::ParseIntError;
/// use std::str::FromStr;
/// use map_ok::MapOk;
///
/// struct Person {
///     age: u8,
/// }
///
/// impl Person {
///     fn new(age: u8) -> Self {
///         Person { age }
///     }
/// }
///
/// impl FromStr for Person {
///     type Err = ParseIntError;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         let age = u8::from_str(s)?;
///         Ok(Person::new(age))
///     }
/// }
///
/// let input = vec!["10", "20", "x", "30"];
/// let mut iterator = input.iter()
///     .map(|s| s.parse::<Person>())
///     .map_ok(|p| p.age);
///
/// assert_eq!(iterator.next(), Some(Ok(10)));
/// assert_eq!(iterator.next(), Some(Ok(20)));
/// assert!(iterator.next().unwrap().is_err());
/// assert_eq!(iterator.next(), Some(Ok(30)));
/// assert_eq!(iterator.next(), None);
/// ```
pub struct MapOkIter<I, T, E, U, F> {
    iter: I,
    f: F,
    _phantom: PhantomData<MapFn<T, E, U>>,
}

/// Helper type to simplify type definition.
type MapFn<T, E, U> = fn(T, E) -> (U, Result<T, E>);

impl<I, T, E, U, F> Iterator for MapOkIter<I, T, E, U, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U,
{
    type Item = Result<U, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(value)) => Some(Ok((self.f)(value))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

/// Represents an iterator that boxes the Ok values.
///
/// This trait is implemented for iterators over `Result<T, E>`, allowing them to box
/// the Ok values using the `Box<T>` type.
///
/// # Implementations
///
/// Implementations of this trait must provide an implementation for the `box_ok` function, which
/// returns a `MapOk` iterator that boxes each Ok value encountered during iteration.
///
/// # Examples
///
/// ```
/// use std::num::ParseIntError;
/// use std::str::FromStr;
/// use map_ok::{BoxOk, MapOk};
///
/// struct Person {
///     age: u8,
/// }
///
/// impl Person {
///     fn new(age: u8) -> Self {
///         Person { age }
///     }
/// }
///
/// impl FromStr for Person {
///     type Err = ParseIntError;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         let age = u8::from_str(s)?;
///         Ok(Person::new(age))
///     }
/// }
///
/// let input = vec!["10", "20", "x", "30"];
/// let mut iterator = input.iter()
///     .map(|s| s.parse::<Person>())
///     .map_ok(|p| p.age)
///     .box_ok();
///
/// assert_eq!(iterator.next(), Some(Ok(Box::new(10))));
/// assert_eq!(iterator.next(), Some(Ok(Box::new(20))));
/// assert!(iterator.next().unwrap().is_err());
/// assert_eq!(iterator.next(), Some(Ok(Box::new(30))));
/// assert_eq!(iterator.next(), None);
/// ```
pub trait BoxOk<T, E>: Sized {
    type Iter: Iterator<Item = Result<Box<T>, E>>;

    fn box_ok(self) -> Self::Iter;
}

/// A function that boxes its argument.
pub type BoxingFn<T> = fn(T) -> Box<T>;

impl<I, T, E> BoxOk<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type Iter = MapOkIter<Self, T, E, Box<T>, BoxingFn<T>>;

    fn box_ok(self) -> Self::Iter {
        self.map_ok(Box::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::ParseIntError;
    use std::str::FromStr;

    struct Person {
        age: u8,
    }

    impl Person {
        fn new(age: u8) -> Self {
            Person { age }
        }
    }

    impl FromStr for Person {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let age = u8::from_str(s)?;
            Ok(Person::new(age))
        }
    }

    #[test]
    fn map_ok_works() {
        let input = vec!["10", "20", "x", "30"];
        let mut iterator = input.iter().map(|s| s.parse::<Person>()).map_ok(|p| p.age);

        assert_eq!(iterator.next(), Some(Ok(10)));
        assert_eq!(iterator.next(), Some(Ok(20)));
        assert!(iterator.next().unwrap().is_err());
        assert_eq!(iterator.next(), Some(Ok(30)));
        assert_eq!(iterator.next(), None);
    }
}
