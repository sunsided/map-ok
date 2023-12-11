use crate::{MapOk, MapOkIter};

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
        let input = ["10", "20", "x", "30"];
        let mut iterator = input
            .iter()
            .map(|s| s.parse::<Person>())
            .map_ok(|p| p.age)
            .box_ok();

        assert_eq!(iterator.next(), Some(Ok(Box::new(10))));
        assert_eq!(iterator.next(), Some(Ok(Box::new(20))));
        assert!(iterator.next().unwrap().is_err());
        assert_eq!(iterator.next(), Some(Ok(Box::new(30))));
        assert_eq!(iterator.next(), None);
    }
}
