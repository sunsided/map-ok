# MapOk

This crate provides the `MapOk` trait that allows mapping `Ok` variants in an iterator to a different type. Instead
of matching `Result` variants in a `map` call, you call

```rust
fn example() {
    let input = ["10", "20", "x", "30"];
    let mut iterator = input.into_iter().map(u32::from_str).map_ok(|x| x * 100);
}
```

instead of the more verbose

```rust
fn example() {
    let input = ["10", "20", "x", "30"];
    let mut iterator = input.into_iter().map(u32::from_str).map(|x| match x {
        Ok(x) => Ok(x * 100),
        Err(e) => Err(e),
    });
}
```

## Examples

Below is a worked example with a bit more involved parsing:

```rust
use std::num::ParseIntError;
use std::str::FromStr;
use map_ok::MapOk;

/// A struct that represents a person.
struct Person {
    age: u8,
}

impl Person {
    /// Constructs a new `Person` instance.
    ///
    /// # Arguments
    ///
    /// * `age` - an unsigned 8-bit integer representing a person's age.
    fn new(age: u8) -> Self {
        Person { age }
    }
}

impl FromStr for Person {
    type Err = ParseIntError;

    /// Converts a string slice into a `Person` instance.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice that holds the person's age.
    ///
    /// # Returns
    ///
    /// A result that is either a `Person` or a `ParseIntError`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let age = u8::from_str(s)?;
        Ok(Person::new(age))
    }
}

/// In this example, the `map_ok` function is utilized to transform the `Ok` variant of a `Result`
/// by mapping the value of the `Person` age.
fn example() {
    let input = vec!["10", "20", "x", "30"];
    let mut iterator = input.into_iter()
        .map(Person::from_str)
        .map_ok(|p| p.age);

    assert_eq!(iterator.next(), Some(Ok(10)));
    assert_eq!(iterator.next(), Some(Ok(20)));
    assert!(iterator.next().unwrap().is_err());
    assert_eq!(iterator.next(), Some(Ok(30)));
    assert_eq!(iterator.next(), None);
}
```
